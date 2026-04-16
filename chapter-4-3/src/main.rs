mod mutcomplex;
mod tempconvert;
mod looper;
mod loopmania;
mod mutabex;
mod mutatext;
use mutcomplex::mutcomplex;
use loopmania::loopmania;
use looper::looper;
use tempconvert::tempconvert;
use mutabex::mutabex;
use mutatext::mutatext;
// use loopmania::*;

fn main() {
    println!("loopmania start");
    loopmania();
    looper();
    tempconvert();
    mutcomplex();
    mutabex();
    mutatext();
}
