extern crate rusto;

use rusto::basic_symm;

fn main() {
    {
        let file_name = "blah.txt".to_string();
        let encrypted_name = "rip".to_string();
        let file_password = "iloveyou".to_string();
        basic_symm::encrypt_file(file_name, encrypted_name, file_password);
    }

    {
        let encrypted_file_name = "rip".to_string();
        let decrypted_file_name = "yay".to_string();
        let password = "iloveyou".to_string();

        basic_symm::decrypt_file(encrypted_file_name, password, decrypted_file_name);
    }

}
