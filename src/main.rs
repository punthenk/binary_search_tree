mod task;
mod tree;
mod file_io;
mod ui;

use tree::TaskTree;
use file_io::{load_tasks, insert_into_file};
use ui::{ask_task_input, get_command};

use crate::{file_io::mark_complete_in_file, ui::ask_task_priority};



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
            println!("Found {}, description: {}", result.unwrap().priority(), result.unwrap().description());
        } else {
            println!("Could not find the task");
        }
    } else {
        println!("No tasks found");
    }

    loop {
        let input = get_command();
        
        let input = input.trim();
        if input == "" {
            continue;
        } else if input == "quit" {
            break;
        }

        match input {
            "add" => {
                let values = ask_task_input();
                let priority = values.0;
                let description = values.1;
                if let Some(tree) = &mut task_tree {
                    tree.insert(priority, description.clone());
                    insert_into_file("tasks.txt", priority, description);
                }
            },
            "all" | "display" | "ls" => {
                if let Some(tree) = &task_tree {
                    tree.display_all();
                }
            },
            "check" => {
                let priority = ask_task_priority();
                if let Some(tree) = &mut task_tree {
                    tree.mark_complete(priority);
                    mark_complete_in_file("tasks.txt", priority);
                }
            }
            _ => println!("Dont know?"),
        }
    }
}

