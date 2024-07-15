use super::vec2;

/// Just a geometrical quad.
#[derive(Debug, Clone, Copy)]
pub struct Quad {
    pub ld: vec2,
    pub rd: vec2,
    pub lu: vec2,
    pub ru: vec2,
}

impl Quad {
    pub fn contains(&self, point: vec2) -> bool {
        return
            internal::point_in_triangle(point, [self.lu, self.ld, self.rd]) || // First triangle
            internal::point_in_triangle(point, [self.rd, self.ru, self.lu]); // Second triangle
    }
}

mod internal {
    use crate::math::vec2;

    // https://stackoverflow.com/questions/2049582/how-to-determine-if-a-point-is-in-a-2d-triangle
    pub fn point_in_triangle(point: vec2, verts: [vec2; 3]) -> bool {
        fn sign(p: vec2, refs: [vec2; 2]) -> f32 {
            return (p.0 - refs[1].0) * (refs[0].1 - refs[1].1) - (refs[0].0 - refs[1].0) * (p.1 - refs[1].1);
        }

        let d1 = sign(point, [verts[0], verts[1]]);
        let d2 = sign(point, [verts[1], verts[2]]);
        let d3 = sign(point, [verts[2], verts[0]]);

        let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
        let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);

        return !has_neg || !has_pos;
    }
}
