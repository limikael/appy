/// Specify dimension in absolute or percentual units.
#[derive(Default, Clone)]
pub enum Dim {
    /// No dimension specified.
    #[default]
    None,

    /// Percentual size relative to parent.
    Percent(f32),

    /// Size specified as an absolute value.
    Absolute(f32)
}

impl Dim {
    pub fn is_some(&self) -> bool {
        !matches!(*self, Dim::None)
    }

    pub fn to_abs(&self, max: f32) -> f32 {
        match *self {
            Dim::Percent(v) => max * v / 100.0,
            Dim::Absolute(v) => v,
            Dim::None => 0.0,
        }
    }

    pub fn compute_span(max: f32, start: Dim, size: Dim, end: Dim) -> (f32, f32) {
        let c = if start.is_some() { 1 << 2 } else { 0 }
            + if size.is_some() { 1 << 1 } else { 0 }
            + if end.is_some() { 1 << 0 } else { 0 };

        match c {
            0b000 => (0.0, max),
            0b001 => (0.0, max - end.to_abs(max)),
            0b010 => ((max - size.to_abs(max)) / 2.0, size.to_abs(max)),
            0b011 => (max - size.to_abs(max) - end.to_abs(max), size.to_abs(max)),
            0b100 => (start.to_abs(max), max - start.to_abs(max)),
            0b101 => (start.to_abs(max), max - start.to_abs(max) - end.to_abs(max)),
            0b110 => (start.to_abs(max), size.to_abs(max)),
            0b111 => panic!("over constrained"),
            _ => panic!("bad constraints"),
        }
    }
}

impl<T> From<T> for Dim
        where f64: From<T> {
    /// Create a Dim from a value.
    fn from(val: T)->Self {
        Dim::Absolute(f64::from(val) as f32)
    }
}

/// Convenience function to create a percentual dimension.
pub fn pct<T>(val: T)->Dim
        where f64: From<T> {
    Dim::Percent(f64::from(val) as f32)
}
