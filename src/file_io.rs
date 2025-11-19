use std::fs::{self, File, OpenOptions};
use std::io::{Write};
use crate::tree::{TaskTree, build_balanced_tree};


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

pub fn delete_from_file(filename: &str, priority: u32) -> std::io::Result<()> {
    let contents = fs::read_to_string(filename)?;
    let priority_str = priority.to_string();

    let filtered: String = contents
        .lines()
        .filter(|line| !line.contains(&priority_str))
        .map(|line| format!("{}\n", line))
        .collect();

    fs::write(filename, filtered)?;
    Ok(())
}

pub fn mark_complete_in_file(filename: &str, priority: u32) -> bool {
    if let Ok(content) = fs::read_to_string(filename) {
        let mut modified = false;
        let mut new_lines = Vec::new();
        for line in content.lines() {
            let parts: Vec<&str> = line.split('|').collect();

            if parts.len() == 3 {
                if let Ok(line_priority) = parts[0].parse::<u32>() {
                    if line_priority == priority && parts[2] == "TODO" {
                        let new_line = format!("{}|{}|DONE", parts[0], parts[1]);
                        new_lines.push(new_line);
                        println!("Task with priority {} is marked completed", priority);
                        modified = true;
                        continue;
                    } else if line_priority == priority && parts[2] == "DONE" {
                        println!("Task is already done");
                        return false;
                    }
                }
            }
            new_lines.push(line.to_string());
        }
        if modified {
            let new_content = new_lines.join("\n") + "\n";
            fs::write(filename, new_content).expect("Failed to write to file");
            return true;
        }

    }
    false
}

pub fn mark_uncomplete_in_file(filename: &str, priority: u32) -> bool {
    if let Ok(content) = fs::read_to_string(filename) {
        let mut modified = false;
        let mut new_lines = Vec::new();
        for line in content.lines() {
            let parts: Vec<&str> = line.split('|').collect();

            if parts.len() == 3 {
                if let Ok(line_priority) = parts[0].parse::<u32>() {
                    if line_priority == priority && parts[2] == "DONE" {
                        let new_line = format!("{}|{}|TODO", parts[0], parts[1]);
                        new_lines.push(new_line);
                        println!("Task with priority {} is marked uncompleted", priority);
                        modified = true;
                        continue;
                    } else if line_priority == priority && parts[2] == "TODO" {
                        println!("Task is not done");
                        return false;
                    }
                }
            }
            new_lines.push(line.to_string());
        }
        if modified {
            let new_content = new_lines.join("\n") + "\n";
            fs::write(filename, new_content).expect("Failed to write to file");
            return true;
        }

    }
    false
}

pub fn load_tasks(filename: &str) -> Option<TaskTree> {
    if let Ok(content) = fs::read_to_string(filename) {
        let mut tasks = Vec::new();

        for line in content.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 3 {
                if let Ok(priority) = parts[0].parse::<u32>() {
                    let description = parts[1].to_string();
                    let completed = parts[2] == "DONE";
                    tasks.push((priority, description, completed));
                }
            }
        }

        tasks.sort_by_key(|task| task.0);
        let tree = build_balanced_tree(&tasks);

        println!("✓ Tasks loaded from {}", filename);
        return tree;
    }
    None
}
