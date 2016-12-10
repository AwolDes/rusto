extern crate encryptfile;

use std::io::prelude::*;
use std::fs::File;
use std::io;
use encryptfile as ef;


fn read_file(file_name: String) -> io::Result<()> {
    let mut f = try!(File::open(file_name));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    println!("{:?}", s);
    Ok(())
}

fn encrypt_file(mut file_name: String, encrypted_file_name: String) {
    println!("Encrypting file: {}", file_name);
    let encrypted_name = encrypted_file_name + ".ef";

    {
        let in_file = &mut file_name;


        let mut c = ef::Config::new();
        c.input_stream(ef::InputStream::File(in_file.to_owned()))
         .output_stream(ef::OutputStream::File(encrypted_name.to_owned()))
         .add_output_option(ef::OutputOption::AllowOverwrite)
         .initialization_vector(ef::InitializationVector::GenerateFromRng)
         .password(ef::PasswordType::Text("iloveyou".to_owned(), ef::scrypt_defaults()))
         .encrypt();
         let _ = ef::process(&c).map_err(|e| panic!("error encrypting: {:?}", e));
     }

    println!("{} was encrypted to {}", file_name, encrypted_name);
}

fn decrypt_file(file_name: String, password: String, decrypted_file_name: String){
    println!("Decrypting file: {} with password {}", file_name, password);
    let encrypted_name = file_name + ".ef";
    let decrypted_name = decrypted_file_name + ".txt";
    {
        let mut c = ef::Config::new();
        c.input_stream(ef::InputStream::File(encrypted_name.to_owned()))
         .output_stream(ef::OutputStream::File(decrypted_name.to_owned()))
         .add_output_option(ef::OutputOption::AllowOverwrite)
         .password(ef::PasswordType::Text("iloveyou".to_owned(), ef::PasswordKeyGenMethod::ReadFromFile))
         .decrypt();
        let _ = ef::process(&c).map_err(|e| panic!("error decrypting: {:?}", e));
    }

    println!("{} was decrypted to {}", encrypted_name, decrypted_name);
}

fn main() {
    //println!("Hello, world!");
    //let y = plus_one(5);
    {
        let file_name = "blah.txt".to_string();
        let encrypted_name = "rip".to_string();
        //encrypt_file(file_name, encrypted_name);
    }

    let encrypted_file_name = "rip".to_string();
    let decrypted_file_name = "yay".to_string();
    let password = "iloveyou".to_string();
    {
        decrypt_file(encrypted_file_name, password, decrypted_file_name);
    }

}
