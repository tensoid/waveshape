pub fn lerp_f32(a: f32, b: f32, f: f32) -> f32
{
    a * (1.0 - f) + (b * f)
}

// pub trait RotateAround {
//     fn rotate_around(&mut self, pivot_point: Vec2, angle: f32);
// }

// impl RotateAround for Vec2 {
//     fn rotate_around(&mut self, pivot_point: Vec2, angle: f32) {
//         let s = angle.sin();
//         let c = angle.cos();

//         // translate point back to origin:
//         self.x -= pivot_point.x;
//         self.y -= pivot_point.y;

//         // rotate point
//         let xnew = self.x * c - self.y * s;
//         let ynew = self.x * s + self.y * c;

//         // translate point back:
//         self.x = xnew + pivot_point.x;
//         self.y = ynew + pivot_point.y;
//     }
// }

