#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod synths;
mod notes;
use tokio;
use notes::create_notes_keyboard;
use notes::twelve_tone::TwelveTone;
use rodio::{OutputStream, source::Source};

fn twelve_tone_keyboard() -> notes::twelve_tone::TwelveTone{
    let mut notes_system = create_notes_keyboard();
    notes_system.create_notes_frequency();
    return notes_system;
}
#[tauri::command]
async fn waveTable(sampleRate: u32,capacity: usize,frequency: f32,waveform: i8){
    let mut oscillator = synths::create_wave_table_oscilator(sampleRate, capacity,waveform);
    oscillator = synths::set_wave_table_frequency(oscillator, frequency);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let _result = stream_handle.play_raw(oscillator.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(30));
} 

#[tauri::command]
async fn inputHandler(noteSystem: i8,sampleRate: u32,capacity: usize){
    let keyboard: TwelveTone = twelve_tone_keyboard();
    tokio::spawn(async{
        loop{

        }
    });
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![waveTable,inputHandler])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}