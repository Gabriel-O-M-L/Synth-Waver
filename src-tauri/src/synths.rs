use self::wavetable::WaveTableOscillator;

pub mod wavetable;


fn create_wavetable(capacity: usize) -> Vec<f32>{
    let mut wave_table: Vec<f32> = Vec::with_capacity(capacity);
    
    for n in 0..capacity {
        wave_table.push((2.0 * std::f32::consts::PI * n as f32 / capacity as f32).sin());
    }
    return wave_table;
}
pub fn create_wave_table_oscilator(sample_rate: u32, capacity: usize) -> WaveTableOscillator{
    let wave_table = create_wavetable(capacity);
    let oscillator: wavetable::WaveTableOscillator = wavetable::WaveTableOscillator::new(sample_rate, wave_table);
    return oscillator;
}
pub fn set_wave_table_frequency(mut oscillator: WaveTableOscillator,frequency: f32) -> WaveTableOscillator{
    oscillator.set_frequency(frequency);
    return oscillator;
}
