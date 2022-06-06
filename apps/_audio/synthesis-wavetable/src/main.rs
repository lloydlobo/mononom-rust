use core::time::Duration;
use rodio::{source::Source, OutputStream};
use std::f32::consts::PI;

fn main() {
    println!("Hello, world!");

    let wave_table_size = 64;

    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size); // Constructs a new, empty Vec<T> with the specified capacity. // The vector will be able to hold exactly capacity elements without reallocating. If capacity is 0, the vector will not allocate.

    for n in 0..wave_table_size {
        // fills wave table with values of 1 sine period // arguments of sine increases from 0 to 2PI // 1st bracket for argument of sin function
        wave_table.push((2.0 * PI * n as f32 / wave_table_size as f32).sin());
    }
}

// using 9 type pairs to define the struct
struct WavetableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
} // struct is like a class in Java

impl WavetableOscillator {
    fn new(sample_rate: u32, wave_table: Vec<f32>) -> WavetableOscillator {
        return WavetableOscillator {
            sample_rate: sample_rate,
            wave_table: wave_table,
            index: 0.0,
            index_increment: 0.0,
        };
    } // new() is a convention in rust for object creating function

    fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }
    /*
    self is the instance of the object on which we'll invoke our method on // mut self is a convention in rust for object mutability

    &mut - reference to a mutable location borrowing of ownership of the object

    it frees memory from the stack and gives it to the heap
    */
}

/* REFERENCE
    ____  ______________________  _______   ______________
   / __ \/ ____/ ____/ ____/ __ \/ ____/ | / / ____/ ____/
  / /_/ / __/ / /_  / __/ / /_/ / __/ /  |/ / /   / __/
 / _, _/ /___/ __/ / /___/ _, _/ /___/ /|  / /___/ /___
/_/ |_/_____/_/   /_____/_/ |_/_____/_/ |_/\____/_____/

title: Wavetable Synthesis in Rust?? Rust Basics Tutorial [Synth #004]
author: Jan Wilczek from WolfSound
source: https://www.youtube.com/watch?v=v0Qp7eWVyes
related: https://github.com/JanWilczek/wolf-sound-blog/tree/master/_rust/synthesis/wavetable

*/

/* Wavetable Synthesis in Rust

- Wavetable is a table of values that can be used to synthesize a waveform.
- It is an array in memory which stores one period of a waveform, which we'll generate with the oscillators.
*/
