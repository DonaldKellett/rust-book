use std::{
    collections::{BTreeMap, BTreeSet},
    io,
};

const OS_SHOULD_READ_INPUT: &str = "The operating system should be able to read user input";
const KEY_IN_MAP: &str = "The specified key should be in the given map";

fn main() {
    println!("Welcome to our company!");
    let stdin = io::stdin();
    let mut departments: BTreeMap<String, BTreeSet<_>> = BTreeMap::new();
    'company: loop {
        println!("Please specify an operation.");
        println!("A: List all employees in all departments");
        println!("D: Add a new department");
        println!("E: Add an employee to an existing department");
        println!("L: List all departments");
        println!("P: List all employees in an existing department");
        println!("Q: Quit");
        let mut operation = String::new();
        stdin.read_line(&mut operation).expect(OS_SHOULD_READ_INPUT);
        match &operation.trim().to_lowercase()[..] {
            "a" => {
                println!();
                println!(
                    "{}",
                    departments
                        .clone()
                        .into_keys()
                        .map(|dept| format!(
                            "{dept}:\n{}",
                            departments
                                .get(&dept)
                                .expect(KEY_IN_MAP)
                                .clone()
                                .into_iter()
                                .collect::<Vec<_>>()
                                .join("\n")
                        ))
                        .collect::<Vec<_>>()
                        .join("\n\n")
                );
                println!();
            }
            "d" => {
                let mut dept = String::new();
                loop {
                    println!("Enter a name for the new department:");
                    stdin.read_line(&mut dept).expect(OS_SHOULD_READ_INPUT);
                    dept = String::from(dept.trim());
                    if dept.len() == 0 {
                        println!("Department name cannot be empty!");
                        dept.clear();
                        continue;
                    } else if departments.contains_key(&dept) {
                        println!("Department name already in use");
                        dept.clear();
                        continue;
                    } else {
                        departments.insert(dept, BTreeSet::new());
                        break;
                    }
                }
            }
            "e" => {
                let mut dept = String::new();
                loop {
                    println!("Enter an existing department name, or provide an empty name to cancel the operation:");
                    stdin.read_line(&mut dept).expect(OS_SHOULD_READ_INPUT);
                    dept = dept.trim().to_string();
                    if dept == "" {
                        continue 'company;
                    }
                    if !departments.contains_key(&dept) {
                        println!("Department does not exist - please provide an existing department name");
                        dept.clear();
                        continue;
                    }
                    break;
                }
                let mut employee = String::new();
                loop {
                    println!("Enter an employee name to add to department {dept}:");
                    stdin.read_line(&mut employee).expect(OS_SHOULD_READ_INPUT);
                    employee = employee.trim().to_string();
                    if employee == "" {
                        println!("Employee name cannot be empty!");
                        employee.clear();
                        continue;
                    }
                    if departments
                        .get(&dept)
                        .expect(KEY_IN_MAP)
                        .contains(&employee)
                    {
                        println!(
                            "Employee already exists in department - please enter another name"
                        );
                        employee.clear();
                        continue;
                    }
                    departments
                        .get_mut(&dept)
                        .expect(KEY_IN_MAP)
                        .insert(employee);
                    break;
                }
            }
            "l" => {
                println!();
                println!(
                    "{}",
                    departments
                        .clone()
                        .into_keys()
                        .collect::<Vec<_>>()
                        .join("\n")
                );
                println!();
            }
            "p" => {
                let mut dept = String::new();
                loop {
                    println!("Enter an existing department name, or an empty name to cancel the operation:");
                    stdin.read_line(&mut dept).expect(OS_SHOULD_READ_INPUT);
                    dept = dept.trim().to_string();
                    if dept == "" {
                        continue 'company;
                    }
                    if !departments.contains_key(&dept) {
                        println!("Department does not exist - please provide an existing department name");
                        dept.clear();
                        continue;
                    }
                    println!();
                    println!(
                        "{}",
                        departments
                            .get(&dept)
                            .expect(KEY_IN_MAP)
                            .clone()
                            .into_iter()
                            .collect::<Vec<_>>()
                            .join("\n")
                    );
                    println!();
                    break;
                }
            }
            "q" => break,
            _ => {
                println!("Invalid operation");
                operation.clear();
                continue;
            }
        }
    }
}
