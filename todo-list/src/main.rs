use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    loop {
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
            view_todo().expect("Failed to read file");
        } else if choice == 2 {
            let mut todo_item = String::new();
            
            println!("Enter your todo item: ");

            io::stdin()
                .read_line(&mut todo_item)
                .expect("Not able to read your damn choice");

            add_todo(todo_item.as_str()).expect("Not able to add todo");
        } else {
            break;
        }
    }
}

fn add_todo(todo_item: &str) -> std::io::Result<()> {
    let todo_file = File::options().append(true).open("todo.txt");

    match todo_file {
        Ok(mut file) => {
            file.write_all(todo_item.as_bytes())?;
        }, 
        Err(_) => {
            let mut buffer = File::create("todo.txt")?;
            buffer.write_all(todo_item.as_bytes())?;
        },
    };
    Ok(())
}

fn view_todo() -> std::io::Result<()> {
    let buffer = File::open("todo.txt");

    match buffer {
        Ok(mut buff) => {
            let mut contents = String::new();
            buff.read_to_string(&mut contents)?;
            println!("{contents}");
        }, 
        Err(_) => {
            println!("No todo found...");
        },

    };

    Ok(())
}
