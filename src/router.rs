use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Router {
    memory_limit: i32,
    packet_map: HashSet<String>,
    packet_queue: VecDeque<(i32, i32, i32)>,
    dest_in_time_map: HashMap<i32, Vec<i32>>,
}

impl Router {
    pub fn new(memory_limit: i32) -> Self {
        Router {
            memory_limit,
            packet_map: HashSet::with_capacity(memory_limit as usize),
            packet_queue: VecDeque::with_capacity(memory_limit as usize),
            dest_in_time_map: HashMap::with_capacity(memory_limit as usize),
        }
    }

    pub fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let key = Self::create_key(source, destination, timestamp);
        if self.packet_map.contains(&key) {
            return false;
        }
        // Remove oldest if at memory limit
        if self.packet_queue.len() as i32 >= self.memory_limit {
            self.forward_packet();
        }
        self.packet_map.insert(key);
        self.packet_queue
            .push_back((source, destination, timestamp));
        self.dest_in_time_map
            .entry(destination)
            .or_insert_with(Vec::new)
            .push(timestamp);
        true
    }

    pub fn forward_packet(&mut self) -> Vec<i32> {
        if let Some((source, destination, timestamp)) = self.packet_queue.pop_front() {
            let key = Self::create_key(source, destination, timestamp);
            self.packet_map.remove(&key);

            // Remove only the first occurrence of this timestamp
            if let Some(times) = self.dest_in_time_map.get_mut(&destination) {
                if let Some(pos) = times.iter().position(|&t| t == timestamp) {
                    times.remove(pos);
                }
            }

            vec![source, destination, timestamp]
        } else {
            vec![]
        }
    }

    pub fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        if let Some(times) = self.dest_in_time_map.get(&destination) {
            let left = times.partition_point(|&x| x < start_time);
            let right = times.partition_point(|&x| x <= end_time);
            (right - left) as i32
        } else {
            0
        }
    }
    fn create_key(source: i32, destination: i32, timestamp: i32) -> String {
        format!("{}-{}-{}", source, destination, timestamp)
    }
}
