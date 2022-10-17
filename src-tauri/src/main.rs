#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod synths;
use rodio::{OutputStream, source::Source};

#[tauri::command]
fn waveTable(sampleRate: u32,capacity: usize,frequency: f32){
    let mut oscillator: synths::wavetable::WaveTableOscillator = synths::create_wave_table_oscilator(sampleRate, capacity);
    oscillator = synths::set_wave_table_frequency(oscillator, frequency);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let _result = stream_handle.play_raw(oscillator.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(5));
} 


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![waveTable,greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
