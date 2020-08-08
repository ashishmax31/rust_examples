use crate::employee_terminal::employee_operations;
use std::io;
use std::process;

#[derive(Debug, PartialEq)]
enum Command<'a> {
    AddUser(&'a str, &'a str),
    DeptPeople(&'a str),
    ListPeople,
    Exit,
    Error(&'static str),
}

pub fn start_terminal() {
    let mut db = employee_operations::new_employee_db();
    println!("Welcome to the employee database terminal!");
    println!("Available commands are:");
    println!("1) add <name> to <dept.name>");
    println!("2) show people in <dept.name>");
    println!("3) list all");
    println!("Type 'exit' at any point to exit the terminal");

    loop {
        let user_input = get_user_input();
        match command(&user_input) {
            Ok(prepared_command) => match prepared_command {
                Command::AddUser(name, dept) => add_user(&mut db, name, dept),
                Command::DeptPeople(dept) => show_people_in_dept(&db, dept),
                Command::ListPeople => list_people(&db),
                Command::Exit => process::exit(0),
                Command::Error(msg) => handle_error(msg),
            },
            Err(msg) => handle_error(msg),
        }
    }
}

fn handle_error(msg: &str) {
    eprintln!("{}", msg);
    process::exit(1);
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Expected a line of text");
    input.trim().to_owned()
}

fn command<'a>(input: &'a str) -> Result<Command<'a>, &'static str> {
    let mut words = input.split_whitespace();
    match words.next() {
        Some(first_commmand) => match first_commmand {
            "add" => Ok(build_add_command(words)),
            "show" => Ok(build_show_command(words)),
            "list" => Ok(Command::ListPeople),
            "exit" => Ok(Command::Exit),
            _ => Err("Unsupported action!"),
        },
        None => Err("Expected at least one word input"),
    }
}

fn build_add_command<'a>(words: std::str::SplitWhitespace<'a>) -> Command<'a> {
    let words: Vec<_> = words.collect();
    if words.len() < 3 {
        Command::Error("Incorrect command")
    } else {
        Command::AddUser(words[0], words[2])
    }
}

fn build_show_command<'a>(words: std::str::SplitWhitespace<'a>) -> Command<'a> {
    let words: Vec<_> = words.collect();

    if words.len() < 3 {
        Command::Error("Incorrect command")
    } else {
        Command::DeptPeople(words[2])
    }
}

// Impure fns :(
fn add_user(db: &mut employee_operations::EmployeeDb, name: &str, dept: &str) {
    db.add_employee(name, dept);
}

fn show_people_in_dept(db: &employee_operations::EmployeeDb, dept: &str) {
    let result = db.get_employees_by_dept(dept);
    for name in result {
        println!("{}", name);
    }
}
fn list_people(db: &employee_operations::EmployeeDb) {
    let result = db.list_employees();
    for emp_details in result {
        println!("{}", emp_details);
    }
}

mod test {
    use super::*;
    #[test]
    fn test_build_show_command() {
        let command_1 = "hello world";
        assert_eq!(
            build_show_command(command_1.split_whitespace()),
            Command::Error("Incorrect command")
        );

        let command_2 = "employees in software";
        assert_eq!(
            build_show_command(command_2.split_whitespace()),
            Command::DeptPeople("software")
        );
    }

    #[test]
    fn test_build_add_command() {
        let command_1 = "batman to gotham";
        assert_eq!(
            build_add_command(command_1.split_whitespace()),
            Command::AddUser("batman", "gotham")
        );

        let command_2 = "robin to";
        assert_eq!(
            build_add_command(command_2.split_whitespace()),
            Command::Error("Incorrect command")
        );
    }
    #[test]
    fn test_command() {
        let input_1 = "";
        assert_eq!(command(input_1), Err("Expected at least one word input"));

        let input_2 = "add batman to gotham";
        assert_eq!(command(input_2), Ok(Command::AddUser("batman", "gotham")));

        let input_3 = "list all";
        assert_eq!(command(input_3), Ok(Command::ListPeople));

        let input_4 = "show people in gotham";
        assert_eq!(command(input_4), Ok(Command::DeptPeople("gotham")));

        let input_5 = "exit 123";
        assert_eq!(command(input_5), Ok(Command::Exit));

        let input_6 = "this is an unsupported command";
        assert_eq!(command(input_6), Err("Unsupported action!"));
    }
}
