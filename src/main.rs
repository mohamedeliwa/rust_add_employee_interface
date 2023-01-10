use std::collections::HashMap;

#[derive(Debug)]
struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    // a method to add an employee to a department
    fn add_employee(&mut self, department: &str, name: &str) {
        match self.departments.get_mut(department) {
            Some(employees) => {
                employees.push(name.to_string());
            }
            None => {
                self.departments
                    .insert(department.to_string(), vec![name.to_string()]);
            }
        };
    }

    // a method to display employees of a single department
    fn display_department(&self, department: &str) {
        println!("{department}' department employees are:");
        println!("{:#?}\n", self.departments[department]);
    }

    // a method to display employees of all departments
    fn display_company(&self) {
        println!("{:#?}", self)
    }
}

fn main() {
    println!("Hello, world!");
}
