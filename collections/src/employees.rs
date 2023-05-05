use std::collections::HashMap;

// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

pub struct Company {
    employees_by_dep: HashMap<String, Vec<String>>,
}

impl Company {
    pub fn new() -> Company {
        let employees = HashMap::new();

        return Company { employees_by_dep: employees };
    }

    pub fn execute_command(&mut self, command: &str) {
        let mut words: Vec<&str> = command.split_whitespace().collect();
        
        if words.is_empty() {
            println!("Please type a command!");
            return;
        }

        let command = words.remove(0).to_ascii_lowercase();

        match command.as_str() {
            "add" => self.add_employee(words),
            "list" => self.list_employee(words),
            s => println!("{} is not a valid command!", s),
        }
    }

    fn add_employee(&mut self, command: Vec<&str>) {
        let mut name = Vec::new();
        let mut dep = Vec::new();

        let mut found_to = false;

        for word in command {
            if word.to_ascii_lowercase() == "to" {
                found_to = true;
                continue;
            }
            if found_to {
                dep.push(word);
            } else {
                name.push(word);
            }
        }

        let name = name.join(" ");
        let dep = dep.join(" ");

        if name.is_empty() || dep.is_empty() {
            println!("Incorrect command usage! Please follow the pattern of: Add Sally to Engineering");
            return;
        }

        if dep.to_ascii_lowercase() == "all" {
            println!("'All' is not a valid department");
            return;
        }

        self.employees_by_dep.entry(dep).or_insert(Vec::new()).push(name);
    }

    fn list_employee(&self, command: Vec<&str>) {
        let dep: String = command.join(" ");

        if dep.to_ascii_lowercase() == "all" {
            println!("All employees:");
            for emp in self.get_all_employees() {
                println!("{}", emp);
            }
            return;
        }

        match self.employees_by_dep.get(&dep) {
            Some(employees) => {
                println!("Employees in the {} department:", dep);

                let mut employees = employees.clone();
                employees.sort();

                for emp in employees {
                    println!("{}", emp);
                }
            },
            None => println!("There are no employees in the {} department", dep),
        }
    }

    fn get_all_employees(&self) -> Vec<String> {
        let mut all_employees = Vec::new();

        for (dep, employees) in self.employees_by_dep.iter() {
            for emp in employees {
                all_employees.push(format!("{} of {}", emp, dep));
            }
        }

        all_employees.sort();
        return all_employees;
    }
}
