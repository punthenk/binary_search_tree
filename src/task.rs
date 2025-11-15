#[derive(Debug, Clone)]
pub struct Task {
    priority: u32,
    description: String,
    completed: bool,
}

impl Task {
    pub fn new(priority: u32, description: String) -> Self {
        Task {
            priority,
            description,
            completed: false,
        }
    }

    pub fn priority(&self) -> u32 {
        self.priority
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }
    
    pub fn set_completed(&mut self) {
        self.completed = true;
    }
}
