<script>
    import { invoke } from "@tauri-apps/api/tauri"
    async function oscilate(){
      await invoke('waveTable')
    }
  </script>