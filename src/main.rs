use rand::prelude::*;
use rand::distributions::Uniform;
use std::char;
use std::env;
use std::iter;
use std::process;
use std::path::Path;

mod diceware;

fn main () {

	let args: Vec <String> = env::args ().collect ();
	let command_path = Path::new (args [0].as_str ());
	let command_name = command_path.file_name ().unwrap ().to_str ().unwrap ();

	match command_name {

		"gen-diceware" => {
			let num_words = match args.len () {
				1 => 6,
				2 => args [1].parse ().unwrap (),
				_ => panic! (),
			};
			if num_words < 1 { panic! () }
			let word_list = diceware::EFF_LONG_1;
			let mut rng = rand::thread_rng ();
			let words: Vec <& str> = word_list.choose_multiple (& mut rng, num_words)
				.copied ().collect ();
			let password = words.join (" ");
			println! ("{}", password);
		},

		"gen-password" => {
			let num_chars = match args.len () {
				1 => 20,
				2 => args [1].parse ().unwrap (),
				_ => panic! (),
			};
			let mut rng = rand::thread_rng ();
			let uniform = Uniform::from ('a' as u32 .. 'z' as u32);
			let password: String = iter::repeat_with (
				|| char::from_u32 (uniform.sample (& mut rng)).unwrap (),
			).take (num_chars).collect ();
			println! ("{}", password);
		},

		_ => {
			eprintln! ("Please invoke via a symlink:");
			eprintln! ();
			eprintln! ("  gen-diceware ...  Diceware words");
			eprintln! ("  gen-password ...  Lowercase ASCII characters");
			eprintln! ();
			process::exit (1);
		},

	}

}

