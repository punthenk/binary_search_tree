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

    pub fn mark_complete(&mut self, priority: u32,) -> bool {
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
