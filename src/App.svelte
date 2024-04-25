<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let number = 0;
  onMount(() => {
    // Spawn a bunch of IPC requests to crash the app
    setInterval(async () => {
      // Each of these tasks takes 20 seconds to complete.
      //
      // If main.ts is modified to cause the browser to reload, a stale task
      // that was started before the reload and finishes AFTER the reload
      // will cause the app to crash.
      const result = await invoke("long_task", { number: number++ });
      console.log(result);
    }, 1000);
  });
</script>

<main class="container">
  <h1>Crash in IPC response</h1>

  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>

  <p>Try modifying main.ts and saving &ndash; the app will crash.</p>
  <p>Sent {number} messages</p>
</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
