const COMMANDS: &[&str] = &["ping", "invoke_erl_rpc"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
