use self::wavetable::WaveTableOscillator;
pub mod wavetable;

fn create_wavetable(capacity: usize,waveform: i8) -> Vec<f32>{
    let mut wave_table: Vec<f32> = Vec::with_capacity(capacity);
    if waveform == 1{
        for n in 0..capacity {
            wave_table.push((2.0 * std::f32::consts::PI * n as f32 / capacity as f32).sin());
        }
    }else if waveform == 2{
        for n in 0..capacity {
            wave_table.push(((2.0 * std::f32::consts::PI * n as f32 / capacity as f32).sin()).signum());
        }
    }

    return wave_table;
}
pub fn create_wave_table_oscilator(sample_rate: u32, capacity: usize,waveform: i8) -> WaveTableOscillator{
    let wave_table = create_wavetable(capacity,waveform);
    let oscillator: wavetable::WaveTableOscillator = wavetable::WaveTableOscillator::new(sample_rate, wave_table);
    return oscillator;
}
pub fn set_wave_table_frequency(mut oscillator: WaveTableOscillator,frequency: f32) -> WaveTableOscillator{
    oscillator.set_frequency(frequency);
    return oscillator;
}
 