mod dangle;
mod looper;
mod loopmania;
mod mutabex;
mod mutatext;
mod mutcomplex;
use dangle::dangle;
use dangle::dangles;
use looper::looper;
use loopmania::loopmania;
use mutabex::mutabex;
use mutatext::mutatext;
use mutcomplex::mutcomplex;
// use loopmania::*;

fn main() {
    println!("loopmania start");
    loopmania();
    looper();
    mutcomplex();
    mutabex();
    mutatext();
    dangle();
}
