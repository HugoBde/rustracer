use crate::geometry::{Point3, Ray};

pub struct HitRecord {
    pub hit: Point3,
    pub target: Box<dyn Hittable>,
}

pub trait Hittable {
    fn check_hit(&self, ray: &Ray) -> Option<HitRecord>;
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn check_hit(&self, ray: &Ray) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let projected_distance = ray.direction * oc;
        let projected_vector = ray.direction * projected_distance;
        let projection_vector = oc - projected_vector;
        if projection_vector.square() > self.radius.powi(2) {
            return None;
        } else {
            let offset = (self.radius.powi(2) - projection_vector.square()).sqrt();
            if offset < projected_distance {
                return Some(HitRecord {
                    hit: ray.origin + ray.direction * (projected_distance - offset),
                    target: Box::new(*self),
                });
            } else {
                return Some(HitRecord {
                    hit: ray.origin + ray.direction * (projected_distance + offset),
                    target: Box::new(*self),
                });
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Triangle {
    pub a: Point3,
    pub b: Point3,
    pub c: Point3,
}

impl Triangle {
    pub fn new(a: Point3, b: Point3, c: Point3) -> Triangle {
        Triangle { a, b, c }
    }
}

impl Hittable for Triangle {
    fn check_hit(&self, ray: &Ray) -> Option<HitRecord> {
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        let normal_vector = ab.cross(ac);
        let dot_product = normal_vector * ray.direction;
        if dot_product.abs() > f32::EPSILON {
            Some(HitRecord {
                hit: Point3::new(0.0, 0.0, 0.0),
                target: Box::new(*self),
            })
        } else {
            None
        }
    }
}
