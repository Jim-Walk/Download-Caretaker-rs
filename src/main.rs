extern crate dirs;
use std::time::SystemTime;
use std::fs;

fn delete_or_move_file(&Path

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
                    println!("{} older than a week", path.display());
                }
            }
        }
    }
}
