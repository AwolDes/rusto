extern crate rusto;

use rusto::basic_symm;
use rusto::directory_reading;
use std::path::Path;

fn main() {
    let run_encrypt = false;
    if run_encrypt {
        {
            let file_name = "blah.txt".to_string();
            let encrypted_name = "rip".to_string();
            let file_password = "iloveyou".to_string();
            basic_symm::encrypt_file(file_name, encrypted_name, file_password);
        }

        {
            let encrypted_file_name = "rip".to_string();
            let decrypted_file_name = "blah".to_string();
            let password = "iloveyou".to_string();

            basic_symm::decrypt_file(encrypted_file_name, password, decrypted_file_name);
        }
    }
    let path = ".";
    //directory_reading::read_directory(path.to_string());

    let real_path = Path::new("./encrypt_me");
    
    directory_reading::visit_dirs(real_path);

}
