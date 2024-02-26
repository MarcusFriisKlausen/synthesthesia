# Synthesthesia

**Hi there!**

This is a small side project I'm pursuing, where my goal is to make a digital synth.

The project currently has an oscillator and has functionallity for listening to MIDI input. 

The main function currently has a phase of listening to and printing MIDI input using the [midir crate](https://docs.rs/midir/latest/midir/#). The user is prompted to press enter, whereatfter a Middle C is produced by an oscillator using the [rodio crate](https://docs.rs/rodio/latest/rodio/).

The next step is connecting MIDI input with correlating sound output using an oscillator.