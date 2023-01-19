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
                employees.sort();
            }
            None => {
                self.departments
                    .insert(department.to_string(), vec![name.to_string()]);
            }
        };
    }

    // a method to display employees of a single department
    fn display_department(&self, department: &str) {
        match self.departments.get(department) {
            Some(employees) => {
                println!("{department}' department employees are:");
                println!("{:#?}\n", employees);
            }
            None => {
                println!("{department} is an empty department.\n");
            }
        };
    }

    // a method to display employees of all departments
    fn display_company(&self) {
        println!("{:#?}", self)
    }
}

fn main() {
    let mut rust_componay = Company {
        departments: HashMap::new(),
    };

    rust_componay.add_employee("ADCs", "Caitlen");
    rust_componay.add_employee("ADCs", "Jinx");
    rust_componay.add_employee("ADCs", "MS");
    rust_componay.add_employee("Fighters", "Garen");

    rust_componay.display_department("ADCs");
    rust_componay.display_company();
}
