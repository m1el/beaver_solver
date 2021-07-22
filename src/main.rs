mod binary_op;
mod ternary_op;
mod program;
// mod compile;

use binary_op::{BinaryOpF64};
use ternary_op::{TernaryOpF64};
use program::{Node, ProgramF64};
// use compile::{compile, to_fn};
use core_simd::{SimdF64, LanesAtMost32};

fn k_sin(x: f64) -> f64 {
    const S0: f64 = -1.66666666666666324348e-01; /* 0xBFC55555, 0x55555549 */
    const S1: f64 =  8.33333333332248946124e-03; /* 0x3F811111, 0x1110F8A6 */
    const S2: f64 = -1.98412698298579493134e-04; /* 0xBF2A01A0, 0x19C161D5 */
    const S3: f64 =  2.75573137070700676789e-06; /* 0x3EC71DE3, 0x57B1FE7D */
    const S4: f64 = -2.50507602534068634195e-08; /* 0xBE5AE5E6, 0x8A2B9CEB */
    const S5: f64 =  1.58969099521155010221e-10; /* 0x3DE5D93A, 0x5ACFD57C */
    let z = x*x;
    let w = z*z;
    let r = S1 + z*(S2 + z*S3 + z*w*(S4 + z*S5));
    let v = z*x;
    x + v*(S0 + z*r)
}

fn k_sin_simd<const LANES: usize>(x: SimdF64<LANES>) -> SimdF64<LANES>
    where SimdF64<LANES>: LanesAtMost32
{
    const S0: f64 = -1.66666666666666324348e-01; /* 0xBFC55555, 0x55555549 */
    const S1: f64 =  8.33333333332248946124e-03; /* 0x3F811111, 0x1110F8A6 */
    const S2: f64 = -1.98412698298579493134e-04; /* 0xBF2A01A0, 0x19C161D5 */
    const S3: f64 =  2.75573137070700676789e-06; /* 0x3EC71DE3, 0x57B1FE7D */
    const S4: f64 = -2.50507602534068634195e-08; /* 0xBE5AE5E6, 0x8A2B9CEB */
    const S5: f64 =  1.58969099521155010221e-10; /* 0x3DE5D93A, 0x5ACFD57C */
    let z = x*x;
    let w = z*z;
    let r = SimdF64::splat(S1) + z*(SimdF64::splat(S2) + z*SimdF64::splat(S3) + z*w*(SimdF64::splat(S4) + z*SimdF64::splat(S5)));
    let v = z*x;
    x + v*(SimdF64::splat(S0) + z*r)
}
/*
#[derive(Clone, Copy)]
enum Instr {
    Set(u8, u8),
    Add(u8, u8, u8),
    Mul(u8, u8, u8),
}

struct CProgram {
    instrs: Vec<Instr>,
    memory: Vec<f64>,
}

impl CProgram {
    fn run(&mut self) {
        for instr in self.instrs.iter().copied() {
            match instr {
                Instr::Set(dst, src) => {
                    self.memory[dst as usize] = self.memory[src as usize];
                }
                Instr::Add(dst, lhs, rhs) => {
                    self.memory[dst as usize] = self.memory[lhs as usize] + self.memory[rhs as usize];
                }
                Instr::Mul(dst, lhs, rhs) => {
                    self.memory[dst as usize] = self.memory[lhs as usize] * self.memory[rhs as usize];
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
struct TInstr {
    op_byte: u8,
    a: u8,
    b: u8,
    c: u8,
}
impl TInstr {
    fn new(op_byte: u8, a: u8, b: u8, c: u8) -> Self {
        Self { op_byte, a, b, c }
    }
}

struct TProgram {
    instrs: Vec<TInstr>,
    memory: Vec<f64>,
}

fn op_add(vm_state: &mut TProgram, instr: TInstr, pc: usize) {
    let TInstr { op_byte: _, a, b, c } = instr;
    vm_state.memory[a as usize] = vm_state.memory[b as usize] + vm_state.memory[c as usize];
    let instr = vm_state.instrs[pc];
    OP_TABLE[instr.op_byte as usize](vm_state, instr, pc + 1)
}
fn op_mul(vm_state: &mut TProgram, instr: TInstr, pc: usize) {
    let TInstr { op_byte: _, a, b, c } = instr;
    vm_state.memory[a as usize] = vm_state.memory[b as usize] * vm_state.memory[c as usize];
    let instr = vm_state.instrs[pc];
    OP_TABLE[instr.op_byte as usize](vm_state, instr, pc + 1)
}
fn op_set(vm_state: &mut TProgram, instr: TInstr, pc: usize) {
    let TInstr { op_byte: _, a, b, c: _ } = instr;
    vm_state.memory[a as usize] = vm_state.memory[b as usize];
    let instr = vm_state.instrs[pc];
    OP_TABLE[instr.op_byte as usize](vm_state, instr, pc + 1)
}
fn op_return(_vm_state: &mut TProgram, _instr: TInstr, _pc: usize) {
}
const OP_TABLE: [fn(&mut TProgram, TInstr, usize); 4] = [op_set, op_add, op_mul, op_return];

impl TProgram {
    fn run(&mut self) {
        let instr = self.instrs[0];
        let op_byte = instr.op_byte;
        OP_TABLE[op_byte as usize](self, instr, 1);
    }
}
*/

