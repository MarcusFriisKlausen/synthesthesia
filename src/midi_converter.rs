use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

/// Returns a CSV Reader containing corresponding midi values, notes and frequencies.
fn init_reader() -> csv::Reader<File> {
    let filename: &Path = Path::new("./data/midi_to_notes.csv");
    let csv_parser: csv::Reader<File> = 
        csv::Reader::from_path(filename).unwrap();

    csv_parser
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

    key_map
}

pub struct MidiConverter {
    // TODO: should be used in input_loop() to translate input to waves.
    pub key_map: HashMap<u8, (f32, String)>
} 

impl MidiConverter {    
    pub fn new() -> MidiConverter {
        MidiConverter {
            key_map: init_key_map()
        }
    }
}
