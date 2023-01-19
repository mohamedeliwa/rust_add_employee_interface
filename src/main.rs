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

    // a method that excutes an addition command
    // to add an employee to a department
    // command structure
    // add [employee name] to [department name]
    fn execute(&mut self, command: &str) {
        let command_components: Vec<&str> = command.split(' ').collect();
        let department = command_components[3];
        let employee = command_components[1];
        self.add_employee(department, employee);
        println!("{} added to {}", employee, department);
    }
}

fn main() {
    let mut rust_componay = Company {
        departments: HashMap::new(),
    };

    rust_componay.execute("Add MS to ADCs");
    rust_componay.execute("Add Caitlen to ADCs");
    rust_componay.execute("Add Jinx to ADCs");
    rust_componay.execute("Add Garen to Fighters");

    rust_componay.display_department("ADCs");
    rust_componay.display_department("Fighters");
    rust_componay.display_company();
}
