mod deconstructtupletwo;
mod stringequal;
mod tempconvert;
mod looper;
mod loopmania;
use loopmania::loopmania;
use looper::looper;
use tempconvert::tempconvert;
use stringequal::*;
use deconstructtupletwo::deconstructtupletwo;
use deconstructtupletwo::calculate_length;
// use loopmania::*;

fn main() {
    println!("loopmania start");
    loopmania();
    looper();
    tempconvert();
    deconstructtupletwo();
}
