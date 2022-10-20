<script>
    import Types from './SynthSelector.svelte'
    let items = [
        {
            label: "New Synth",
            value: 1,
            component: Types
        },
        {
            label: "Load Synth",
            value: 2
        }   
    ];
    function placeholder(){
        let test = "ok";
    }

    export let activeTabValue = 1;
    const handleClick = tabValue => () => (activeTabValue = tabValue); 
</script>
    <ul class="sidebar">
        {#each items as item}
            <li style="list-style: none;">
                <div class="tabSelect">
                    {#if item.value === 1}
                        <button class="button" style="background-image: url(/piano.svg);background-position: center;background-repeat: no-repeat;" on:click={handleClick(item.value)}></button>
                    {:else if item.value === 2}
                        <button class="button" style="background-image: url(/load.svg);background-position: center;background-repeat: no-repeat;" on:click={handleClick(item.value)}></button>
                    {/if}
                </div>
            </li>    
        {/each}
        <button class="setting" on:click={placeholder}></button>
    </ul>

    {#each items as item}
        {#if activeTabValue == item.value}
            <div>
                <svelte:component this={item.component}/>
            </div>
        {/if}    
    {/each}

<style>
    .sidebar{
        flex-direction: column;
        position: absolute;
        left: -5px;
        top: 0px;
        padding: 0px;
        width: 65px;
        height: 100%;
        background-color: #535050 ;
        margin: 0px; 
    }
    .button{
        height: 70px;
        width: 100%;
        position: relative;
        font-size: 12px;
        left: 0px;
        top: 1px;
        background-color:transparent;
        border-style: none;
        box-shadow: none;
    }
    .button:hover{
        border: 1px solid white;
        background-color:#17202a;
    }
    .button:focus{
        border: 1px solid white;
        background-color: #17202a ;
    }
    .setting{
        position: fixed;
        bottom: 5px;
        width: 60px;
        height: 60px;
        left: 0px;
        padding: 15px;
        background-color: transparent;
        box-shadow: none;
        background-image: url(/settings.svg);
        background-position: center;
        background-repeat: no-repeat;
        border-radius: 50%;
    }
</style>