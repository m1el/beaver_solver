use core::fmt;
use crate::binary_op::{BinaryOp, BinaryOpF32, BinaryOpF64, BinaryOpI32};
use crate::ternary_op::{TernaryOp, TernaryOpF32, TernaryOpF64, TernaryOpI32};

#[derive(Debug, Clone, Copy)]
pub enum Node<BOP, TOP> {
    Input(u8),
    Local(u8),
    Constant(u8),
    Lettuce,
    BinaryOp(BOP),
    TernaryOp(TOP),
}

pub type NodeF32 = Node<BinaryOpF32, TernaryOpF32>;
pub type NodeF64 = Node<BinaryOpF64, TernaryOpF64>;
pub type NodeI32 = Node<BinaryOpI32, TernaryOpI32>;

#[derive(Debug, PartialEq, Eq)]
pub enum ProgramError {
    InvalidTree,
    NonExistentConstant,
    NonExistentInput,
    NonExistentLocal,
    TooFewInputs,
    TooFewConstants,
    TooFewNodes,
    TooFewRegisters,
    RegisterDoubleFree,
    TooManyNodes,
    InvalidLocal,
}

pub fn validate<BOP: Copy, TOP: Copy>(nodes: &[Node<BOP, TOP>]) -> Result<(), ProgramError> {
    let mut depth = 1;
    let mut lets = 0;

    for node in nodes {
        match node {
            Node::Constant(_) | Node::Input(_) => {
                depth -= 1;
            }
            &Node::Local(index) => {
                if (index as usize) >= lets {
                    return Err(ProgramError::InvalidLocal);
                }
                depth -= 1;
            }
            Node::Lettuce => {
                lets += 1;
                depth += 1;
            }
            Node::BinaryOp(_) => {
                depth += 1;
            }
            Node::TernaryOp(_) => {
                depth += 2;
            }
        }

        if depth < 0 {
            return Err(ProgramError::TooManyNodes);
        }
    }

    if depth == 0 {
        Ok(())
    } else {
        Err(ProgramError::TooFewNodes)
    }
}

pub fn count_vars<BOP: Copy, TOP: Copy>(nodes: &[Node<BOP, TOP>]) -> (usize, usize, usize) {
    let mut input_count = 0;
    let mut local_count = 0;
    let mut const_count = 0;
    for node in nodes {
        match node {
            &Node::Input(index) => {
                input_count = input_count.max(index as usize);
            }
            &Node::Local(index) => {
                local_count = local_count.max(index as usize);
            }
            &Node::Constant(index) => {
                const_count = local_count.max(index as usize);
            }
            Node::Lettuce | Node::BinaryOp(_) | Node::TernaryOp(_) => {}
        }
    }
    return (input_count, local_count, const_count)
}

pub struct Program<T, BOP, TOP>
    where BOP: BinaryOp<T> + Copy,
          TOP: TernaryOp<T> + Copy
{
    pub nodes: Vec<Node<BOP, TOP>>,
    locals: Vec<T>,
    constants: Vec<T>,
    position: usize,
    const_count: usize,
    local_count: usize,
    input_count: usize,
}

pub type ProgramF32 = Program<f32, BinaryOpF32, TernaryOpF32>;
pub type ProgramF64 = Program<f64, BinaryOpF64, TernaryOpF64>;
pub type ProgramI32 = Program<i32, BinaryOpI32, TernaryOpI32>;

impl<T, BOP, TOP> Program<T, BOP, TOP>
    where T: Copy,
          BOP: Copy + BinaryOp<T>,
          TOP: Copy + TernaryOp<T>
{
    pub fn new(nodes: Vec<Node<BOP, TOP>>) -> Result<Self, ProgramError> {
        validate(&nodes)?;
        let (input_count, local_count, const_count) = count_vars(&nodes);

        Ok(Self {
            nodes,
            locals: Vec::new(),
            constants: Vec::new(),
            position: 0,
            const_count,
            local_count,
            input_count,
        })
    }

    pub fn set_constants(&mut self, constants: &[T]) -> Result<(), ProgramError> {
        if constants.len() < self.const_count {
            return Err(ProgramError::TooFewConstants);
        }
        self.constants.clear();
        self.constants.extend(constants.iter().copied());
        Ok(())
    }

    pub fn eval(&mut self, inputs: &[T]) -> Result<T, ProgramError> {
        if inputs.len() < self.input_count {
            return Err(ProgramError::TooFewInputs);
        }
        if self.constants.len() < self.const_count {
            return Err(ProgramError::TooFewConstants);
        }

        self.position = 0;
        self.locals.clear();
        self.locals.reserve(self.local_count);
        self.eval_inner(inputs)
    }

    fn eval_inner(&mut self, inputs: &[T]) -> Result<T, ProgramError> {
        let node = *self.nodes.get(self.position)
            .ok_or(ProgramError::InvalidTree)?;
        self.position += 1;

        match node {
            Node::Input(index) => {
                inputs.get(index as usize).copied()
                    .ok_or(ProgramError::NonExistentInput)
            }
            Node::Local(index) => {
                self.locals.get(index as usize).copied()
                    .ok_or(ProgramError::NonExistentLocal)
            }
            Node::Constant(index) => {
                self.constants.get(index as usize).copied()
                    .ok_or(ProgramError::NonExistentConstant)
            }
            Node::Lettuce => {
                let value = self.eval_inner(inputs)?;
                self.locals.push(value);
                self.eval_inner(inputs)
            }
            Node::BinaryOp(op) => {
                let lhs = self.eval_inner(inputs)?;
                let rhs = self.eval_inner(inputs)?;
                Ok(op.run(lhs, rhs))
            }
            Node::TernaryOp(op) => {
                let a = self.eval_inner(inputs)?;
                let b = self.eval_inner(inputs)?;
                let c = self.eval_inner(inputs)?;
                Ok(op.run(a, b, c))
            }
        }
    }
}

