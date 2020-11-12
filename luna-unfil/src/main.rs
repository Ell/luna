use clap::Clap;
use libfil::Fil;
use std::io::prelude::*;
use std::{fs, fs::File, path::Path};

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Ell <projects@ell.dev>")]
struct Opts {
    #[clap(short, long)]
    input_file: String,
    #[clap(short, long)]
    output_path: String,
}

fn main() {
    let opts = Opts::parse();

    let output_path = Path::new(&opts.output_path);
    if !output_path.exists() {
        eprintln!("Invalid or missing output path");
        std::process::exit(1);
    }

    let input_path = Path::new(&opts.input_file);
    if !input_path.exists() {
        eprintln!("Invalid or missing input file");
        std::process::exit(1);
    }

    let file = fs::read(input_path).unwrap();
    let fil = Fil::from_buffer(file);

    fil.entries.iter().for_each(|entry| {
        let path = output_path.join(&entry.path);
        let mut file = File::create(&path).unwrap();

        let buffer = fil.read_entry(entry);

        file.write_all(buffer).unwrap();

        println!("Wrote {}", path.to_str().unwrap());
    });
}
