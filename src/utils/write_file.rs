use std::fs;
use std::fs::File;
use std::io::Write;

pub fn write(data: String) {
    match fs::metadata("./todos.json") {
        Ok(_) => {
            fs::write("./todos.json", data).expect("Failed to write file");
        }
        Err(_) => {
            let mut file = File::create("./todos.json").expect("Failed to create file");
            file.write(data.as_bytes()).expect("Failed to write file");
        }
    }
}
