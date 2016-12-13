
use std::fs;
use std::io;
use std::fs::{DirEntry};
use std::path::Path;
// use part of your lib :O
use basic_symm;

pub fn read_directory(path: String) -> io::Result<()> {
    for entry in try!(fs::read_dir(path)) {
        let dir = try!(entry);
        println!("{:?}", dir.path());
    }
    Ok(())
}

pub fn visit_dirs(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            println!("{:?}", entry);
            let path = entry.path();
            let path_str = path.to_str();
            println!("{:?}", path_str.unwrap());

            //assert_eq!(path_str, Some("hello.txt"));
            let file_name = "ripu";
            let pass = "haha";
            basic_symm::encrypt_file(path_str.unwrap().to_string(), file_name.to_string(), pass.to_string());
            // needs modification
            if path.is_dir() {
                try!(visit_dirs(&path));
            }
        }
    }
    Ok(())
}
