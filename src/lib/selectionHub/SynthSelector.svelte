<script>
    import { invoke } from "@tauri-apps/api/tauri"
    // import {wavecomponent} from "../synthOptions/wavetable.svelte"
    let sampleRate = 44100
    let capacity = 64
    let noteSystem = 1
    let frequency = 440.0
    async function waveTableOscilate(){
      await invoke('waveTable',{sampleRate,capacity,frequency})
    }
    async function additiveOscilate(){
      await invoke('additive',{sampleRate,capacity})
    }
    async function subtractiveOscilate(){
      await invoke('subtractive',{sampleRate,capacity})
    }
    async function fmOscilate(){
      await invoke('fm',{sampleRate,capacity})
    }
    async function sampleBasedOscilate(){
      await invoke('sampleBased',{sampleRate,capacity})
    }
    async function vectorOscilate(){
      await invoke('vector',{sampleRate,capacity})
    }
    async function granularOscilate(){
      await invoke('granular',{sampleRate,capacity})
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
    </div>
    <div style="height: 100%; width: 5%"></div>
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
<button class="centerButton" on:click={fmOscilate}>FM Synthesis</button>
<style>
    .row{
        position: relative;
        display: flex;
        width: 95%;
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
        width: 100%;
        height: 215px;
        border-radius: 1%;
    }
    .centerButton{
        padding: 15px;
        width: 857px;
        height: 215px;
        position: relative;
        left: 550px;
        border-radius: 1%;
    }
    .spacing{
        height: 10px;
    }
</style>
