#![allow(dead_code, unused_imports, unused_variables, unused_mut)]
mod router;
mod solution;
mod spreadsheet;
mod task_manager_system;

use router::Router;
use solution::Solution;
use spreadsheet::Spreadsheet;
use task_manager_system::TaskManager;

fn main() {
    let mut router = Router::new(3);
    router.add_packet(1, 4, 90);
    router.add_packet(2, 5, 90);
    router.add_packet(1, 4, 90);
    router.add_packet(3, 5, 95);
    router.add_packet(4, 5, 105);
    router.forward_packet();
    router.add_packet(5, 2, 110);
    router.get_count(5, 100, 110);
    dbg!(&router);
}
