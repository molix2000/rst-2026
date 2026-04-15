mod mutcomplex;
mod tempconvert;
mod looper;
mod loopmania;
use mutcomplex::mutcomplex;
use loopmania::loopmania;
use looper::looper;
use tempconvert::tempconvert;
// use loopmania::*;

fn main() {
    println!("loopmania start");
    loopmania();
    looper();
    tempconvert();
    mutcomplex();
}
