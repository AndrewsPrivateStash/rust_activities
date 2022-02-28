// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

#[derive(Debug)]
enum EmployeeType {
    Maintenance,
    Marketing,
    Managers,
    LineSupervisors,
    KitchenStaff,
    AssemblyTech,
}

struct Employee {
    name: String,
    e_type: EmployeeType,
    is_active: bool,
}

impl Employee {
    fn new(n: &str, t: EmployeeType, a: bool) -> Employee {
        Employee {
            name: n.to_owned(),
            e_type: t,
            is_active: a,
        }
    }

    fn check_active(&self) -> Result<(), String> {
        match self.is_active {
            true => Ok(()),
            _ => Err(format!("{} is not active, no entry!", self.name)),
        }
    }

    fn check_entry(&self) -> Result<(), String> {
        self.check_active()?; // check if employee is active
        match self.e_type {
            EmployeeType::Maintenance => Ok(()),
            EmployeeType::Marketing => Ok(()),
            EmployeeType::Managers => Ok(()),
            _ => Err(format!("{} is a {:?}, no entry!", self.name, self.e_type)),
        }
    }
}

fn main() {
    let emps: Vec<Employee> = vec![
        Employee::new("Andrew", EmployeeType::Managers, true),
        Employee::new("Joshua", EmployeeType::Maintenance, false),
        Employee::new("Kathy", EmployeeType::KitchenStaff, true),
        Employee::new("Tom", EmployeeType::Marketing, true),
        Employee::new("Nanny", EmployeeType::AssemblyTech, false),
        Employee::new("Bob", EmployeeType::LineSupervisors, true),
    ];

    for e in &emps {
        match e.check_entry() {
            Ok(_) => println!("{} is good to enter", e.name),
            Err(s) => println!("{}", s),
        }
    }
}
