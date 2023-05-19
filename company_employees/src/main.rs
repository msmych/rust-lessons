use std::collections::HashMap;
use std::io;

fn main() {
    let company = fill_company();
    query(company);
}

fn fill_company() -> HashMap<String, Vec<String>> {
    println!("Add employees to departments");
    println!("Format: Add <employee> to <department>");
    println!("When finished, type 'done'");
    let mut company = HashMap::new();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        if input == "done" {
            println!("Saved");
            break;
        } else {
            let mut tokens = input.split_whitespace();
            if !matches!(tokens.next(), Some("Add")) {
                continue;
            }
            let employee = match tokens.next() {
                Some(token) => token.to_string(),
                None => continue,
            };
            if !matches!(tokens.next(), Some("to")) {
                continue;
            }
            let department = match tokens.next() {
                Some(token) => token.to_string(),
                None => continue,
            };
            let employees = company.entry(department).or_insert(Vec::new());
            employees.push(employee);
            println!("Added");
        }
    }
    company
}

fn query(company: HashMap<String, Vec<String>>) {
    println!("Enter department name to view its employees");
    println!("Enter 'all' to view all employees by departments, sorted alphabetically");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim() {
            "all" => {
                for (department, employees) in &company {
                    let mut employees = employees.clone();
                    employees.sort();
                    println!("{}: {:?}", department, employees);
                }
            }
            department => {
                if let Some(employees) = company.get(department) {
                    let mut employees = employees.clone();
                    employees.sort();
                    println!("{:?}", employees);
                }
            }
        }
    }
}
