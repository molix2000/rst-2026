mod dangle;
mod experimenter01;
mod looper;
mod loopmania;
mod mutabex;
mod mutatext;
mod mutcomplex;
mod usizer;
use dangle::dangle;
use dangle::dangles;
use experimenter01::display_vec_position_value;
use experimenter01::experimenter01;
use looper::looper;
use loopmania::loopmania;
use mutabex::mutabex;
use mutatext::mutatext;
use mutcomplex::mutcomplex;
use usizer::*;
// use loopmania::*;

fn main() {
    println!("loopmania start");
    loopmania();
    looper();
    mutcomplex();
    mutabex();
    mutatext();
    dangle();
    usizer();
    experimenter01();
}
