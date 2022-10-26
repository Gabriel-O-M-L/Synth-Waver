<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import KEY from "../../../data/KEYS" 
    import {onMount,onDestroy} from "svelte"
    import Dropdown from "../../../components/general/Dropdown/Dropdown.svelte"

    let capacity = 64
    let sampleRate = 44100
    let frequency = 440.0
    let waveForm = 2
    function callSynth(){
        invoke("waveTable",{sampleRate,capacity,frequency,waveForm})
    }
    
    const clickedKeys = {}
    function handler(event){
    if(event.type === "keydown"){
        clickedKeys[event.code] = true
        switch(event.code){
            case KEY.KeyA:{
                while(clickedKeys[event.code] == true){
                    callSynth()       
                }
            }
            case KEY.KeyS:{
                callSynth()
            }
            case KEY.KeyD:{
                callSynth()
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