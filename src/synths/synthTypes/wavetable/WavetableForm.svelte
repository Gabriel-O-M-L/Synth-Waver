<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import KEY from "../../../data/KEYS" 
    import {onMount,onDestroy} from "svelte"
    import Dropdown from "../../../components/general/Dropdown/Dropdown.svelte"

    let capacity = 64
    let sampleRate = 44100
    
    function callSynth(frequency){
        invoke("waveTable",{sampleRate,capacity,frequency})
    }
    
    const clickedKeys = {}
    function handler(event){
    if(event.type === "keydown"){
        clickedKeys[event.code] = true
        switch(event.code){
            case KEY.KeyA:{
                callSynth(55)       
            }
            case KEY.KeyB:{
                callSynth(61.173)
            }
            case KEY.KeyC:{
                callSynth(32.703)
            }
        }
    }else{
        clickedKeys[event.code] = false
    }
    }
    onMount(() => {
        document.addEventListener("keydown", handler)
        document.addEventListener("keyup", handler)
    })
    onDestroy(() => {
        document.removeEventListener("keydown", handler)
        document.removeEventListener("keyup", handler)
    });

</script>

<div style="position:absolute;left: 80px;">
    <Dropdown>
        <button slot="button" class="dropdown">
            Select Wave Form
        </button>
        <button>Square Wave</button>
        Square wave
        <!--    Resto do conteudo é sem slot (pode ser qualquer coisa)-->
        <button on:click={_ => console.log("OI")}>
            Sine
        </button>
    </Dropdown>
    <!-- <select class="dropdown">Choose Wave form
    <option class="options" on:click={callSynth}>Sine Wave</option>
    <option class="options">Square Wave</option>
    </select> -->
</div>
<style>
    body{
        background-color: blue;
    }
    .dropdown{
        justify-self: center;
        color: white;
        background-color: #535050;
    }
    .options{
        color: white;
        background-color: #535050;
    }
    .dropdown {
        display: flex;
        align-items: center;
        gap: 4px;
        border: none;
    }
</style>