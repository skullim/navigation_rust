use crate::geometry::Pose;

#[derive(Clone)]
pub struct LocalizationMsg
{
   pub pose: Pose,
}
pub struct LaserScanMsg
{
}
#[derive(Clone)]
pub struct ImuMsg
{
}

pub trait SupportedControlTrait
{
}

impl SupportedControlTrait for ControlOmnidirectionalMsg {}
impl SupportedControlTrait for ControlBicycleMsg {}
impl SupportedControlTrait for ControlDifferentialMsg {}

pub struct ControlOmnidirectionalMsg
{
}
pub struct ControlBicycleMsg
{
}
pub struct ControlDifferentialMsg
{
}