fn main() {
    use BinaryOpF64::*;
    use TernaryOpF64::*;
    use Node::*;
    let mut sin = ProgramF64::new(vec![
        // let z = x*x
        Lettuce,
        BinaryOp(Mul), Input(0), Input(0),

        Lettuce, // let w = z*z
        BinaryOp(Mul), Local(0), Local(0),

        Lettuce, // let r = S1 + z*(S2 + z*S3) + z*w*(S4 + z*S5)
        TernaryOp(MulAdd),
        BinaryOp(Mul),
        Local(0),
        Local(1),
        TernaryOp(MulAdd),
        Local(0),
        Constant(5),
        Constant(4),
        TernaryOp(MulAdd),
        Local(0),
        TernaryOp(MulAdd),
        Local(0),
        Constant(3),
        Constant(2),
        Constant(1),

        Lettuce, // let v = z*x
        BinaryOp(Mul),
        Local(0),
        Input(0),

        // x + v*(S0 + z*r)
        TernaryOp(MulAdd),
        Local(3),
        TernaryOp(MulAdd),
        Local(0),
        Local(2),
        Constant(0),
        Input(0),
    ]).expect("failed to validate program");

    let count: usize = 1_000_000;
    let step: f64 = 1.0 / (count as f64);

    println!("sin: {:?}", sin);
    let constants = &[
        -1.66666666666666324348e-01, /* 0xBFC55555, 0x55555549 */
        8.33333333332248946124e-03, /* 0x3F811111, 0x1110F8A6 */
        -1.98412698298579493134e-04, /* 0xBF2A01A0, 0x19C161D5 */
        2.75573137070700676789e-06, /* 0x3EC71DE3, 0x57B1FE7D */
        -2.50507602534068634195e-08, /* 0xBE5AE5E6, 0x8A2B9CEB */
        1.58969099521155010221e-10, /* 0x3DE5D93A, 0x5ACFD57C */
    ];
    sin.set_constants(constants).expect("cannot set constants");
    use std::f64::consts::PI;

    let start = std::time::Instant::now();
    let mut sum = 0.0;
    let mut x = 0.0;
    for _ in 0..count {
        let inputs = &[x];
        sum += sin.eval(inputs).unwrap();
        x += step;
    }
    let elapsed = start.elapsed();
    println!("vm_1 time:   {:?} v={}", elapsed, sum);
    // println!("compiled:\n{}", compile(&sin.nodes).expect("cant compile"));

    /*
    // input=0, consts=1..6, locals=7..11
    let instrs = vec![
        Instr::Mul(7, 0, 0), // let z = x*x
        Instr::Mul(8, 7, 7), // let w = z*z
        // let r = S1 + z*(S2 + z*S3) + z*w*(S4 + z*S5)
        Instr::Mul(9, 7, 6),
        Instr::Add(9, 9, 5),
        Instr::Mul(9, 9, 8),
        Instr::Mul(9, 9, 7),
        Instr::Mul(10, 7, 4),
        Instr::Add(10, 10, 3),
        Instr::Mul(10, 10, 7),
        Instr::Add(10, 10, 2),
        Instr::Add(9, 9, 10),
        // let v = z*x;
        Instr::Mul(10, 7, 0),
        // x + v*(S0 + z*r)
        Instr::Mul(11, 7, 9),
        Instr::Add(11, 11, 1),
        Instr::Mul(11, 11, 10),
        Instr::Add(0, 0, 11),
    ];
    let memory = vec![
        0.0,
        -1.66666666666666324348e-01, /* 0xBFC55555, 0x55555549 */
        8.33333333332248946124e-03, /* 0x3F811111, 0x1110F8A6 */
        -1.98412698298579493134e-04, /* 0xBF2A01A0, 0x19C161D5 */
        2.75573137070700676789e-06, /* 0x3EC71DE3, 0x57B1FE7D */
        -2.50507602534068634195e-08, /* 0xBE5AE5E6, 0x8A2B9CEB */
        1.58969099521155010221e-10, /* 0x3DE5D93A, 0x5ACFD57C */
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
    ];
    let mut csin = CProgram { instrs, memory };
    let start = std::time::Instant::now();
    let mut sum = 0.0;
    let mut x = 0.0;
    for _ in 0..count {
        csin.memory[0] = x;
        csin.run();
        sum += csin.memory[0];
        x += step;
    }
    let elapsed = start.elapsed();
    println!("vm_2 time:   {:?} v={}", elapsed, sum);

    // input=0, consts=1..6, locals=7..11
    const ADD: u8 = 1;
    const MUL: u8 = 2;
    const RETURN: u8 = 3;
    let instrs = vec![
        TInstr::new(MUL, 7, 0, 0), // let z = x*x
        TInstr::new(MUL, 8, 7, 7), // let w = z*z
        // let r = S1 + z*(S2 + z*S3) + z*w*(S4 + z*S5)
        TInstr::new(MUL, 9, 7, 6),
        TInstr::new(ADD, 9, 9, 5),
        TInstr::new(MUL, 9, 9, 8),
        TInstr::new(MUL, 9, 9, 7),
        TInstr::new(MUL, 10, 7, 4),
        TInstr::new(ADD, 10, 10, 3),
        TInstr::new(MUL, 10, 10, 7),
        TInstr::new(ADD, 10, 10, 2),
        TInstr::new(ADD, 9, 9, 10),
        // let v = z*x;
        TInstr::new(MUL, 10, 7, 0),
        // x + v*(S0 + z*r)
        TInstr::new(MUL, 11, 7, 9),
        TInstr::new(ADD, 11, 11, 1),
        TInstr::new(MUL, 11, 11, 10),
        TInstr::new(ADD, 0, 0, 11),
        TInstr::new(RETURN, 0, 0, 0),
    ];
    let memory = vec![
        0.0,
        -1.66666666666666324348e-01, /* 0xBFC55555, 0x55555549 */
        8.33333333332248946124e-03, /* 0x3F811111, 0x1110F8A6 */
        -1.98412698298579493134e-04, /* 0xBF2A01A0, 0x19C161D5 */
        2.75573137070700676789e-06, /* 0x3EC71DE3, 0x57B1FE7D */
        -2.50507602534068634195e-08, /* 0xBE5AE5E6, 0x8A2B9CEB */
        1.58969099521155010221e-10, /* 0x3DE5D93A, 0x5ACFD57C */
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
    ];
    let mut tsin = TProgram { instrs, memory };
    let start = std::time::Instant::now();
    let mut sum = 0.0;
    let mut x = 0.0;
    for _ in 0..count {
        tsin.memory[0] = x;
        tsin.run();
        sum += tsin.memory[0];
        x += step;
    }
    let elapsed = start.elapsed();
    println!("vm_tco time: {:?} v={}", elapsed, sum);
    */

    /*
    let sin_jit = to_fn();
    println!("const_ptr = {:x?}", constants.as_ptr());
    let start = std::time::Instant::now();
    let mut sum = 0.0;
    let mut x = 0.0;
    for _ in 0..count {
        sum += sin_jit(x, constants.as_ptr());
        x += step;
    }
    let elapsed = start.elapsed();
    println!("jit time:    {:?} v={}", elapsed, sum);
    */

    let start = std::time::Instant::now();
    let mut sum = 0.0;
    let mut x = 0.0;
    for _ in 0..count {
        sum += k_sin(x);
        x += step;
    }
    let elapsed = start.elapsed();
    println!("native time: {:?} v={}", elapsed, sum);

    const LANES: usize = 32;
    let mut init = [0.0_f64; LANES];
    for ii in 0..LANES {
        init[ii] = (ii as f64) * step;
    }

    let mut x = SimdF64::from_array(init);
    let step = SimdF64::splat(step * (LANES as f64));
    let mut sum = 0.0;
    let start = std::time::Instant::now();
    for _ in (0..count).step_by(LANES) {
        sum += k_sin_simd(x).horizontal_sum();
        x += step;
    }
    let elapsed = start.elapsed();
    println!("simd time:   {:?} v={}", elapsed, sum);

    //println!("eval({:?}, c={:?}) = {:?}", inputs, constants, );
    //println!("sin(0.3)={}", (PI/3.0).sin());
}
/*
(let l0 (* i0 i0)
(let l1 (* l0 l0)
(let l2 (+ c2 (* l0 c3))
(let l3 (* l0 i0)
    (+ (+ i0 (* l3 (+ c0 (* l0 c1))))
       (* l3 (* l1 l2)))))))
*/
