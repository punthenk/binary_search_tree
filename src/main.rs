use core::num;
use std::array;
use std::fs;
use std::io;
use std::io::Write;

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

fn add_task() -> (u32, String) {
    let mut description = String::new();
    print!("name: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut description)
        .expect("Not found");
    
    let description = description.trim().to_string();

    let mut priority = String::new();
    print!("priority: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut priority)
        .expect("Not found");

    println!("\ndescription: {}", description);
    println!("priority: {}", priority);

    let priority_u32: u32 = loop {
        match priority.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Not a number. Try again");
                print!("priority: ");
                io::stdout().flush().unwrap();
                let mut new_input = String::new();
                io::stdin().read_line(&mut new_input).unwrap();
                priority = new_input;
            }
        }
    };

    return (priority_u32, description);
}

fn main() {
    let mut task_tree: Option<TaskTree> = load_tasks("tasks.txt");
    
    if let Some(tree) = &task_tree {
        tree.display_all();
    } else {
        println!("No tasks found");
    }

    if let Some(tree) = &task_tree {
        let result = tree.find_task(108);
        if result.is_some() {
            println!("Found {}, description: {}", result.unwrap().priority, result.unwrap().description);
        } else {
            println!("Could not find the task");
        }
    } else {
        println!("No tasks found");
    }

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let input = input.trim();
        if input == "" {
            continue;
        } else if input == "quit" {
            break;
        }

        match input {
            "add" => {
                let values = add_task();
                let priority = values.0;
                let description = values.1;
                if let Some(tree) = &mut task_tree {
                    tree.insert(priority, description);
                }
            },
            "all" | "display" | "ls" => {
                if let Some(tree) = &task_tree {
                    tree.display_all();
                }
            }
            _ => println!("Dont know?"),
        }
    }
}
