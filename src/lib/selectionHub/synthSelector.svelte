<script>
    import { invoke } from "@tauri-apps/api/tauri"
    async function oscilate(){
      let test = await invoke('waveTable',{sampleRate: 44100,capacity: 64,frequency: 440.0})
    }
</script>

<div class="row">
    <div class="column">
        <div class="spacing"></div>
        <button class="type" on:click={oscilate}>Wave Table Synthesis</button>
        <div class="spacing"></div>
        <button class="type">Additive Synthesis</button>
        <div class="spacing"></div>
        <button class="type">Subtractive Synthesis</button>
        <div class="spacing"></div>
        <button class="type">FM Synthesis</button>
        <div class="spacing"></div>
    </div>
    <div class="column">
        <div class="spacing"></div>
        <button class="type">Sample-based Synthesis</button>
        <div class="spacing"></div>
        <button class="type">Vector Synthesis</button>
        <div class="spacing"></div>
        <button class="type">Granular Synthesis</button>
        <div class="spacing"></div>
    </div>
</div>
<style>
    .row{
        position: relative;
        display: flex;
        flex-direction: row;
        left: 80px;
    }
    .column{
        float: left;
        display: flex;
        justify-content: center;
        flex-direction: column;
        width: 50%;
        height: 100%;
        display: table;
    }
    .type{
        padding: 15px;
        width: 90%;
        height: 215px;
        border-radius: 1%;
    }
    .spacing{
        height: 10px;
    }
</style>
