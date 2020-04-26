use std::ops::*;
use std::f32;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    e:[f32;3]
}

impl Vec3 {
    pub fn zero() -> Vec3{
        Vec3 {e:[0.0, 0.0, 0.0]}
    }

    pub fn new(x:f32, y:f32, z:f32) -> Vec3{
        Vec3 {e:[x, y, z]}
    }

    pub fn x (&self) -> f32 {
        self.e[0]
    }

    pub fn y (&self) -> f32 {
        self.e[1]
    }

    pub fn z (&self) -> f32 {
        self.e[2]
    }

    pub fn r (&self) -> f32 {
        self.e[0]
    }

    pub fn g (&self) -> f32 {
        self.e[1]
    }

    pub fn b (&self) -> f32 {
        self.e[2]
    }

    pub fn length (&self) -> f32 {
        self.squered_length().sqrt()
    }

    pub fn squered_length (&self) -> f32 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }

    pub fn make_unit_vector(&self) -> Vec3{
        *self / self.length()
    }
}

// related function
pub fn dot(v1 :Vec3, v2 :Vec3) -> f32 {
    v1.e[0]*v2.e[0] + v1.e[1]*v2.e[1] + v1.e[2]*v2.e[2]
}

pub fn cross(v1 :Vec3, v2 :Vec3) -> Vec3 {
    Vec3 {e: [
        v1.e[1]*v2.e[2] - v1.e[2]*v2.e[1],
        v1.e[2]*v2.e[0] - v1.e[0]*v2.e[2],
        v1.e[0]*v2.e[1] - v1.e[1]*v2.e[0]]
    }
}

// operator overload
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, operand : Vec3) -> Vec3 {
        Vec3 {e:[
            self.e[0] + operand.e[0], 
            self.e[1] + operand.e[1], 
            self.e[2] + operand.e[2]] 
        }
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;
    fn add(self, operand : f32) -> Vec3 {
        Vec3 {e:[
            self.e[0] + operand, 
            self.e[1] + operand, 
            self.e[2] + operand]
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, operand : Vec3) -> Vec3 {
        Vec3 {e:[
            self.e[0] - operand.e[0], 
            self.e[1] - operand.e[1], 
            self.e[2] - operand.e[2]] 
        }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Vec3;
    fn sub(self, operand : f32) -> Vec3 {
        Vec3 {e:[
            self.e[0] - operand, 
            self.e[1] - operand, 
            self.e[2] - operand]
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, operand : Vec3) -> Vec3 {
        Vec3 {e:[
            self.e[0] * operand.e[0], 
            self.e[1] * operand.e[1], 
            self.e[2] * operand.e[2]] 
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, operand : f32) -> Vec3 {
        Vec3 {e:[
            self.e[0] * operand, 
            self.e[1] * operand, 
            self.e[2] * operand]
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, operand : Vec3) -> Vec3 {
        Vec3 {e:[
            self.e[0] / operand.e[0], 
            self.e[1] / operand.e[1], 
            self.e[2] / operand.e[2]] 
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, operand : f32) -> Vec3 {
        Vec3 {e:[
            self.e[0] / operand, 
            self.e[1] / operand, 
            self.e[2] / operand]
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, operand : Vec3){
        self.e[0] += operand.e[0];
        self.e[1] += operand.e[1];
        self.e[2] += operand.e[2];
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, operand : Vec3){
        self.e[0] -= operand.e[0];
        self.e[1] -= operand.e[1];
        self.e[2] -= operand.e[2];
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, operand : Vec3){
        self.e[0] *= operand.e[0];
        self.e[1] *= operand.e[1];
        self.e[2] *= operand.e[2];
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, operand : Vec3){
        self.e[0] /= operand.e[0];
        self.e[1] /= operand.e[1];
        self.e[2] /= operand.e[2];
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {e:[
            -self.e[0],
            -self.e[1],
            -self.e[2]]
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
  
    fn index(&self, i: usize) -> &f32 {
      &self.e[i]
    }
}
  
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
      &mut self.e[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot() {
        let v = Vec3::new(4.0,2.0,0.0);
        let v2 = Vec3::new(2.0,-4.0,0.0);
        let vz = Vec3::zero();
        assert_eq!(dot(v,v2), 0.0);
    }
}