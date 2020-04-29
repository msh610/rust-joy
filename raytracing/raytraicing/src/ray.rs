
use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray{
    a : Vec3,
    b : Vec3
}

impl Ray {
    pub fn new(a: Vec3, b :Vec3) -> Ray {
        Ray{a:a, b:b}
    }

    pub fn origin (&self) -> Vec3 {
        self.a
    }

    pub fn direction (&self) -> Vec3 {
        self.b
    }

    pub fn point_at_parameter(&self, t :f32) -> Vec3 {
        self.a + self.b*t
    }
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn origin() {
        let ray = Ray::new(Vec3::zero(), Vec3::new(5.0,1.0,2.0));
        assert_eq!(ray.origin().x(), 0.0);

    }

    fn direction() {
        let ray = Ray::new(Vec3::zero(), Vec3::new(5.0,1.0,2.0));
        assert_eq!(ray.direction().x(), 5.0);
        assert_eq!(ray.direction().y(), 1.0);
    }



}