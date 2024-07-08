use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

fn print_usage() {
    println!(
        r#"
        _____       .___  ._____________
        /  _  \    __| _/__| _/\______   \ ____
       /  /_\  \  / __ |/ __ |  |     ___// __ \
      /    |    \/ /_/ / /_/ |  |    |   \  ___/
      \____|__  /\____ \____ |  |____|    \___  >
              \/      \/    \/                \/
                                version: 1.0
                                author:  NoobAuto
  "#);

    println!("Usage of AddPe.exe\n");
    println!("  -D string");
    println!("\tPath to original file");
    println!("  -S string");
    println!("\tHow many MBs to increase the file by");
    println!("  -O string");
    println!("\tOutput file name");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "-h" {
        print_usage();
        return Ok(());
    }

    if args.len() < 6 || args.len() % 2 != 0 || args[1] != "-D" || args[3] != "-T" || args[args.len() - 2] != "-O" {
        println!("Usage: AddPe.exe -D inputfile -T MB -O outputfile");
        return Ok(());
    }

    let former_file_name = &args[2];
    let mb_size: usize = args[4].parse().unwrap_or_else(|_| {
        println!("Invalid size format.");
        std::process::exit(1);
    });
    let new_file_name = &args[args.len() - 1];

    if !Path::new(former_file_name).exists() {
        println!("File doesn't exist.");
        return Ok(());
    }

    if mb_size <= 0 {
        println!("Size cannot be negative or zero.");
        return Ok(());
    }

    let mut old_file = File::open(former_file_name)?;
    let mut old_file_content = Vec::new();
    old_file.read_to_end(&mut old_file_content)?;

    let old_file_size = old_file_content.len();
    let new_size = mb_size * 1024 * 1024;

    if new_size <= old_file_size {
        println!("New size must be larger than the original file size.");
        return Ok(());
    }

    let padding_size = new_size - old_file_size;
    let mut padding_bytes = vec![0u8; padding_size];

    let mut new_file = File::create(new_file_name)?;
    new_file.write_all(&old_file_content)?;
    new_file.write_all(&padding_bytes)?;

    println!(
        "Added {} MB of padding to the {} file, saved as {}",
        mb_size, former_file_name, new_file_name
    );

    Ok(())
}
