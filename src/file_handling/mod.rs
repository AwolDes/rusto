use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::io;

pub fn read_file(file_directory: String) -> io::Result<String>{
    //println!("Hello!");
    let mut f = try!(File::open(file_directory));
    let mut s = String::new();
    let file_content = try!(f.read_to_string(&mut s));
    //println!("{:?}", s);

    Ok(s)
}

pub fn remove_file(file_directory: String) -> io::Result<()> {
    println!("Deleted: {}", file_directory);
    try!(fs::remove_file(file_directory));
    Ok(())
}

pub fn write_file(file_directory: String, data: &Vec<u8>) -> io::Result<()> {
    let mut f = try!(File::create(file_directory));
    //let write_data = &[u8] = &data;
    try!(f.write_all(data.as_slice()));
    Ok(())
}
