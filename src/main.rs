extern crate dirs;
extern crate bat;
extern crate clap;

use clap::{App, Arg};
use bat::{PrettyPrinter, line_range::LineRange, line_range::LineRanges};
use std::time::SystemTime;
use std::path::Path;
use std::{io, fs, process};


fn trim_path(p: &Path) -> String {

    let dl_dir = dirs::download_dir().unwrap();    
    p.strip_prefix(dl_dir).unwrap().to_str().unwrap().to_string()

}

// Helper function to save repeating error checking and print statements
fn move_file(src: &Path, dest_folder: &Path) {
    let f_name = trim_path(src);
    let new_f_name = dest_folder.join(f_name);
    fs::rename(src, new_f_name).unwrap();
    
    println!("Moved to {}", dest_folder.to_str().unwrap());
    println!("-------------------------------------------")
}

fn remove_file_or_dir(src: &Path){
    let metadata = fs::metadata(&src).unwrap();
    if metadata.is_dir() {
        fs::remove_dir_all(src).unwrap();
    } else {
       fs::remove_file(src).unwrap();
    }

    println!("-------------------------------------------")
}

fn bat_file(src: &Path) {
    PrettyPrinter::new().input_file(src)
                        .line_ranges(LineRanges::from(vec![LineRange::new(0,15)]))
                        .print().unwrap();
    println!("-------------------------------------------")
}

fn delete_or_move_file(p: &Path) {
    println!("{}", trim_path(p));
    println!("Move to (d)ocuments, (m)usic, (p)ictures, (v)ideos/mo(v)ies");
    println!("You can also (b)at the file, (o)pen or (r)emove it");
    println!("You can (a)bort at any time");

    let mut input = String::new();
    io::stdin()
       .read_line(&mut input)
       .expect("Failed to read line");

   match input.trim().as_ref() {
       "d" => move_file(p, &dirs::document_dir().unwrap()),
       "m" => move_file(p, &dirs::audio_dir().unwrap()),
       "p" => move_file(p, &dirs::picture_dir().unwrap()),
       "v" => move_file(p, &dirs::video_dir().unwrap()),
       "r" => remove_file_or_dir(p),
       "a" => process::exit(0x0100),
       "b" => {
                bat_file(p); 
                delete_or_move_file(p);
              },
       "o" => {
                process::Command::new("open")
                                     .arg(p)
                                     .output()
                                     .expect("unable to open file, this command only works on Mac");
                delete_or_move_file(p);
              },
        _  =>   println!("Skipped File")
   }
}

fn main() -> io::Result<()>{
    let matches = App::new("Download Caretaker (rust)")
                        .version("1.0")
                        .author("Jim Walker")
                        .arg(Arg::with_name("days")
                            .short("d")
                            .long("days")
                            .help("Set the maximum age in days for items in your downloads folder")
                            .takes_value(true))
                        .about("Keeps your downloads folder tidy")
                        .get_matches();

    let days = matches.value_of("days")
                    .unwrap_or("30")
                    .parse::<u64>()
                    .unwrap();
    println!("Download Caretaker Rust");
    let dl_dir = dirs::download_dir().unwrap();    
    let paths: Vec<_> = fs::read_dir(&dl_dir)?
                            .filter_map(Result::ok)
                            .map(|entry| entry.path()).collect();
    for path in paths {
        let metadata = fs::metadata(&path).unwrap();
        if let Ok(file_modified) = metadata.modified(){
            // If file is older than a month, delete it
            if let Ok(file_age) = SystemTime::now().duration_since(file_modified){
                if file_age.as_secs() > 60 * 60 * 24 * days {
                    delete_or_move_file(&path);
                }
            }
        }
    }
    Ok(())
}
