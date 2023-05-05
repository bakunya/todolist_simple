use std::fs;

pub fn read() -> String {
    match fs::metadata("./todos.json") {
        Ok(_) => {
            let todos = fs::read_to_string("./todos.json").expect("Failed to read file");
            todos
        }
        Err(_) => {
            println!("Failed to read file");
            String::from("")
        }
    }
}
