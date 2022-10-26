# modular synth app, made with tauri and svelte 
(Right now Studying for a Web assembly port and migration)

### Reason for migration:
- Delay in api calls between front-end and back-end
- Allow the use of Web apis in back-end
- Allow the usange o native web Input handler, witch will remove delay between button press and sound
- It allows for an easy way to render graphics for math functions

## Peding implementation :
Synth configuration

### For wavetable:
- Frequency
- Sample rate
- Wave table length control

Wavetable algorithm based on the following article https://thewolfsound.com/sound-synthesis/wavetable-synth-in-rust/

### Other synth with pending implementation

- Additive Synthesis,
- Subtractive Synthesis,
- FM Synthesis,
- Sample-based Synthesis,
- Vector Synthesis,
- Granular Synthesis,

## Functions supported as of this moment

- Square wave
- Sin wave
