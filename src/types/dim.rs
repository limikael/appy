/// Specify dimension in absolute or percentual units.
#[derive(Default, Clone)]
pub enum Dim {
    /// No dimension specified.
    #[default]
    None,

    /// Percentual size relative to parent.
    Pc(f32),

    /// Absolute size specified in hardware pixels.
    Px(f32),

    /// Size specified in device independent pixels.
    /// This is the same as hardware pixels, scaled with
    /// a factor defined by the device.
    Dp(f32)
}

impl Dim {
    pub fn is_some(&self) -> bool {
        !matches!(*self, Dim::None)
    }

    pub fn to_px(&self, max: f32, pixel_ratio: f32) -> f32 {
        match *self {
            Dim::Px(v) => v,
            Dim::Pc(v) => max * v / 100.0,
            Dim::Dp(v) => v * pixel_ratio,
            Dim::None => 0.0,
        }
    }

    pub fn compute_span(max: f32, pr: f32, start: Dim, size: Dim, end: Dim) -> (f32, f32) {
        let c = if start.is_some() { 1 << 2 } else { 0 }
            + if size.is_some() { 1 << 1 } else { 0 }
            + if end.is_some() { 1 << 0 } else { 0 };

        match c {
            0b000 => (0.0, max),
            0b001 => (0.0, max - end.to_px(max,pr)),
            0b010 => ((max - size.to_px(max,pr)) / 2.0, size.to_px(max,pr)),
            0b011 => (max - size.to_px(max,pr) - end.to_px(max,pr), size.to_px(max,pr)),
            0b100 => (start.to_px(max,pr), max - start.to_px(max,pr)),
            0b101 => (start.to_px(max,pr), max - start.to_px(max,pr) - end.to_px(max,pr)),
            0b110 => (start.to_px(max,pr), size.to_px(max,pr)),
            0b111 => panic!("over constrained"),
            _ => panic!("bad constraints"),
        }
    }
}

impl Dim {}
