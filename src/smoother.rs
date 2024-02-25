use crate::path_planner::PathTag;

trait Smooth {
    fn smooth(path: &impl PathTag) {}
}
