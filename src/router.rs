use std::collections::{HashSet, VecDeque};

pub struct Router {
    memory_limit: i32,
    packet_map: HashSet<String>,
    packet_queue: VecDeque<(i32, i32, i32)>,
}

impl Router {
    pub fn new(memory_limit: i32) -> Self {
        Router {
            memory_limit,
            packet_map: HashSet::new(),
            packet_queue: VecDeque::new(),
        }
    }

    pub fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let key = Self::create_key(source, destination, timestamp);
        if self.packet_map.contains(&key) {
            return false;
        }
        // Remove oldest if at memory limit
        if self.packet_queue.len() as i32 >= self.memory_limit {
            if let Some((old_source, old_dest, old_time)) = self.packet_queue.pop_front() {
                let old_key = Self::create_key(old_source, old_dest, old_time);
                self.packet_map.remove(&old_key);
            }
        }
        self.packet_map.insert(key);
        self.packet_queue
            .push_back((source, destination, timestamp));
        true
    }

    pub fn forward_packet(&mut self) -> Vec<i32> {
        if let Some((source, destination, timestamp)) = self.packet_queue.pop_front() {
            let key = Self::create_key(source, destination, timestamp);
            self.packet_map.remove(&key);
            vec![source, destination, timestamp]
        } else {
            vec![]
        }
    }

    pub fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        let mut count = 0;
        for &(_, dest, timestamp) in &self.packet_queue {
            if dest == destination && timestamp >= start_time && timestamp <= end_time {
                count += 1;
            }
        }
        count
    }

    fn create_key(source: i32, destination: i32, timestamp: i32) -> String {
        format!("{}-{}-{}", source, destination, timestamp)
    }
}
