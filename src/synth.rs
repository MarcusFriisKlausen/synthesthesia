use std::io::{stdin, stdout, BufRead, BufReader, Write};
use std::error::Error;
use std::fs::File;
use midir::{MidiInput, Ignore};
use rodio::{OutputStream, source::Source};

use crate::midi_converter::MidiConverter;
use crate::oscillator::{Oscillator, SAMPLE_RATE, WAVETABLE_SIZE};

const PI: f32 = std::f32::consts::PI;

/// For now, a synth with a simple sine wave.
pub struct Synth {
}

impl Synth {
    pub fn new() -> Synth {
        Synth {
        }
    }

    // CURRENT ITERATION IS A TEMPORARY SOLUTION
    // PROBLEMS:
    // CAN ONLY PLAY 1 NOTE AT A TIME. NO OVERLAPPING NOTES.
    // NOTES ARE PLAYED FOR A FIXED AMOUNT OF TIME, NOT EXACTLY FROM PRESS TO RELEASE,
    // THOUGH THE FIXED SAMPLES LOOP FROM PRESS TILL RELEASE.

    /// Opens MIDI input port connection and listens for input.
    /// Connection prints note corresponding to input.
    /// When input is detected, the connection writes the MIDI value to a file ./input/input.txt.
    /// A play loop then checks for key presses and releases, and plays note until released.
    pub fn input_loop(self) -> Result<(), Box<dyn Error>> {
        let mut midi_in: MidiInput = MidiInput::new("midir reading input")?;
        midi_in.ignore(Ignore::None);

        // Get an input port (read from console if multiple are available)
        let in_ports: Vec<midir::MidiInputPort> = midi_in.ports();
        let in_port: &midir::MidiInputPort = 
            match in_ports.len() {
                0 => return Err("no input port found".into()),
                1 => {
                    println!(
                        "Choosing the only available input port: {}", 
                        midi_in.port_name(&in_ports[0]).unwrap()
                    );
                    &in_ports[0]
                },
                _ => {
                    println!("\nAvailable input ports:");
                    for (i, p) in in_ports.iter().enumerate() {
                        println!("{}: {}", i, midi_in.port_name(p).unwrap());
                    }
                    print!("Please select input port: ");
                    stdout().flush()?;
                    let mut input: String = String::new();
                    stdin().read_line(&mut input)?;
                    in_ports.get(input.trim().parse::<usize>()?)
                             .ok_or("invalid input port selected")?
                }
            };
        
        println!("\nOpening connection");
        let in_port_name: String = midi_in.port_name(in_port)?;

        let mut file: File = File::create("./inputs/inputs.txt")?;

        // _conn_in needs to be a named parameter, because it needs to be kept alive until 
        // the end of the scope.
        let _conn_in: midir::MidiInputConnection<()> = 
            midi_in.connect(
                in_port, 
                "midir-read-input", 
                move |_stamp: u64, message: &[u8], _| {
                    //println!("{}: {:?} (len = {})", stamp, message, message.len());
                    if message[0] == 144 {
                        let midi_converter: MidiConverter = MidiConverter::new();
                        let note = &midi_converter.key_map[&message[1]].1;
                        println!("{}", note);
                        let midi_value = format!("{}\n", message[1]);
                        file.write_all(
                            midi_value.as_bytes()
                        ).expect("Couldn't write to file");
                    } else if message[0] == 128 {
                        file.write_all(b"Release\n").expect("Couldn't write to file");
                    }
                }, 
                ()
            )?;

        println!("Connection open, reading input from '{}'...", in_port_name);
        
        loop {
            let amplitude = 0.25;  
            let mut waveform: Vec<f32> = Vec::with_capacity(WAVETABLE_SIZE);
                for i in 0..WAVETABLE_SIZE {
                    waveform.push(
                        amplitude
                        * (2.0 * PI * i as f32 / WAVETABLE_SIZE as f32)
                        .sin()
                        );
                }
                
            let mut oscillator: Oscillator = Oscillator::new(SAMPLE_RATE, waveform);
            let midi_converter = MidiConverter::new();

            let reader: BufReader<File> = BufReader::new(
                File::open("./inputs/inputs.txt").expect("Couldn't open file.")
            );
            let mut last_line: String = "".to_string();
            for line in reader.lines() {
                last_line = line.expect("Couldn't read line.");
            }
            if last_line == "" {
            } else if last_line != "Release" {
                let midi_value = last_line.parse::<u8>().expect("ParseIntError");
                let current_frequency = midi_converter.key_map[&midi_value].0;
                oscillator.set_frequency(current_frequency);
                
                let (_stream, stream_handle) = 
                            OutputStream::try_default().unwrap();

                let _result: Result<(), rodio::PlayError> = 
                    stream_handle.play_raw(oscillator.convert_samples());

                std::thread::sleep(std::time::Duration::from_millis(200));
            }
        }

        //println!("Closing connection");
        //Ok(());
    }
}