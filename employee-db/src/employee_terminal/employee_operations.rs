use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug, PartialEq)]
pub struct EmployeeDb {
    db: HashMap<String, Vec<String>>,
}

pub fn new_employee_db() -> EmployeeDb {
    EmployeeDb { db: HashMap::new() }
}

impl EmployeeDb {
    pub fn add_employee(&mut self, name: &str, dept: &str) {
        match self.db.get_mut(dept) {
            Some(employees) => {
                employees.push(name.to_string());
                employees.sort();
            }
            None => {
                self.db.insert(dept.to_string(), vec![name.to_string()]);
            }
        }
    }

    pub fn get_employees_by_dept(&self, dept: &str) -> Vec<&String> {
        match self.db.get(dept) {
            Some(items) => items.iter().collect(),
            None => Vec::new(),
        }
    }

    pub fn list_employees(&self) -> Vec<String> {
        self.db
            .iter()
            .flat_map(|(dept, employees)| build_employee_listing(employees, dept))
            .collect()
    }
}

fn build_employee_listing(employees: &Vec<String>, dept: &str) -> Vec<String> {
    employees
        .iter()
        .map(|employee| format!("Name: {} Department: {}", employee, dept))
        .collect()
}

mod testing {
    use super::*;

    #[test]
    fn test_add_employee() {
        let mut test_db = new_employee_db();
        test_db.add_employee("batman", "gotham");
        test_db.add_employee("superman", "daily planet");
        test_db.add_employee("robin", "gotham");

        let mut stored_depts: Vec<&String> = test_db.db.keys().collect();
        stored_depts.sort();
        assert_eq!(stored_depts, ["daily planet", "gotham"]);

        let mut stored_employees: Vec<&String> = test_db.db.values().flatten().collect();
        stored_employees.sort();
        assert_eq!(stored_employees, ["batman", "robin", "superman"]);
    }

    #[test]
    fn test_get_employees_by_dept() {
        let mut test_db = new_employee_db();
        test_db.add_employee("robin", "gotham");
        test_db.add_employee("superman", "daily planet");
        test_db.add_employee("batman", "gotham");
        test_db.add_employee("Lois lane", "daily planet");

        let res_1 = test_db.get_employees_by_dept("gotham");
        // Assert names are stored in the sorted order.
        assert_eq!(res_1, ["batman", "robin"]);

        let res_2 = test_db.get_employees_by_dept("daily planet");
        assert_eq!(res_2, ["Lois lane", "superman"]);

        assert_eq!(
            test_db.get_employees_by_dept("Area 51"),
            Vec::<&String>::new()
        );
    }

    #[test]
    fn test_list_employees() {
        let mut test_db = new_employee_db();
        test_db.add_employee("robin", "gotham");
        test_db.add_employee("superman", "daily planet");
        test_db.add_employee("batman", "gotham");
        test_db.add_employee("Lois lane", "daily planet");

        let mut res = test_db.list_employees();
        res.sort();

        let mut expected_result = [
            "Name: Lois lane Department: daily planet",
            "Name: superman Department: daily planet",
            "Name: batman Department: gotham",
            "Name: robin Department: gotham",
        ];
        expected_result.sort();
        assert_eq!(res, expected_result);
    }
}
