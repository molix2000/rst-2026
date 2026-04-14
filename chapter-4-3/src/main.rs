mod stringequal;
mod tempconvert;
mod looper;
mod loopmania;
mod change;
use change::change;
use loopmania::loopmania;
use looper::looper;
use tempconvert::tempconvert;
use stringequal::*;
// use loopmania::*;

fn main() {
    println!("loopmania start");
    loopmania();
    looper();
    tempconvert();
}
