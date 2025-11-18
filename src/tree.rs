use std::usize;

use crate::task::Task;

pub struct TaskTree {
    task: Task,
    left: Option<Box<TaskTree>>,
    right: Option<Box<TaskTree>>,
}

impl TaskTree {
    pub fn new(priority: u32, description: String) -> Self {
        TaskTree {
            task: Task::new(priority, description),
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, priority: u32, description: String) {
        if priority == self.task.priority() {
            println!("Found the same priority");
            return;
        }

        let description = description.trim().to_string();

        if priority < self.task.priority() {
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

    // HOLY MOLY I am littely working on this the entire day and now it finally works a little!!!
    // This was by far the hardest thing I did in this project so far
    pub fn delete(mut self: Box<Self>, priority: u32) -> Option<Box<TaskTree>> {
        if priority < self.task.priority() {
            self.left = self.left.and_then(|left| left.delete(priority));
            Some(self)
        } else if priority > self.task.priority() {
            self.right = self.right.and_then(|right| right.delete(priority));
            Some(self)
        } else {
            if self.left.is_none() && self.right.is_none() {
                return None;
            }
            Some(self)
        }
    }

    pub fn mark_complete(&mut self, priority: u32) -> bool {
        if priority == self.task.priority() {
            self.task.set_completed();
            return true;
        } else if priority < self.task.priority() {
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

    pub fn mark_uncomplete(&mut self, priority: u32) -> bool {
        if priority == self.task.priority() {
            self.task.set_uncompleted();
            return true;
        } else if priority < self.task.priority() {
            if let Some(node) = &mut self.left {
                return node.mark_uncomplete(priority)
            }
        } else {
            if let Some(node) = &mut self.right {
                return node.mark_uncomplete(priority)
            }
        }
        false
    }

    pub fn find_task(&self, priority: u32) -> Option<&Task> {
        if priority == self.task.priority() {
            Some(&self.task)
        } else if priority < self.task.priority() {
            self.left.as_ref()?.find_task(priority)
        } else {
            self.right.as_ref()?.find_task(priority)
        }
    }

    pub fn display_all(&self) {
        if let Some(right) = &self.right {
            right.display_all();
        }

        let status = if self.task.is_completed() { "✓" } else { " " };
        println!("[{}] Priority {}: {}", status, self.task.priority(), self.task.description());

        if let Some(left) = &self.left {
            left.display_all();
        }
    }

    pub fn display_tree_horizontal(&self, indent: usize, is_right: bool) {
        if let Some(right) = &self.right {
            right.display_tree_horizontal(indent + 4, true);
        }

        print!("{}", " ".repeat(indent));
        if indent > 0 {
            print!("{} ", if is_right { "┌──" } else { "└──" });
        }

        println!("[{}]", self.task.priority());

        if let Some(left) = &self.left {
            left.display_tree_horizontal(indent + 4, false);
        }
    }

    pub fn display_tree_vertical(&self) {
        let mut levels: Vec<Vec<Option<u32>>> = Vec::new();
        collect_levels(Some(self), &mut levels, 0);

        let max_depth = levels.len();
        let base_width = 1; // Width per node

        for (depth, level) in levels.iter().enumerate() {
            // Calculate spacing for this level
            let spaces_between = base_width * (max_depth - depth);
            let leading_spaces = spaces_between / 2;

            // Print leading spaces
            print!("{}", " ".repeat(leading_spaces));

            // Print each node with spacing
            for (i, node) in level.iter().enumerate() {
                match node {
                    Some(priority) => print!("{:^3}", priority), // Center in 3 chars
                    None => print!("   "), // Empty space
                }

                // Add spacing between nodes (except after last)
                if i < level.len() - 1 {
                    print!("{}", " ".repeat(spaces_between));
                }
            }

            println!(); // New line after each level
            // After printing a level, print the branches
            if depth < max_depth - 1 {
                print!("{}", " ".repeat(leading_spaces - 1));

                for (i, _) in level.iter().enumerate() {
                    print!("/");
                    print!("{}", " ".repeat(spaces_between - 2));
                    print!("\\");

                    if i < level.len() - 1 {
                        print!("{}", " ".repeat(spaces_between));
                    }
                }
                println!();
            }
        }
    }
}

fn collect_levels(node: Option<&TaskTree>, levels: &mut Vec<Vec<Option<u32>>>, depth: usize) {
    if levels.len() <= depth {
        levels.push(Vec::new());
    }

    match node {
        Some(tree) => {
            // Add this node's priority to its level
            levels[depth].push(Some(tree.task.priority()));

            // Recursively add children to next level
            collect_levels(tree.left.as_deref(), levels, depth + 1);
            collect_levels(tree.right.as_deref(), levels, depth + 1);
        }
        None => {
            // Add None to maintain tree structure
            levels[depth].push(None);
        }
    }
}

pub fn build_balanced_tree(tasks: &[(u32, String, bool)]) -> Option<TaskTree> {
    if tasks.is_empty() {
        return None;
    }

    let mid = tasks.len() / 2;
    let (priority, description, completed) = &tasks[mid];

    let mut tree = TaskTree::new(*priority, description.clone());

    if mid > 0 {
        tree.left = build_balanced_tree(&tasks[0..mid]).map(Box::new);
    }

    if mid + 1 < tasks.len() {
        tree.right = build_balanced_tree(&tasks[mid + 1..]).map(Box::new);
    }

    if *completed {
        tree.mark_complete(*priority);
    }

    Some(tree)
}
