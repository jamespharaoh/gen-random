use std::char;
use std::env;
use std::process;
use std::path::Path;

use rand;
use rand::distributions::Distribution;
use rand::distributions::Uniform;

fn main () {

	let args: Vec <String> = env::args ().collect ();
	let command_path = Path::new (args [0].as_str ());
	let command_name = command_path.file_name ().unwrap ().to_str ().unwrap ();

	match command_name {

		"gen-password" => {

			let uniform = Uniform::from ('a' as u32 .. 'z' as u32);
			let mut password = String::new ();
			let mut rng = rand::thread_rng ();

			for _ in 0 .. 20 {
				let ch = char::from_u32 (uniform.sample (& mut rng)).unwrap ();
				password.push (ch);
			}

			println! ("{}", password);

		},

		_ => {

			eprintln! ("Program name not recognised: {}", command_name);

			process::exit (1);

		}

	}

}

