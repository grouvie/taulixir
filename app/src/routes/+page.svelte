<script lang="ts">
  import { invoke_erl_rpc } from "tauri-plugin-erl-rpc";

  const BASE_URL = "http://localhost:8080";

  let counter: number | null = 0;
  let error: string | null = null;

  function getErrorMessage(err: unknown): string {
    if (err instanceof Error) return err.message;
    return String(err);
  }

  async function loadCounter() {
    try {
      const res = await invoke_erl_rpc("Elixir.Midway.Counter", "get", null);

      counter = Number(res);
      error = null;
    } catch (err: unknown) {
      error = getErrorMessage(err);
    }
  }

  async function increaseCounter() {
    try {
      const res = await invoke_erl_rpc("Elixir.Midway.Counter", "inc", null);

      counter = Number(res);
      error = null;
    } catch (err: unknown) {
      error = getErrorMessage(err);
    }
  }

  async function decreaseCounter() {
    try {
      const res = await invoke_erl_rpc("Elixir.Midway.Counter", "dec", null);

      counter = Number(res);
      error = null;
    } catch (err: unknown) {
      error = getErrorMessage(err);
    }
  }
</script>

<main class="container">
  <h1>Counter App</h1>

  {#if error}
    <p style="color: red;">{error}</p>
  {/if}

  <p>Counter: {counter !== null ? counter : "Loading..."}</p>

  <div class="buttons">
    <button on:click={decreaseCounter}>Decrease</button>
    <button on:click={increaseCounter}>Increase</button>
    <button on:click={loadCounter}>Refresh</button>
  </div>
</main>

<style>
  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  h1 {
    margin-bottom: 1rem;
  }

  .buttons {
    margin-top: 1rem;
    display: flex;
    justify-content: center;
    gap: 1rem;
  }

  button {
    padding: 0.6em 1.2em;
    font-size: 1em;
    border: none;
    border-radius: 8px;
    background-color: #0f0f0f;
    color: #ffffff;
    cursor: pointer;
    transition: background-color 0.25s;
  }

  button:hover {
    background-color: #333333;
  }
</style>
