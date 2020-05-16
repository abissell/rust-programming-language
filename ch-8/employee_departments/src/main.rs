use std::io;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut names = HashMap::<String, Vec<String>>::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().to_lowercase().as_str() {
            "exit" => break,
            "show" => {
                for (department, dept_names) in &names {
                    print!("{}: ", department);
                    for name in dept_names {
                        print!("{} ", name);
                    }
                    println!("");
                }
            }
            &_ => {
                let (command, rest) = input.split_at(4);
                assert_eq!("Add ", command);
                let mut iter = rest.split(" to ");
                let name = iter.next().expect("Could not get a name from command").trim().to_string();
                let department = iter.next().expect("Could not get department from command").trim().to_string();
                let dept_names = names.entry(department).or_insert(Vec::new());
                dept_names.push(name);
                dept_names.sort();
            }
        }
    }
}
