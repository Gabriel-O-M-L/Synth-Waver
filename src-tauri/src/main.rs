#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod synths;
mod notes;
use notes::create_notes_keyboard;
use notes::twelve_tone::TwelveTone;
use rodio::{OutputStream, source::Source,Sink};
use std::thread;
use std::sync::mpsc;

fn twelve_tone_keyboard() -> notes::twelve_tone::TwelveTone{
    let mut notes_system = create_notes_keyboard();
    notes_system.create_notes_frequency();
    return notes_system;
}
#[tauri::command]
async fn waveTable(sampleRate: u32,capacity: usize,frequency: f32,waveForm: i8){
    let mut oscillator = synths::create_wave_table_oscilator(sampleRate, capacity,waveForm);
    
    oscillator = synths::set_wave_table_frequency(oscillator, frequency);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(oscillator);
    let stop:i8 = 0;
    
    while(true){
        stop = rx.re
    }
    sink.sleep_until_end();
    //let _result = stream_handle.play_raw(oscillator.convert_samples());
    
    //std::thread::sleep(std::time::Duration::from_secs(1));
} 


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![waveTable])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}