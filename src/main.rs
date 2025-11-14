use std::fs;

#[derive(Debug, Clone)]
struct Task {
    priority: u32,
    description: String,
    completed: bool,
}

#[derive(Debug)]
struct TaskTree {
    task: Task,
    left: Option<Box<TaskTree>>,
    right: Option<Box<TaskTree>>,
}

impl TaskTree {
    fn new(priority: u32, description: String) -> Self {
        TaskTree {
            task: Task { 
                priority,
                description,
                completed: false,
            },
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, priority: u32, description: String) {
        if priority < self.task.priority {
            match &mut self.left {
                None => self.left = Some(Box::new(TaskTree::new(priority, description))),
                Some(node) => node.insert(priority, description),
            }
        } else {
            match &mut self.right {
                None => self.right = Some(Box::new(TaskTree::new(priority, description))),
                Some(node) => node.insert(priority, description),
            }
        }
    }

    fn mark_complete(&mut self, priority: u32,) -> bool {
        if priority == self.task.priority {
            self.task.completed = true;
            return true;
        } else if priority < self.task.priority {
            if let Some(node) = &mut self.left {
                return node.mark_complete(priority);
            }

        } else {
            if let Some(node) = &mut self.right {
                return node.mark_complete(priority)
            }
        }
        false
    }

    fn find_task(&self, priority: u32) -> Option<&Task> {
        if priority == self.task.priority {
            Some(&self.task)
        } else if priority < self.task.priority {
            self.left.as_ref()?.find_task(priority)
        } else {
            self.right.as_ref()?.find_task(priority)
        }
    }

    fn display_all(&self) {
        if let Some(right) = &self.right {
            right.display_all();
        }

        let status = if self.task.completed { "✓" } else { " " };
        println!("[{}] Priority {}: {}", status, self.task.priority, self.task.description);

        if let Some(left) = &self.left {
            left.display_all();
        }
    }
}

fn load_tasks(filename: &str) -> Option<TaskTree> {
    if let Ok(content) = fs::read_to_string(filename) {
        let mut tree: Option<TaskTree> = None;

        for line in content.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 3 {
                if let Ok(priority) = parts[0].parse::<u32>() {
                    let description = parts[1].to_string();
                    let completed = parts[2] == "DONE";

                    match &mut tree {
                        None => tree = Some(TaskTree::new(priority, description)),
                        Some(node) => node.insert(priority, description),
                    }

                    if completed {
                        if let Some(node) = &mut tree {
                            node.mark_complete(priority);
                        }
                    }
                }
            }
        }
        println!("✓ Tasks loaded from {}", filename);
        return tree;
    }
    None
}

fn main() {
    let mut task_tree: Option<TaskTree> = load_tasks("tasks.txt");
    
    if let Some(tree) = &task_tree {
        tree.display_all();
    } else {
        println!("No tasks found");
    }

    if let Some(tree) = &task_tree {
        let result = tree.find_task(202);
        if result.is_some() {
            println!("Found {}, description: {}", result.unwrap().priority, result.unwrap().description);
        } else {
            println!("Could not find the task");
        }
    } else {
        println!("No tasks found");
    }
}
