struct Person {
    name: String,
    department: Box<Department>
}

struct Department {
    name: String,
    manager: Option<Box<Person>>
}

impl Department {
    // Method to set manager
    fn set_manager(&mut self, manager: Box<Person>) {
        self.manager = Some(manager);
    }
}

fn main() {
    // Create mutable Engineering department
    let mut engineering_dept = Box::new(Department {
        name: String::from("Engineering"),
        manager: None
    });

    // Create Alice (who will be Engineering manager)
    let mut alice = Box::new(Person {
        name: String::from("Alice"),
        department: engineering_dept
    });

    // Set Alice as Engineering manager
    // engineering_dept.set_manager(alice);

    println!("No error.");

    // Create John with the engineering department
    // let mut john = Box::new(Person {
    //     name: String::from("John"),
    //     department: engineering_dept
    // });

    // Create Sales department with John as manager
    // let sales_dept = Box::new(Department {
    //     name: String::from("Sales"),
    //     manager: Some(john)
    // });

    // Print the structure
    // println!("Sales Department: {}", sales_dept.name);
    // if let Some(sales_manager) = &sales_dept.manager {
    //     println!("Sales Manager: {}", sales_manager.name);
    //     println!("Engineering Department: {}", sales_manager.department.name);
    //     if let Some(eng_manager) = &sales_manager.department.manager {
    //         println!("Engineering Manager: {}", eng_manager.name);
    //     }
    // }
}