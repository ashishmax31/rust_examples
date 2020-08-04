use crate::employee_terminal::employee_operations;
use std::io;
use std::process;

enum Command {
  AddUser(String, String),
  DeptPeople(String),
  ListPeople,
  Exit,
  Error
}

pub fn start_terminal(){
  let mut db = employee_operations::new_employee_db();
  println!("Welcome to the employee database terminal!");
  println!("Available commands are:");
  println!("1) add <name> to <dept.name>");
  println!("2) show people in <dept.name>");
  println!("3) list all");
  println!("Type 'exit' at any point to exit the terminal");

  loop {
    println!();
    let user_input = get_user_input();
    match command(user_input){
      Command::AddUser(name, dept) => add_user(&mut db, name, dept),
      Command::DeptPeople(dept) => show_people_in_dept(&mut db, dept),
      Command::ListPeople => list_people(&mut db),
      Command::Error => panic!("Unknown action"),
      Command::Exit => process::exit(0),
    }
  }
}


fn get_user_input() -> String{
  let mut input = String::new();
  io::stdin()
      .read_line(&mut input)
      .expect("Expected a line of text");
  String::from(input.trim())
}

fn command(input: String) -> Command {
  let words: Vec<&str> = input.split_whitespace().collect();
  match words[0]{
    "add"  => build_add_command(words),
    "show" => build_show_command(words),
    "list" => Command::ListPeople,
    "exit" => Command::Exit,
    _ => Command::Error,
  }
}

fn build_add_command(words: Vec<&str>) -> Command {
  if words.len() == 4 {
    Command::AddUser(
      String::from(words[1]),
      String::from(words[3]),
    )
  }else {
    Command::Error
  }
}

fn build_show_command(words: Vec<&str>) -> Command {
  if words.len() == 4 {
    Command::DeptPeople(
      String::from(words[3]),
    )
  }else {
    Command::Error
  }
}

fn add_user(db: &mut employee_operations::EmployeeDb, name: String, dept: String){
  db.add_employee(&name[..], &dept[..]);
}

fn show_people_in_dept(db: &mut employee_operations::EmployeeDb, dept: String) {
  let result = db.get_employees_by_dept(&dept[..]);
  for name in result {
    println!("{}", name);
  }
}
fn list_people(db: &mut employee_operations::EmployeeDb){
  let result = db.list_employees();
  for emp_details in result {
    println!("{}", emp_details);
  }
}
