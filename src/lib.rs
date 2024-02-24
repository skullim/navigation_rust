///@todo Adrian: How to abstract sensor data? 


pub mod geometry;
pub mod vehicle;
pub mod map_handler;
pub mod collision_checker;
pub mod jobs_dispatcher;
pub mod behavior_controller;

pub mod path_planner;
pub mod global_planner;
pub mod smoother;
pub mod trajectory_planner;
pub mod controller;

pub mod data_storage;
pub mod communication_msgs;



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
