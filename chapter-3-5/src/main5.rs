fn main() {
   let mut cake = 0;
   let _cream_load = loop {
       cake += 1; 
       if cake == 10 {
       break cake * 3;
      }
   };

   println!("The cake count was {cake}");
  
}