impl<T, BOP, TOP> fmt::Debug for Program<T, BOP, TOP>
    where T: Copy,
          BOP: Copy + fmt::Debug + BinaryOp<T>,
          TOP: Copy + fmt::Debug + TernaryOp<T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut stack = vec![];
        let mut left = 1;
        let mut lettuce = 0;

        for (pos, node) in self.nodes.iter().enumerate() {
            let mut do_reduce = false;
            //println!("node {:?}, stack {:?}, left {}", node, stack, left);
            match node {
                Node::TernaryOp(op) => {
                    write!(f, "({}", op.repr())?;
                    if pos != 0 {
                        stack.push(left);
                    }
                    left = 3;
                }
                Node::BinaryOp(op) => {
                    write!(f, "({}", op.repr())?;
                    if pos != 0 {
                        stack.push(left);
                    }
                    left = 2;
                }
                Node::Lettuce => {
                    write!(f, "(let l{}", lettuce)?;
                    if pos != 0 {
                        stack.push(left);
                    }
                    left = 2;
                    lettuce += 1;
                }
                Node::Local(index) => {
                    write!(f, "l{}", index)?;
                    do_reduce = true;
                }
                Node::Constant(index) => {
                    write!(f, "c{}", index)?;
                    do_reduce = true;
                }
                Node::Input(index) => {
                    write!(f, "a{}", index)?;
                    do_reduce = true;
                }
            }

            if do_reduce {
                loop {
                    left -= 1;

                    if left == 0 {
                        f.write_str(")")?;
                        if let Some(last) = stack.pop() {
                            left = last
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }

            if left > 0 {
                f.write_str(" ")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        ProgramF32::new(vec![ Node::Input(0) ])
            .unwrap();

        assert_eq!(
            ProgramF32::new(vec![ Node::BinaryOp(BinaryOpF32::Add) ]).err(),
            Some(ProgramError::TooFewNodes)
        );

        assert_eq!(
            ProgramF32::new(vec![
                Node::BinaryOp(BinaryOpF32::Add),
                Node::Input(0),
            ]).err(),
            Some(ProgramError::TooFewNodes)
        );

        ProgramF32::new(vec![
            Node::BinaryOp(BinaryOpF32::Add),
            Node::Input(0),
            Node::Input(0),
        ]).unwrap();

        ProgramF32::new(vec![
            Node::BinaryOp(BinaryOpF32::Add),
            Node::Input(0),
            Node::BinaryOp(BinaryOpF32::Add),
            Node::Input(0),
            Node::Input(0),
        ]).unwrap();
    }

    #[test]
    fn tree_format() {
        let program = ProgramF32::new(vec![
            Node::Lettuce,
            Node::BinaryOp(BinaryOpF32::Add),
            Node::Input(1),
            Node::BinaryOp(BinaryOpF32::Sub),
            Node::Input(0),
            Node::Input(2),
            Node::Lettuce,
            Node::Input(0),
            Node::BinaryOp(BinaryOpF32::Mul),
            Node::Local(1),
            Node::BinaryOp(BinaryOpF32::Div),
            Node::Input(2),
            Node::Local(0),
        ]).unwrap();

        assert_eq!(
            format!("{:?}", program),
            "(let l0 (+ a1 (- a0 a2)) (let l1 a0 (* l1 (/ a2 l0))))"
        );
    }

    #[test]
    fn tree_format_i32() {
        let program = ProgramI32::new(vec![
            Node::Lettuce,
            Node::BinaryOp(BinaryOpI32::Add),
            Node::Input(1),
            Node::BinaryOp(BinaryOpI32::Sub),
            Node::Input(0),
            Node::Input(2),
            Node::Lettuce,
            Node::Input(0),
            Node::BinaryOp(BinaryOpI32::Mul),
            Node::Local(1),
            Node::BinaryOp(BinaryOpI32::Div),
            Node::Input(2),
            Node::BinaryOp(BinaryOpI32::Xor),
            Node::Input(2),
            Node::BinaryOp(BinaryOpI32::And),
            Node::Input(2),
            Node::BinaryOp(BinaryOpI32::Or),
            Node::Input(2),
            Node::Constant(0),
        ]).unwrap();

        assert_eq!(
            format!("{:?}", program),
            "(let l0 (+ a1 (- a0 a2)) (let l1 a0 (* l1 (/ a2 (^ a2 (& a2 (| a2 c0)))))))"
        );
    }

    #[test]
    fn eval_simple() {
        let mut program = ProgramF32::new(vec![
            Node::Lettuce,
            Node::BinaryOp(BinaryOpF32::Add),
            Node::Input(1),
            Node::Input(2),
            Node::BinaryOp(BinaryOpF32::Mul),
            Node::Input(0),
            Node::Local(0),
        ]).unwrap();

        assert_eq!(program.eval(&[2.0, 3.0, 4.0]).ok(), Some(14.0_f32));
    }
}
