# Reference can have different types.</p>

### Question what is the difference between 


1 -</p>
```
   &self

```
   When using &self , you are telling the  compiler that method only needs to read data from the struct, not change it, once read only, multiple parts of your code can call this method. at the exact same time. without causing data conflicts.

  * Use case, Getteers, calculateing a value, printing status, or checking a condition.

```
   rust
   struct Player {

          name: String,
          score: u32,
   }

   impl Player {


           // Uses &self because it only reads the score
           fn display_score(&self) {

               println!("{} has a score of {}.", self.name, self.score);

    }

```
   and

2-</p>
```
   &mut self

```

    This is a mutable borrow. You are telling the compiler that the method needs to modify, (write to) thre struct's data. Rust's strict concurency rules dictate that if you are mutating data. no one else can be reading or writing to it at the same time.

Use case: Setters, update state,incrementing counters, or cleaning data.

```
  rust
  impl Player {

       // Users &mut self because it modifies the score
       fn gain_points(&mut self, points: u32) {
          self.score += points; // This line requires mutation
       }
  }




```

Why des this disctinction matter? Rust enforces these rules at compile time to prevent data races. 
If you try &mut self whilst anther part of the code is holding reference to the same object. Rust will throw a compiler error.


```
  rust

  let mut player1  = Player {

     name: String::from("Alice"), 
     score: 10,
    
  };
  
 let score_ref: &player1.score; //mutable borrow occurs here
 player1.gain_points(5); 
  // X Error here ! Cannot borrow 'player1' as mutable because its already borrowed as immutable
 println!("Old score was : {}",score _ref);
 
  
```

What about just: self
If a method takes self only (without the &), this means that method takes total ownership of the object. Once that method finishes, the object is destoryed, (dropped). and can never be used again. This is common for conversationmethods. like .ino_string().
