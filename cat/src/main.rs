use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Result;

fn reader(reader: &mut Read) -> Result<()> {
	let mut buf = String::new();
	reader.read_to_string(&mut buf)?;
	print!("{}", buf);
	Ok(())
}

fn main() -> Result<()> {
	let args: Vec<String> = env::args().collect();
	if args.len() == 1 {
		reader(&mut io::stdin())?;
	} else {
		for i in 1..args.len() {
			let arg = args[i].clone();
			if arg == "-" {
				reader(&mut io::stdin())?;
			} else {
				let mut file = File::open(arg)?;
				reader(&mut file)?;
			}
		}
	}
	Ok(())
}

