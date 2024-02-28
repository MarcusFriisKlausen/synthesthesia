use std::io::{stdin, stdout, Write};
use std::error::Error;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use midir::{MidiInput, Ignore};

/// Returns a CSV Reader containing corresponding midi values, notes and frequencies.
fn init_reader() -> csv::Reader<File> {
    let filename: &Path = Path::new("./data/midi_to_notes.csv");
    let csv_parser: csv::Reader<File> = 
        csv::Reader::from_path(filename).unwrap();

    return csv_parser;
}

/// Returns a hashmap with information regarding MIDI keys' conversion to 
/// notes and their frequencies.
/// Keys corresponds to MIDI values.
/// Values are a tuple of (frequency, note).
fn init_key_map() -> HashMap<u8, (f32, String)> {
    let mut csv_parser = init_reader();
    let mut key_map: HashMap<u8, (f32, String)> = HashMap::new();
    for record in csv_parser.records() {
        let rec_unwrapped = record.unwrap();
        let (field0, field1, field2) = 
            (rec_unwrapped.get(0), rec_unwrapped.get(1), rec_unwrapped.get(2));
        
        let midi_record;

        match (field0, field1, field2) {
            (Some(midi), Some(note), Some(freq)) =>
                midi_record  = (midi.to_owned(), note.to_owned(), freq.to_owned()),
            _ => panic!()
        }

        key_map.insert(
            midi_record.0.parse().unwrap(), 
            (midi_record.2.parse().unwrap(), midi_record.1)
        );
    }

    return key_map;
}

pub struct MidiConverter {
    // TODO: should be used in input_loop() to translate input to waves.
    _key_map: HashMap<u8, (f32, String)>
} 

impl MidiConverter {    
    pub fn new() -> MidiConverter {
        return MidiConverter {
            _key_map: init_key_map()
        }
    }
    
    // TODO: refactor the loop into new class which brings the midi converter, waves and oscillator
    // together.
    /// Opens MIDI input port and listens for input
    pub fn input_loop(self) -> Result<(), Box<dyn Error>> {
        let mut input: String = String::new();

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

        // _conn_in needs to be a named parameter, because it needs to be kept alive until 
        // the end of the scope
        let _conn_in: midir::MidiInputConnection<()> = 
            midi_in.connect(
                in_port, 
                "midir-read-input", 
                move |stamp, message, _| {
                    println!("{}: {:?} (len = {})", stamp, message, message.len());
                }, 
                ()
            )?;
        
        println!("Connection open, reading input from '{}' (press enter to exit) ...", in_port_name);

        input.clear();
        stdin().read_line(&mut input)?; // wait for next enter key press
        
        println!("Closing connection");
        Ok(())
    }
}
