
   <div class="sidebar">
      <button class="button">New  Synth</button>
      <button class="button">Load Synth</button>
      <button class="setting"></button>
    </div>
      
    <style>
    .sidebar{
        flex-direction: column;
        position: absolute;
        left: -15px;
        top: 0px;
        padding: 15px;
        width: 50px;
        height: 100%;
        text-decoration: none;
        font-size: 12px;
        color: white;
        background-color: #535050 ;
        border: none;
    }
    .button:hover{
        border: 1px solid white;
        background-color:#2c3e50  ;
    }
    .button{
        height: 50px;
        background-color:transparent;
        border-style: none;
        box-shadow: none;
    }
    .setting:hover{
        border: 1px solid white;
        background-color:#2c3e50  ;
    }
    .setting{
        position: fixed;
        bottom: 5px;
        width: 65px;
        height: 60px;
        padding: 15px;
        background-color: transparent;
        box-shadow: none;
        border-radius: 50%;
    }
</style>