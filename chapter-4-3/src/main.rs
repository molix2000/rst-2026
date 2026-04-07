mod looper;
mod loopmania;
use loopmania::loopmania;
use looper::looper;
// use loopmania::*;

fn main() {
    println!("loopmania start");
    loopmania();
    looper();
}
