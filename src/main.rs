use std::io;
use std::collections::HashMap;

fn main() {
    println!("Insert string to have its characters counted");
    let mut input: String = String::new();
    let mut char_count: HashMap<char, u32> = HashMap::new();

    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{n} bytes read");

            for c in input.chars() { 
                if !char_count.contains_key(&c) {
                    char_count.insert(c, 1);
                }
                else{
                    *char_count.get_mut(&c).unwrap() += 1;
                }
            }

            let mut sorted_char_count: Vec<_> = char_count.iter().collect();
            sorted_char_count.sort_by_key(|a| a.0);

            for (key, value) in &sorted_char_count {
                println!("{}: {}", key, value);
            }
        }
        Err(error) => println!("error: {error}"),
    }
}
