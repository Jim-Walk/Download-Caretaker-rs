extern crate dirs;
use std::time::SystemTime;
use std::path::Path;
use std::{io, fs, process};

fn delete_or_move_file(p: &Path){
    println!("{}", p.display());
    println!("Move to (d)ocuments, (m)usic, (p)ictures, mo(v)ies or (o)pen");
    println!("You can (a)bort at any time");

    let mut input = String::new();
    io::stdin()
       .read_line(&mut input)
       .expect("Failed to read line");

   match input.trim().as_ref() {
       "d" => println!("moved to documents"),
       "a" => process::exit(0x0100),
       _ => println!("idk")
   }
}

fn main() {
    println!("Download Caretaker Rust");
    let download_dir = dirs::download_dir().unwrap();    
    let paths = fs::read_dir(download_dir).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let metadata = fs::metadata(&path).unwrap();
        if let Ok(file_modified) = metadata.modified(){
            // If file is older than a month, delete it
            if let Ok(file_age) = SystemTime::now().duration_since(file_modified){
                if file_age.as_secs() > 60 * 60 * 24 * 7 {
                    delete_or_move_file(&path);
                }
            }
        }
    }
}
