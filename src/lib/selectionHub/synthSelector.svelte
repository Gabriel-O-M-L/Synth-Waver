<script>
    import { invoke } from "@tauri-apps/api/tauri"
    let sampleRate = 44100
    let capacity = 64
    let frequency = 349.228
    function waveTableOscilate(){
        invoke('waveTable',{sampleRate,capacity,frequency})
    }
    async function additiveOscilate(){
      await invoke('additive',{sampleRate,capacity,frequency})
    }
    async function subtractiveOscilate(){
      await invoke('subtractive',{sampleRate,capacity,frequency})
    }
    async function fmOscilate(){
      let test = await invoke('fm',{sampleRate,capacity,frequency})
    }
    async function sampleBasedOscilate(){
      let test = await invoke('sampleBased',{sampleRate,capacity,frequency})
    }
    async function vectorOscilate(){
      let test = await invoke('vector',{sampleRate,capacity,frequency})
    }
    async function granularOscilate(){
      await invoke('granular',{sampleRate,capacity,frequency})
    }
</script>

<div class="row">
    <div class="column">
        <div class="spacing"></div>
        <button class="type" on:click={waveTableOscilate}>Wave Table Synthesis</button>
        <div class="spacing"></div>
        <button class="type" on:click={additiveOscilate}>Additive Synthesis</button>
        <div class="spacing"></div>
        <button class="type" on:click={subtractiveOscilate}>Subtractive Synthesis</button>
        <div class="spacing"></div>
        <button class="type" on:click={fmOscilate}>FM Synthesis</button>
        <div class="spacing"></div>
    </div>
    <div class="column">
        <div class="spacing"></div>
        <button class="type" on:click={sampleBasedOscilate}>Sample-based Synthesis</button>
        <div class="spacing"></div>
        <button class="type" on:click={vectorOscilate}>Vector Synthesis</button>
        <div class="spacing"></div>
        <button class="type" on:click={granularOscilate}>Granular Synthesis</button>
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
        left: 80px;
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
