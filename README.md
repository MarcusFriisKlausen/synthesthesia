# Synthesthesia

**Hi there!**

This is a small side project I'm pursuing, where my goal is to make a digital synth.

Currently the program takes MIDI input and produces the correlating notes which are output as sounds and printed to the console. For now it's not working optimally, as the waves are playing in a fixed time interval, which loops on key hold, and notes can't overlap. These problems will be fixed in the future. 

The project is made by gathering MIDI input using [midir crate](https://docs.rs/midir/latest/midir/#). Audio output is implemented using [rodio crate](https://docs.rs/rodio/latest/rodio/).
