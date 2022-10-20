<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    let sampleRate = 44100
    let capacity = 64
    let noteSystem = 1
    onMount(()=>
    {invoke("inputHandler",{noteSystem,sampleRate,capacity})
    })

    
</script>
<div>
    display test
</div>