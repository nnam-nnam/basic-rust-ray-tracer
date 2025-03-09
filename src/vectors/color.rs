use std::fmt::Display;

pub(crate) use super::Vec3 as Color;

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = self.x();
        let g = self.y();
        let b = self.z();

        let r = (255.999 * r) as i32;
        let g = (255.999 * g) as i32;
        let b = (255.999 * b) as i32;

        writeln!(f, "{r} {g} {b}")?;

        Ok(())
    }
}
