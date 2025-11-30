use ab_glyph::PxScale;

#[derive(Clone, Copy)]
pub struct InternalScale {
    /// Horizontal scale, in pixels.
    pub x: f32,
    /// Vertical scale, in pixels.
    pub y: f32,
}

// impl Into<Scale> for InternalScale {
//     fn into(self) -> Scale {
//         Scale {
//             x: self.x,
//             y: self.y,
//         }
//     }
// }

impl From<InternalScale> for PxScale {
    fn from(value: InternalScale) -> Self {
        PxScale {
            x: value.x,
            y: value.y,
        }
    }
}
