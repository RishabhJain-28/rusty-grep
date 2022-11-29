use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let search_query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", search_query);
    println!("In file {}", file_path);
}
