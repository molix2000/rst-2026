mod mutcomplex;
mod tempconvert;
mod looper;
mod loopmania;
mod mutabex;
use mutcomplex::mutcomplex;
use loopmania::loopmania;
use looper::looper;
use tempconvert::tempconvert;
use mutabex::mutabex;
// use loopmania::*;

fn main() {
    println!("loopmania start");
    loopmania();
    looper();
    tempconvert();
    mutcomplex();
    mutabex();
}
