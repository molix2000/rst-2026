mod mutcomplex;
mod looper;
mod loopmania;
mod mutabex;
mod mutatext;
use mutcomplex::mutcomplex;
use loopmania::loopmania;
use looper::looper;
use mutabex::mutabex;
use mutatext::mutatext;
// use loopmania::*;

fn main() {
    println!("loopmania start");
    loopmania();
    looper();
    mutcomplex();
    mutabex();
    mutatext();
}
