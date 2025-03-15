use erl_rpc::RpcClientHandle;
use tauri::Manager;
use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::ErlRpcExt;
use crate::Result;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.erl_rpc().ping(payload)
}

#[command]
pub(crate) async fn invoke_erl_rpc<R: Runtime>(
    app: AppHandle<R>,
    payload: RpcRequest,
) -> Result<RpcResponse> {
    let rpc_handle = app.state::<RpcClientHandle>();
    app.erl_rpc()
        .invoke_erl_rpc(
            &payload.module,
            &payload.function,
            payload.value,
            rpc_handle.inner().clone(),
        )
        .await
}
