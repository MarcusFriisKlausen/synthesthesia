use rodio::{OutputStream, source::Source};

mod midi_converter;
mod wave;
mod oscillator;

const PI: f32 = std::f32::consts::PI;

fn main() {
    let midi_converter: midi_converter::MidiConverter = midi_converter::MidiConverter::new();

    // Listen for midi input.
    // Prints [x, y, z] for key press:
    // x = 144 for key press, x = 128 for key release.
    // y is the in the range (0..=120) which is the different keys.
    // z is key velocity.
    // I haven't figured out what it prints for other knobs and buttons.
    match midi_converter.input_loop() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }

    let wave: wave::Wave = wave::Wave::new(0.25, 261.63);
    let mut waveform: Vec<f32> = Vec::with_capacity(oscillator::WAVETABLE_SIZE);

    for i in 0..oscillator::WAVETABLE_SIZE {
        waveform.push(
            wave.get_amp()
            * (2.0 * PI * i as f32 / oscillator::WAVETABLE_SIZE as f32)
            .sin()
        );
    }

    let mut oscillator: oscillator::Oscillator = 
        oscillator::Oscillator::new(oscillator::SAMPLE_RATE, waveform);
    
    oscillator.set_frequency(wave.get_freq());

    let (_stream, stream_handle) = 
        OutputStream::try_default().unwrap();
    
    let _result: Result<(), rodio::PlayError> = 
        stream_handle.play_raw(oscillator.convert_samples());

    std::thread::sleep(std::time::Duration::from_secs(5));
}
