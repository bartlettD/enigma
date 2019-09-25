use std::collections::HashMap;

#[derive(Debug)]
struct Plug(char, char);

fn main() {
    println!("Hello, world!");
	let pluga = Plug('A', 'B');
	let plugb = Plug('c','d');
	println!("Pluga : {:?}", pluga);
	
	let Plugboard: Vec<Plug> = vec![pluga,plugb];
	println!("Plugboard Settings: {:?}", Plugboard);
}
