use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("\nEmployee Management System");
        println!("1. Add Employee");
        println!("2. List Employees by Department");
        println!("3. List All Employees (Alphabetical)");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim(); // Remove leading and trailing whitespaces

        match choice.parse::<u8>() {
            Ok(1) => add_employee(&mut company),
            Ok(2) => list_by_department(&company),
            Ok(3) => list_all_employees(&company),
            Ok(4) => break,
            _ => println!("Invalid choice. Please enter a number between 1 and 4."),
        }
    }

    println!("Exiting Employee Management System...");
}

fn add_employee(company: &mut HashMap<String, Vec<String>>) {
    println!("Enter employee name and department (e.g., Sally Engineering):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.pop(); // Remove trailing newline

    let parts = input.splitn(2, ' ').collect::<Vec<_>>();
    if parts.len() != 2 {
        println!("Invalid input format. Please enter name and department separated by a space.");
        return;
    }

    let name = parts[0].to_string();
    let department = parts[1].to_string().to_uppercase(); // Convert department to uppercase

    let department_employees = company.entry(department.clone()).or_insert(Vec::new());
    department_employees.push(name.clone());
    println!("Employee '{}' added to department '{}'.", name, department);
}

fn list_by_department(company: &HashMap<String, Vec<String>>) {
    println!("List of Employees by Department:");
    for (department, employees) in company {
        println!("Department: {}", department);
        let mut sorted_employees = employees.clone();
        sorted_employees.sort();
        for employee in sorted_employees {
            println!("- {}", employee);
        }
    }
}

fn list_all_employees(company: &HashMap<String, Vec<String>>) {
    println!("List of All Employees (Alphabetical):");
    let mut all_employees: Vec<String> = Vec::new();
    for department in company.keys() {
        for employee in company.get(department).unwrap() {
            all_employees.push(format!("{} ({})", employee, department));
        }
    }
    all_employees.sort();
    for employee in all_employees {
        println!("{}", employee);
    }
}

