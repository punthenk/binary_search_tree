mod task;
mod tree;
mod file_io;
mod ui;

use tree::TaskTree;
use file_io::{load_tasks, insert_into_file};
use ui::{ask_task_input, get_command};

use crate::{file_io::{delete_from_file, mark_complete_in_file, mark_uncomplete_in_file}, ui::ask_task_priority};

fn handle_command(input: &str, task_tree: &mut Option<TaskTree>) -> bool {
    match input {
        "quit" | "exit" => return true,
        "add" => {
            let values = ask_task_input();
            let priority = values.0;
            let description = values.1;
            if let Some(tree) = task_tree {
                tree.insert(priority, description.clone());
                insert_into_file("tasks.txt", priority, description);
            }
        },
        "find" => {
            let priority = ask_task_priority();
            if let Some(tree) = &task_tree {
                if let Some(task) = tree.find_task(priority) {
                    task.display();
                } else {
                    println!("Task not found");
                }
            }
        },
        "rm" | "remove" => {
            let priority = ask_task_priority();
            if let Some(tree) = task_tree.take() {
                *task_tree = Box::new(tree).delete(priority).map(|boxed| *boxed);
                let result = delete_from_file("tasks.txt", priority);
            }
        },
        "all" | "display" | "ls" => {
            if let Some(tree) = &*task_tree {
                tree.display_all();
            }
        },
        "check" => {
            let priority = ask_task_priority();
            if let Some(tree) = task_tree {
                tree.mark_complete(priority);
                mark_complete_in_file("tasks.txt", priority);
            }
        },
        "uncheck" => {
            let priority = ask_task_priority();
            if let Some(tree) = task_tree {
                tree.mark_uncomplete(priority);
                mark_uncomplete_in_file("tasks.txt", priority);
            }
        }
        "tree" | "htree" => {
            if let Some(tree) = task_tree {
                tree.display_tree_horizontal(0, false);
            }
        },
        "vtree" => {
            if let Some(tree) = &*task_tree {
                tree.display_tree_vertical();
            }
        },
        _ => println!("Unknown Command?"),
    }
    false
}

fn live_tree_mode(task_tree: &mut Option<TaskTree>) {
    loop {
        println!("\x1B[2J\x1B[1;1H");
        if let Some(tree) = &task_tree {
            tree.display_tree_horizontal(0, false);
        }

        let input = get_command("live >");
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        if handle_command(input, task_tree) {
            break;
        }

    }
}

fn main() {
    let mut task_tree: Option<TaskTree> = load_tasks("tasks.txt");
    
    if let Some(tree) = &task_tree {
        tree.display_all();
    } else {
        println!("No tasks found");
    }

    loop {
        let input = get_command(">");
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "live" {
            live_tree_mode(&mut task_tree);
            continue;
        }

        if handle_command(input, &mut task_tree) {
            break;
        }
    }
}

