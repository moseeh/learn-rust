// Publicly expose the mall module
pub mod mall;

// Re-export items for easier external access
pub use floor::store;
pub use mall::floor;
pub use mall::guard;
pub use mall::*;
pub use store::employee;

// Find and return the largest store in the mall by square meters
pub fn biggest_store(mall: mall::Mall) -> store::Store {
    let mut res: store::Store = store::Store::new("", 0, vec![]);

    // Iterate through all floors and stores
    for floor_a in mall.floors.iter() {
        for shop in floor_a.stores.iter() {
            // Keep track of the store with the most square meters
            if shop.square_meters > res.square_meters {
                res = shop.clone();
            }
        }
    }

    res
}

// Find and return all employees with the highest salary in the mall
pub fn highest_paid_employee(mall: mall::Mall) -> Vec<employee::Employee> {
    let mut res = vec![employee::Employee::new("", 0, 0, 0, 0.0)];

    // Iterate through all employees in all stores on all floors
    for elem in mall.floors.iter() {
        for shop in elem.stores.iter() {
            for emp in shop.employees.clone().into_iter() {
                // If a higher salary is found, reset the result list
                if emp.salary > res[0].salary {
                    res[0] = emp.clone();
                // If salary matches current max, add to result list
                } else if emp.salary == res[0].salary {
                    res.push(emp.clone());
                }
            }
        }
    }

    res
}

// Return the total number of employees including guards
pub fn nbr_of_employees(mall: mall::Mall) -> usize {
    let mut res = 0;

    // Sum up employees from each store on every floor
    for floor_a in mall.floors.iter() {
        for shop in floor_a.stores.iter() {
            res += shop.employees.len();
        }
    }

    // Include guards as employees
    res + mall.guards.len()
}

// Hire additional guards if current security is insufficient
pub fn check_for_securities(mall: &mut mall::Mall, available_sec: Vec<guard::Guard>) {
    let mut size = 0;

    // Calculate the total floor size limit
    for floor in mall.floors.iter() {
        size += floor.size_limit;
    }

    let mut i = 0;
    // Hire guards until the number of guards meets the required ratio (1 guard per 200 sq meters)
    while (mall.guards.len() as f64) < size as f64 / 200.0 {
        mall.hire_guard(available_sec[i].clone());
        i += 1;
    }
}

// Give raise or cut to employees based on their working hours
pub fn cut_or_raise(mall: &mut mall::Mall) {
    // Clone the mall to iterate safely while mutating the original
    for (i, elem) in mall.clone().floors.iter().enumerate() {
        for (j, shop) in elem.stores.iter().enumerate() {
            for (z, emp) in shop.employees.iter().enumerate() {
                // Raise salary if employee works 10 or more hours
                if emp.working_hours.1 - emp.working_hours.0 >= 10 {
                    mall.floors[i].stores[j].employees[z].raise(emp.salary * 0.1);
                // Otherwise, cut the salary
                } else {
                    mall.floors[i].stores[j].employees[z].cut(emp.salary * 0.1);
                }
            }
        }
    }
}