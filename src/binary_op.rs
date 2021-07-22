pub trait BinaryOp<T> {
    fn run(&self, lhs: T, rhs: T) -> T;
    fn repr(&self) -> &'static str;
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum BinaryOpF32 {
    Add,
    Sub,
    Mul,
    Div,
    Min,
    Max,
    Pow,
    Hypot,
}

impl BinaryOp<f32> for BinaryOpF32 {
    fn run(&self, lhs: f32, rhs: f32) -> f32 {
        match self {
            BinaryOpF32::Add => lhs + rhs,
            BinaryOpF32::Sub => lhs - rhs,
            BinaryOpF32::Mul => lhs * rhs,
            BinaryOpF32::Div => lhs / rhs,
            BinaryOpF32::Min => lhs.min(rhs),
            BinaryOpF32::Max => lhs.max(rhs),
            BinaryOpF32::Pow => lhs.powf(rhs),
            BinaryOpF32::Hypot => lhs.hypot(rhs),
        }
    }
    fn repr(&self) -> &'static str {
        match self {
            BinaryOpF32::Add => "+",
            BinaryOpF32::Sub => "-",
            BinaryOpF32::Mul => "*",
            BinaryOpF32::Div => "/",
            BinaryOpF32::Min => "min",
            BinaryOpF32::Max => "max",
            BinaryOpF32::Pow => "pow",
            BinaryOpF32::Hypot => "hypot",
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum BinaryOpF64 {
    Add,
    Sub,
    Mul,
    Div,
    Min,
    Max,
    Pow,
    Hypot
}

impl BinaryOp<f64> for BinaryOpF64 {
    fn run(&self, lhs: f64, rhs: f64) -> f64 {
        match self {
            BinaryOpF64::Add => lhs + rhs,
            BinaryOpF64::Sub => lhs - rhs,
            BinaryOpF64::Mul => lhs * rhs,
            BinaryOpF64::Div => lhs / rhs,
            BinaryOpF64::Min => lhs.min(rhs),
            BinaryOpF64::Max => lhs.max(rhs),
            BinaryOpF64::Pow => lhs.powf(rhs),
            BinaryOpF64::Hypot => lhs.hypot(rhs),
        }
    }
    fn repr(&self) -> &'static str {
        match self {
            BinaryOpF64::Add => "+",
            BinaryOpF64::Sub => "-",
            BinaryOpF64::Mul => "*",
            BinaryOpF64::Div => "/",
            BinaryOpF64::Min => "min",
            BinaryOpF64::Max => "max",
            BinaryOpF64::Pow => "pow",
            BinaryOpF64::Hypot => "hypot",
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum BinaryOpI32 {
    Add,
    Sub,
    Mul,
    Div,
    Xor,
    And,
    Or,
    Shl,
    Shr,
}

impl BinaryOp<i32> for BinaryOpI32 {
    fn run(&self, lhs: i32, rhs: i32) -> i32 {
        match self {
            BinaryOpI32::Add => lhs + rhs,
            BinaryOpI32::Sub => lhs - rhs,
            BinaryOpI32::Mul => lhs * rhs,
            BinaryOpI32::Div => lhs.checked_div(rhs).unwrap_or(0),
            BinaryOpI32::Xor => lhs ^ rhs,
            BinaryOpI32::And => lhs & rhs,
            BinaryOpI32::Shl => lhs << rhs,
            BinaryOpI32::Shr => lhs >> rhs,
            BinaryOpI32::Or  => lhs | rhs,
        }
    }

    fn repr(&self) -> &'static str {
        match self {
            BinaryOpI32::Add => "+",
            BinaryOpI32::Sub => "-",
            BinaryOpI32::Mul => "*",
            BinaryOpI32::Div => "/",
            BinaryOpI32::Xor => "^",
            BinaryOpI32::And => "&",
            BinaryOpI32::Or  => "|",
            BinaryOpI32::Shl => "<<",
            BinaryOpI32::Shr => ">>",
        }
    }
}
