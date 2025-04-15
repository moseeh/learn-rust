#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        Self { grade: None }
    }
    pub fn add_worker(&mut self, role: String, name: String) {
        let new_worker = Worker {
            role,
            name,
            next: self.grade.take(),
        };
        self.grade = Some(Box::new(new_worker));
    }
    pub fn remove_worker(&mut self) -> Option<String> {
        if let Some(node) = self.grade.take() {
            self.grade = node.next; // Set the head to the next worker in the list.
            Some(node.name)
        } else {
            None
        }
    }
    pub fn last_worker(&self) -> Option<(String, String)> {
        if let Some(ref node) = self.grade {
            Some((node.name.clone(), node.role.clone()))
        } else {
            None
        }
    }
}
