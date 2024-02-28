mod synth;
mod oscillator;
mod midi_converter;

use synth::Synth;

fn main() {
    // Listen for midi input.
    // Message is [x, y, z] for key press:
    // x = 144 for key press, x = 128 for key release.
    // y is the in the range (0..=120) which is the different keys.
    // z is key velocity.
    // I haven't figured out what it prints for other knobs and buttons.
    let synth: Synth = Synth::new();
    
    match synth.input_loop() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}
