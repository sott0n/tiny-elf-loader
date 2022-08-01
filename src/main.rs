use std::env;
use std::fs::File;
use std::io::Read;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("This ELF loader's args must be only object file name.");
        exit(1);
    }

    let filename = &args[1];
    if !filename.ends_with(".o") {
        eprintln!("Arg filename must be `.o`");
        exit(1);
    }

    let mut f = File::open(filename).unwrap();
    let mut buf = Vec::new();

    assert!(f.read_to_end(&mut buf).unwrap() > 0);
}
