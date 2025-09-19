#![allow(dead_code, unused_imports, unused_variables, unused_mut)]

mod task_manager_system;
use core::task;

use task_manager_system::TaskManager;

fn main() {
    let tasks = [[3, 12, 11], [6, 2, 46], [2, 1, 46], [5, 23, 21]]
        .map(|x| x.to_vec())
        .to_vec();
    let mut task_manager = TaskManager::new(tasks);
    dbg!(&task_manager);
}
