pub trait TernaryOp<T> {
    fn run(&self, a: T, b: T, c: T) -> T;
    fn repr(&self) -> &'static str;
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum TernaryOpF32 {
    MulAdd,
    Clamp,
}

impl TernaryOp<f32> for TernaryOpF32 {
    fn run(&self, a: f32, b: f32, c: f32) -> f32 {
        match self {
            TernaryOpF32::MulAdd => a.mul_add(b, c),
            TernaryOpF32::Clamp => a.clamp(b, c),
        }
    }
    fn repr(&self) -> &'static str {
        match self {
            TernaryOpF32::MulAdd => "mul_add",
            TernaryOpF32::Clamp => "clamp",
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum TernaryOpF64 {
    MulAdd,
    Clamp,
}

impl TernaryOp<f64> for TernaryOpF64 {
    fn run(&self, a: f64, b: f64, c: f64) -> f64 {
        match self {
            TernaryOpF64::MulAdd => a.mul_add(b, c),
            TernaryOpF64::Clamp => a.clamp(b, c),
        }
    }
    fn repr(&self) -> &'static str {
        match self {
            TernaryOpF64::MulAdd => "mul_add",
            TernaryOpF64::Clamp => "clamp",
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum TernaryOpI32 {
    Clamp,
}

impl TernaryOp<i32> for TernaryOpI32 {
    fn run(&self, a: i32, b: i32, c: i32) -> i32 {
        match self {
            TernaryOpI32::Clamp => a.max(b).min(c),
        }
    }
    fn repr(&self) -> &'static str {
        match self {
            TernaryOpI32::Clamp => "clamp",
        }
    }
}
