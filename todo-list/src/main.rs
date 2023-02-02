use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut choice = String::new();

    println!("1) See your awesome todo");
    println!("2) Add new todo");

    io::stdin()
        .read_line(&mut choice)
        .expect("Not able to read your damn choice");

    let choice: i32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid choice");
            1
        },
    };

    if choice == 1 {
    } else {
        add_todo("test todo\n");
    }
}

fn add_todo(todo_item: &str) -> std::io::Result<()> {
    let todo_file = File::open("todo.txt");

    match todo_file {
        Ok(mut file) => {
            // Just read the file & add todo
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            contents.push_str(todo_item);
            file.write_all(contents.as_bytes())?;
        }, 
        Err(_) => {
            // Create file & add todo
            let mut buffer = File::create("todo.txt")?;
            buffer.write_all(todo_item.as_bytes())?;
        },
    };
    Ok(())
}
