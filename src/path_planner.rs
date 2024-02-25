///@todo check if Enum is not better
pub trait PathTag {}

struct Path {
    cont: f64,
}

struct DiscretizedPath {
    sample: i32,
}

impl PathTag for Path {}
impl PathTag for DiscretizedPath {}

///@todo Add error handling -> wrap into Result?
trait PlanPath {
    type PlannerParameters;
    fn plan(&self, params: Self::PlannerParameters) -> impl PathTag;
}

use crate::geometry::Point;
/// Usage
use crate::geometry::Pose;

struct BestPlannerParams {
    start: Pose,
    end: Point,
}

struct BestPlanner {}
impl PlanPath for BestPlanner {
    type PlannerParameters = BestPlannerParams;
    fn plan(&self, params: BestPlannerParams) -> Path {
        Path { cont: 0.5 }
    }
}

struct BestDiscretizedPlannerParams {
    start: Pose,
    middle: Pose,
    end: Pose,
}

struct BestDiscretizedPathPlanner {}
impl PlanPath for BestDiscretizedPathPlanner {
    type PlannerParameters = BestDiscretizedPlannerParams;
    fn plan(&self, params: BestDiscretizedPlannerParams) -> DiscretizedPath {
        DiscretizedPath { sample: 15 }
    }
}
