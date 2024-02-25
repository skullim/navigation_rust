use angle::Angle as AngleExt;
use angle::Rad;

use std::ops::Add;
use std::ops::Sub;

pub trait SpatialPositionTag {}

#[derive(Clone, Debug)]
pub enum AngleWrapping {
    PlusMinusPi,
    TwoPi,
}

#[derive(Clone, Debug)]
pub struct Angle {
    angle: angle::Rad<f32>,
    wrapping: AngleWrapping,
}

impl Angle {
    pub fn new(angle: f32, wrapping: AngleWrapping) -> Self {
        Self::wrap(angle, wrapping)
    }

    ///@todo: maybe use WrappingStrategy with wrap trait instead?
    fn wrap(angle: f32, wrapping: AngleWrapping) -> Self {
        let mut angle = angle::Rad(angle).wrap();

        if let AngleWrapping::TwoPi = wrapping {
            return Self { angle, wrapping };
        }

        let pi = Rad::<f32>::pi();
        if angle > pi {
            angle -= Rad::<f32>::two_pi();
        }
        Self {
            angle,
            wrapping: AngleWrapping::PlusMinusPi,
        }
    }

    fn value(self) -> f32 {
        self.angle.value()
    }

    fn sin(self) -> f32 {
        self.angle.value().sin()
    }

    fn cos(self) -> f32 {
        self.angle.value().cos()
    }
}

impl Add for Angle {
    type Output = Angle;

    fn add(self, other: Angle) -> Angle {
        let added_angle = self.angle.add(other.angle);
        Self::wrap(added_angle.value(), self.wrapping)
    }
}

impl Sub for Angle {
    type Output = Angle;

    fn sub(self, other: Angle) -> Angle {
        let subtracted_angle = self.angle.sub(other.angle);
        Self::wrap(subtracted_angle.value(), self.wrapping)
    }
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug)]
pub struct Pose {
    pub x: f32,
    pub y: f32,
    pub theta: Angle,
}

pub struct G1Pose {
    pub x: f32,
    pub y: f32,
    pub theta: Angle,
    pub kappa: f32,
}

impl SpatialPositionTag for Point {}
impl SpatialPositionTag for Pose {}
impl SpatialPositionTag for G1Pose {}
