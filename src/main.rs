extern crate basic_symmetric;

use std::io::prelude::*;
use std::fs::File;
use std::io;



fn read_file(file_name: String) -> io::Result<()> {
    let mut f = try!(File::open(file_name));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    println!("{:?}", s);
    Ok(())
}



fn main() {
    basic_symmetric::say_hi();
    //println!("Hello, world!");
    //let y = plus_one(5);
    // {
    //     let file_name = "blah.txt".to_string();
    //     let encrypted_name = "rip".to_string();
    //     let file_password = "iloveyou".to_string();
    //     encrypt_file(file_name, encrypted_name, file_password);
    // }
    //
    // {
    //     let encrypted_file_name = "rip".to_string();
    //     let decrypted_file_name = "yay".to_string();
    //     let password = "iloveyou".to_string();
    //
    //     decrypt_file(encrypted_file_name, password, decrypted_file_name);
    // }

}
