pub mod color;
pub mod operations;
pub mod ray;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub(crate) fn new() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub(crate) fn x(&self) -> f64 {
        self.0
    }

    pub(crate) fn y(&self) -> f64 {
        self.1
    }

    pub(crate) fn z(&self) -> f64 {
        self.2
    }

    pub(crate) fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub(crate) fn length_squared(&self) -> f64 {
        f64::powi(self.0, 2) + f64::powi(self.1, 2) + f64::powi(self.2, 2)
    }

    pub(crate) fn dot(lhs: &Self, rhs: &Self) -> f64 {
        lhs.0 * rhs.0 + lhs.1 * rhs.1 + lhs.2 * rhs.2
    }

    pub(crate) fn cross(lhs: &Self, rhs: &Self) -> Self {
        Self(
            lhs.1 * rhs.2 - lhs.2 * rhs.1,
            lhs.2 * rhs.0 - lhs.0 * rhs.2,
            lhs.0 * rhs.1 - lhs.1 * rhs.0,
        )
    }

    pub(crate) fn unit_vector(v: &Self) -> Self {
        *v / v.length()
    }
}
