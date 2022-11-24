use std::{io, collections::HashMap};
use colored::{Colorize, ColoredString};
use clearscreen;

fn create_department(user_commands: Vec<&str>, departments: &mut Vec<String>) -> ColoredString {
    if user_commands.len() != 2 {
        return "Couldn't create department - department name needs to be 1 word".red();
    }
    
    let department_name = String::from(user_commands[1]);

    if departments.contains(&department_name) {
        return format!("Department '{}' already created", user_commands[1]).red();
    }

    departments.push(department_name);

    format!("Department '{}' created", user_commands[1]).green()
}

fn add_employee_to_department(user_commands: Vec<&str>, departments: &Vec<String>, 
                              company: &mut HashMap<String, Vec<String>>) -> ColoredString {
    if user_commands.len() != 4 {
        return "Couldn't add employee to department - employee name and/or department name is not 1 word".red();
    }
    if user_commands[2].to_lowercase() != String::from("to") {
        return format!("Invalid syntax, please do Add [employee name] {} [department name]", "to".underline()).red();
    }
    if !departments.contains(&String::from(user_commands[3])) {
        return format!("Couldn't add {} to {}, '{}' is not a department", user_commands[1], user_commands[3], user_commands[3]).red();
    }

    let employee_name = String::from(user_commands[1]);
    let employee_department_vector = company.entry(employee_name).or_insert(Vec::new());
    employee_department_vector.push(String::from(user_commands[3]));

    format!("Employee {} added to department '{}'", user_commands[1], user_commands[3]).green()
}

fn see_all_employees(departments: &Vec<String>, company: &HashMap<String, Vec<String>>) {
    clearscreen::clear().expect("failed to clear screen");
    for department_name in departments {
        see_department(department_name, departments, company);
        println!("");
    }

    println!("\nPress enter to finish seeing the employees");
    let mut user_input: String = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
}

fn see_department(department_name: &String, departments: &Vec<String>, company: &HashMap<String, Vec<String>>) -> ColoredString {
    if !departments.contains(department_name) {
        return format!("Couldn't see department '{}' as it doesn't exist", department_name).red()
    }
    println!("{}", department_name.underline());
    for (employee, department_vector) in company {
        if department_vector.contains(department_name) {
            println!("{}", employee);
        }
    }

    "".black()
}

fn see(user_commands: Vec<&str>, departments: &Vec<String>, company: &HashMap<String, Vec<String>>) -> ColoredString {
    if user_commands.len() != 2{
        return "Invalid sytax for 'See []', please enter only 1 word".red();
    }
    if user_commands[1] == "employees" {
        see_all_employees(departments, company);
    }
    else {
        clearscreen::clear().expect("failed to clear screen");
        let message = see_department(&String::from(user_commands[1]), departments, company);
        if message !=  "".black() {
            return message;
        }
        println!("\nPress enter to finish seeing {}", user_commands[1]);
        let mut user_input: String = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
    }

    "".black()
}

fn main() {
    // key = employee, value = departments
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    let mut departments: Vec<String> = Vec::new();

    let mut message: ColoredString = "".black();

    loop {
        clearscreen::clear().expect("failed to clear screen");
        println!("\n--- EMPLOYEE TEXT INTERFACE -- \n\n{}\n\nCOMMANDS:\n - Create [department name]\n - Add [employee name] to [department name]\n - See Employees\n - See [department name]\n - Exit \n\nEnter Your Command:", message);

        let mut user_input: String = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        user_input = String::from(user_input.trim()).to_lowercase();

        let mut user_commands: Vec<&str> = Vec::new();
        for command in user_input.split_whitespace() {
            user_commands.push(command);
        }
        
        match user_commands[0] {
            "create" => message = create_department(user_commands, &mut departments),
            "add" => message = add_employee_to_department(user_commands, &departments, &mut company),
            "see" => message = see(user_commands, &departments, &company),
            "exit" => break,
            _ => message = "Invalid input".red(),
        }
    }
}
