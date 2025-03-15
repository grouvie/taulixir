use std::sync::{Arc, Mutex};
use tauri::Manager;
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::{process::CommandChild, ShellExt};

struct SidecarHandle {
    child: CommandChild,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_erl_rpc::init())
        .setup(move |app| {
            // Acquire the shell, create the sidecar command.
            let sidecar_command = app
                .shell()
                .sidecar("midway")
                .expect("Creating sidecar Command failed")
                .args(["start"]);

            // Spawn the sidecar
            let (mut rx, child) = sidecar_command.spawn().expect("Failed to spawn sidecar");

            // Save the child process handle into the state for later kill.
            let sidecar_state = Arc::new(Mutex::new(Some(SidecarHandle { child })));

            // Spawn a new async task to log output from the sidecar.
            tauri::async_runtime::spawn(async move {
                while let Some(event) = rx.recv().await {
                    match event {
                        CommandEvent::Stdout(line_bytes) => {
                            let line = String::from_utf8_lossy(&line_bytes);
                            println!("Sidecar stdout: {}", line);
                        }
                        CommandEvent::Stderr(line_bytes) => {
                            let line = String::from_utf8_lossy(&line_bytes);
                            eprintln!("Sidecar stderr: {}", line);
                        }
                        CommandEvent::Terminated(status) => {
                            println!("Sidecar terminated with: {:?}", status);
                        }
                        CommandEvent::Error(error) => {
                            println!("Sidecar error: {:?}", error);
                        }
                        _ => {}
                    }
                }
            });

            // Register an event handler for when the main window is closed.
            app.get_webview_window("main")
                .unwrap()
                .on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { .. } = event {
                        println!("Main window is closing, attempting to kill the sidecar");

                        let maybe_sidecar = sidecar_state
                            .lock()
                            .expect("Locking sidecar_state failed")
                            .take();
                        if let Some(handle) = maybe_sidecar {
                            {
                                // Attempt to kill the child process.
                                if let Err(error) = handle.child.kill() {
                                    eprintln!("{error}");
                                };
                            }
                        }
                    }
                });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
