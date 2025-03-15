import { invoke } from '@tauri-apps/api/core'

export async function ping(value: string): Promise<string | null> {
  return await invoke<{value?: string}>('plugin:erl-rpc|ping', {
    payload: {
      value,
    },
  }).then((r: { value?: any; }) => (r.value ? r.value : null));
}

export async function invoke_erl_rpc(
  mod: string,
  func: string,
  payload: any | null
): Promise<any | null> {
  return await invoke<{value?: any}>('plugin:erl-rpc|invoke_erl_rpc', {
    payload: {
      module: mod,
      function: func,
      payload: payload,
    }
  }).then((r: { value?: any; }) => (r.value ? r.value : null));
}