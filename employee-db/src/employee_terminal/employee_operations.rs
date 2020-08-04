use std::collections::HashMap;

pub struct EmployeeDb {
  db: HashMap<String, String>,
}


pub fn new_employee_db() -> EmployeeDb {
  EmployeeDb{
    db: HashMap::new()
  }
}


impl EmployeeDb{
  pub fn add_employee(&mut self, name: &str, dept: &str) -> Option<String> {
    self.db.insert(name.to_string(), dept.to_string())
  }

  pub fn get_employees_by_dept(&self, dept: &str) -> Vec<&String>{
    self.db.iter().filter_map(|(k, v)|{
      if v == dept {
        Some(k)
      }else {
        None
      }
    }).collect()
  }

  pub fn list_employees(&self) -> Vec<String> {
    let depts = get_sorted_dept_names(self);
    depts.iter().flat_map(|dept| {
      build_employee_listing(get_sorted_employees_by_dept(self, dept), dept)
    }).collect()
  }
}

fn get_sorted_dept_names(db_obj: &EmployeeDb) -> Vec<&String> {
  let mut values: Vec<&String> = db_obj.db.values().collect();
  values.sort();
  values.dedup();
  values
}

fn get_sorted_employees_by_dept(db_obj: &EmployeeDb, dept: &str) -> Vec<String> {
  let mut employees: Vec<String> = db_obj.get_employees_by_dept(dept)
                                                        .iter()
                                                        .map(|x| String::from(*x))
                                                        .collect();
  employees.sort();
  employees
}

fn build_employee_listing(employees: Vec<String>, dept: &str) -> Vec<String> {
  employees.iter().map(|employee|{
    format!("Name: {} Department: {}", employee, dept)
  }).collect()
}

