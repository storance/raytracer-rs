use num::{Zero, Float};
use math::vector::Vector3f;
use math::point::Point3f;
use math::scalar::FloatScalar;
use std::convert::From;

#[derive(PartialEq, Copy, Clone)]
pub struct Ray {
    pub origin: Point3f,
    pub direction: Vector3f,
    pub tmax: FloatScalar,
    pub time: FloatScalar,
}

pub struct RayDifferential {
    pub ray: Ray,
    pub rx_origin: Option<Point3f>,
    pub ry_origin: Option<Point3f>,
    pub rx_direction: Option<Vector3f>,
    pub ry_direction: Option<Vector3f>,
}

impl Ray {
    pub fn new(origin: Point3f, direction: Vector3f) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
            tmax: Float::infinity(),
            time: FloatScalar::zero(),
        }
    }

    pub fn with_tmax(&self, tmax: FloatScalar) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.direction,
            tmax: tmax,
            time: self.time,
        }
    }

    pub fn with_time(&self, time: FloatScalar) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.direction,
            tmax: self.tmax,
            time: time,
        }
    }

    pub fn point_at(&self, t: FloatScalar) -> Point3f {
        self.origin + self.direction * t
    }
}

impl RayDifferential {
    pub fn new(origin: Point3f, direction: Vector3f) -> RayDifferential {
        RayDifferential {
            ray: Ray::new(origin, direction),
            rx_origin: None,
            ry_origin: None,
            rx_direction: None,
            ry_direction: None
        }
    }

    pub fn with_differentials(&self, rx_origin: Point3f, rx_direction: Vector3f, ry_origin: Point3f,
            ry_direction: Vector3f) -> RayDifferential {
        RayDifferential {
            ray: self.ray,
            rx_origin: Some(rx_origin),
            ry_origin: Some(ry_origin),
            rx_direction: Some(rx_direction),
            ry_direction: Some(ry_direction),
        }
    }

    pub fn scale_differentials(&self, t: FloatScalar) -> RayDifferential {
        RayDifferential {
            ray: self.ray,
            rx_origin: self.rx_origin.map(|origin| self.ray.origin + (origin - self.ray.origin) * t),
            ry_origin: self.ry_origin.map(|origin| self.ray.origin + (origin - self.ray.origin) * t),
            rx_direction: self.rx_direction.map(|dir| self.ray.direction + (dir - self.ray.direction) * t),
            ry_direction: self.ry_direction.map(|dir| self.ray.direction + (dir - self.ray.direction) * t),
        }
    }

    pub fn has_differentials(&self) -> bool {
        self.rx_origin.is_some() && self.ry_origin.is_some() && self.rx_direction.is_some() 
            && self.ry_direction.is_some()
    }
}

impl From<Ray> for RayDifferential {
    fn from(ray: Ray) -> RayDifferential {
        RayDifferential {
            ray: ray,
            rx_origin: None,
            ry_origin: None,
            rx_direction: None,
            ry_direction: None,
        }
    }
}