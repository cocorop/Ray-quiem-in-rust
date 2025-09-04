use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};
use std::io::{self, Write};
#[derive(Debug, Copy, Clone)]
struct Vec3 {
    e: [f64;3],
}


impl Vec3 {

    //Constructeur

    fn new(x:f64, y:f64, z:f64) -> Self {
        Vec3 { e: [x, y, z] }
    }

    //Getters
    fn x(&self) -> f64 {self.e[0]}
    fn y(&self) -> f64 {self.e[1]}
    fn z(&self) -> f64 {self.e[2]}

    //fonctions autres
    fn cross(&self, other: &Self) -> Self {
        Vec3::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    fn length(&self) -> f64 {
        (self.x()*self.x() + self.y()*self.y() + self.z()*self.z()).sqrt()
    }
    fn unit(&self) -> Self {
        *self / self.length()
    }


    //ecrire couleur
    pub fn write_color<W: Write>(out: &mut W, pixel_color: &Vec3) -> io::Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Convert [0.0, 1.0] floats to [0, 255] bytes
    let r_byte = (r) as i32;
    let g_byte = (g) as i32;
    let b_byte = (b) as i32;

    // Write the pixel color
    writeln!(out, "{} {} {}", r_byte, g_byte, b_byte)
}
}

//opérateurs
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Self) -> Self {
        Vec3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Self) -> Self {
        Vec3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Self {
        Vec3::new(
            self.x() * t,
            self.y() * t,
            self.z() * t,
        )
    }
}


impl Mul for Vec3 {
    type Output = f64;
    fn mul(self, other: Self) -> f64 {
        self.x() * other.x() +
        self.y() * other.y() +
        self.z() * other.z()
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Self {
        self * (1.0/t)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self {
        Vec3::new(
            -self.x(),
            -self.y(),
            -self.z(),
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.x();
        self.e[1] += other.y();
        self.e[2] += other.z();
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.e[0] -= other.x();
        self.e[1] -= other.y();
        self.e[2] -= other.z();
    }
}
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0/t;
    }
}


#[cfg(test)]
mod tests {
    use super::*; // import Vec3 into this scope

    #[test]
    fn test_new_and_getters() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn test_addition() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let result = a + b;
        assert_eq!(result.x(), 5.0);
        assert_eq!(result.y(), 7.0);
        assert_eq!(result.z(), 9.0);
    }

    #[test]
    fn test_subtraction() {
        let a = Vec3::new(5.0, 7.0, 9.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        let result = a - b;
        assert_eq!(result.x(), 4.0);
        assert_eq!(result.y(), 5.0);
        assert_eq!(result.z(), 6.0);
    }

    #[test]
    fn test_scalar_multiplication() {
        let v = Vec3::new(1.0, -2.0, 3.0);
        let result = v * 2.0;
        assert_eq!(result.x(), 2.0);
        assert_eq!(result.y(), -4.0);
        assert_eq!(result.z(), 6.0);
    }

    #[test]
    fn test_scalar_division() {
        let v = Vec3::new(2.0, 4.0, 6.0);
        let result = v / 2.0;
        assert_eq!(result.x(), 1.0);
        assert_eq!(result.y(), 2.0);
        assert_eq!(result.z(), 3.0);
    }

    #[test]
    fn test_dot_product() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, -5.0, 6.0);
        let dot = a * b; // you overloaded * for dot product
        assert_eq!(dot, 12.0);
    }

    #[test]
    fn test_cross_product() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let cross = a.cross(&b);
        assert_eq!(cross.x(), -3.0);
        assert_eq!(cross.y(), 6.0);
        assert_eq!(cross.z(), -3.0);
    }

    #[test]
    fn test_length_and_unit() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(v.length(), 5.0);
        let unit = v.unit();
        assert!((unit.length() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_negation() {
        let v = Vec3::new(1.0, -2.0, 3.0);
        let neg = -v;
        assert_eq!(neg.x(), -1.0);
        assert_eq!(neg.y(), 2.0);
        assert_eq!(neg.z(), -3.0);
    }
}