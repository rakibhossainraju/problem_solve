use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug)]
pub struct Task {
    priority: i32,
    user_id: i32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct QueueItem {
    priority: i32,
    task_id: i32,
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority
            .cmp(&other.priority)
            .then_with(|| self.task_id.cmp(&other.task_id))
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct TaskManager {
    tasks: HashMap<i32, Task>,
    queue: BinaryHeap<QueueItem>,
}

impl TaskManager {
    pub fn new(initial_tasks: Vec<Vec<i32>>) -> Self {
        let mut tasks = HashMap::new();
        let mut queue = BinaryHeap::new();

        for task in initial_tasks {
            let user_id = task[0];
            let task_id = task[1];
            let priority = task[2];

            tasks.insert(task_id, Task { priority, user_id });
            queue.push(QueueItem { priority, task_id });
        }

        Self { tasks, queue }
    }

    pub fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.tasks.insert(task_id, Task { priority, user_id });
        self.queue.push(QueueItem { priority, task_id });
    }

    pub fn edit(&mut self, task_id: i32, new_priority: i32) {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.priority = new_priority;
            self.queue.push(QueueItem {
                priority: new_priority,
                task_id,
            });
        }
    }

    pub fn rmv(&mut self, task_id: i32) {
        self.tasks.remove(&task_id);
    }

    pub fn exec_top(&mut self) -> i32 {
        while let Some(item) = self.queue.pop() {
            if let Some(task) = self.tasks.get(&item.task_id) {
                if task.priority == item.priority {
                    let user_id = task.user_id;
                    self.tasks.remove(&item.task_id);
                    return user_id;
                }
            }
        }
        -1
    }
}
