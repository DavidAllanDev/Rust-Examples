extern crate phrases;
use phrases::greetings::french;

fn main() {
    println!("English: {}, {}", 
      phrases::greetings::english::hello(), 
      phrases::greetings::english::goodbye()
    );
    println!("French: {}, {}", 
      french::hello(), 
      french::goodbye()
    );
}