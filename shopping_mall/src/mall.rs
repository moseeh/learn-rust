// Main structure representing a Mall
#[derive(Debug, Clone, PartialEq)]
pub struct Mall {
    // Name of the mall
    pub name: String,
    // List of guards working in the mall
    pub guards: Vec<guard::Guard>,
    // List of floors in the mall
    pub floors: Vec<floor::Floor>,
}

impl Mall {
    // Creates a new Mall instance with the provided name, guards, and floors
    #[allow(dead_code)]
    pub fn new(name: &str, guards: Vec<guard::Guard>, floors: Vec<floor::Floor>) -> Mall {
        Mall {
            name: name.to_string(),
            guards: guards,
            floors: floors,
        }
    }

    // Changes the name of the mall
    #[allow(dead_code)]
    pub fn change_name(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }

    // Hires a new guard by adding them to the guards list
    #[allow(dead_code)]
    pub fn hire_guard(&mut self, guard: guard::Guard) {
        self.guards.push(guard);
    }
}

// Module representing a security guard
pub mod guard {

    // Structure representing a Guard
    #[derive(Debug, Clone, PartialEq)]
    pub struct Guard {
        // Name of the guard
        pub name: String,
        // Age of the guard
        pub age: u8,
        // Years of experience of the guard
        pub years_experience: u8,
    }

    impl Guard {
        // Creates a new Guard instance
        #[allow(dead_code)]
        pub fn new(name: &str, age: u8, years_experience: u8) -> Guard {
            Guard {
                name: name.to_string(),
                age: age,
                years_experience: years_experience,
            }
        }
    }
}

// Module representing a floor in the mall
pub mod floor {

    // Structure representing a Floor
    #[derive(Debug, Clone, PartialEq)]
    pub struct Floor {
        // Name of the floor
        pub name: String,
        // List of stores on the floor
        pub stores: Vec<store::Store>,
        // Maximum allowed size for the floor
        pub size_limit: u64,
    }

    impl Floor {
        // Creates a new Floor instance
        #[allow(dead_code)]
        pub fn new(name: &str, stores: Vec<store::Store>, store_limit: u64) -> Floor {
            Floor {
                name: name.to_string(),
                stores: stores,
                size_limit: store_limit,
            }
        }

        // Replaces an existing store with a new one
        #[allow(dead_code)]
        pub fn change_store(&mut self, store: &str, new_store: store::Store) {
            let pos = self.stores.iter().position(|x| x.name == store).unwrap();
            self.stores[pos] = new_store;
        }

        // Adds a new store if size limit is not exceeded
        #[allow(dead_code)]
        pub fn add_store(&mut self, new_store: store::Store) {
            let mut current_floor_size = 0;

            for store in self.stores.iter() {
                current_floor_size += store.square_meters;
            }

            if self.size_limit < current_floor_size + new_store.square_meters {
                self.stores.push(new_store);
            }
        }

        // Removes a store by its name
        #[allow(dead_code)]
        pub fn remove_store(&mut self, store_name: String) {
            self.stores.retain(|x| x.name != store_name);
        }
    }

    // Module representing a store on a floor
    pub mod store {

        // Structure representing a Store
        #[derive(Debug, Clone, PartialEq)]
        pub struct Store {
            // Name of the store
            pub name: String,
            // Size of the store in square meters
            pub square_meters: u64,
            // List of employees working in the store
            pub employees: Vec<employee::Employee>,
        }

        impl Store {
            // Creates a new Store instance
            #[allow(dead_code)]
            pub fn new(name: &str, space: u64, employees: Vec<employee::Employee>) -> Store {
                Store {
                    name: name.to_string(),
                    square_meters: space,
                    employees: employees,
                }
            }

            // Adds a new employee to the store
            #[allow(dead_code)]
            pub fn hire_employee(&mut self, employee: employee::Employee) {
                self.employees.push(employee);
            }

            // Removes an employee by their name
            #[allow(dead_code)]
            pub fn fire_employee(&mut self, employee_name: &str) {
                self.employees.retain(|x| x.name != employee_name);
            }

            // Expands the store size by the given square meters
            #[allow(dead_code)]
            pub fn expand(&mut self, square_meters: u64) {
                self.square_meters += square_meters;
            }
        }

        // Module representing an employee working in a store
        pub mod employee {

            // Structure representing an Employee
            #[derive(Debug, Clone, PartialEq)]
            pub struct Employee {
                // Name of the employee
                pub name: String,
                // Age of the employee
                pub age: u8,
                // Tuple representing working hours (entry, exit)
                pub working_hours: (u8, u8),
                // Salary of the employee
                pub salary: f64,
            }

            impl Employee {
                // Creates a new Employee instance
                #[allow(dead_code)]
                pub fn new(
                    name: &str,
                    age: u8,
                    entry_hour: u8,
                    exit_hour: u8,
                    salary: f64,
                ) -> Employee {
                    Employee {
                        name: name.to_string(),
                        age: age,
                        working_hours: (entry_hour, exit_hour),
                        salary: salary,
                    }
                }

                // Increments the employee's age by 1
                #[allow(dead_code)]
                pub fn birthday(&mut self) {
                    self.age += 1;
                }

                // Changes the employee's working hours
                #[allow(dead_code)]
                pub fn change_workload(&mut self, entry_hour: u8, exit_hour: u8) {
                    self.working_hours = (entry_hour, exit_hour);
                }

                // Increases the employee's salary by a given amount
                #[allow(dead_code)]
                pub fn raise(&mut self, amount: f64) {
                    self.salary += amount;
                }

                // Decreases the employee's salary by a given amount
                #[allow(dead_code)]
                pub fn cut(&mut self, amount: f64) {
                    self.salary = self.salary - amount;
                }
            }
        }
    }
}