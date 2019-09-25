#[derive(Debug)]
struct Plug(String, String); //Struct to hold plugboard settings.

fn main() {

	let pluga = Plug("A".to_string(), 'B'.to_string());
	let plugb = Plug('c'.to_string(),'d'.to_string());
	println!("Pluga : {:?}", pluga);
	
	let plugboard: Vec<Plug> = vec![pluga,plugb];
	println!("Plugboard Settings: {:?}", plugboard);

	let mut string = String::from("ABBA");

	println!("{}",string);
	plugboard_transform(&mut string, plugboard);
	println!("{}",string);
}

fn plugboard_transform(s: &mut String, settings: Vec<Plug>) {
	let settings_iter = settings.iter();
	for set in settings_iter {
		*s = s.replace(&set.0 , &set.1);
	}
}
