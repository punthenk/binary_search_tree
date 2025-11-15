use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use crate::tree::TaskTree;


pub fn insert_into_file(filename: &str, priority: u32, description: String) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .expect("Failed to open file");

    let line = format!("{}|{}|TODO\n", priority.to_string(), description.trim());

    file.write(line.as_bytes())
        .expect("Unable to write");
}

pub fn load_tasks(filename: &str) -> Option<TaskTree> {
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
