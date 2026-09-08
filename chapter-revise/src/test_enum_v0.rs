#[derive(Debug)]
enum Block {

    A1b,
    A2a,
    C3g,
    K4z,

}

#[derive(Debug)]
enum Road {
  R401(Block),
  R338(Block),
  R886(Block),
  R609(Block),
}

fn block_to_road(road: Road) -> u8 {
  match road {
    Road::R401(block) => {
      println!("This is the 1st block, {block:#?}");
      13
    }
    Road::R338(block) => {
      println!("The Block the 2nd block, {block:#?}");
      15
    }
    Road::R886(block) => {
      println!("This is the 3rd block, {block:#?}");
      18
    }
    Road::R609(block) => {
      println!("This is the 4th block, {block:#?}");
      19
    }
  }
}

fn main() {
    println!("The road Block from the R401 is {:#?}", block_to_road(Road::R401(Block::A1b)));
    println!("The road Block from the R338 is {:#?}", block_to_road(Road::R338(Block::A2a)));
    println!("The road Block from the R886 is {:#?}", block_to_road(Road::R886(Block::C3g)));
    println!("The road Block from the R609 is {:#?}", block_to_road(Road::R609(Block::K4z)));
}
