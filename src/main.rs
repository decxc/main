use std::env;
use std::{thread,time};
use rand::prelude::*;
use rand::seq::*;
fn main() {
        let rand_time = 0;
        let now = std::time::Instant::now();
        
        loop{
        let ten_millis = time::Duration::from_millis(100);
        let mut rng = thread_rng();
        let mut y = ["O","L","N","G","_"];
        // println!("Unshuffled: {:?}", y);
        y.shuffle(&mut rng);
        println!("Shuffled:   {:?}", y);
        thread::sleep(ten_millis);

        if y == ["L","_","N","G","O"]{
            let elapsed_time = now.elapsed();
            println!("Random found wordle in {} seconds",elapsed_time.as_secs());
            break
        }
    }

}