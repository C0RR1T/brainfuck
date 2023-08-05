use std::cmp::Ordering;

use std::vec::IntoIter;

use peekmore::{PeekMore, PeekMoreIterator};

use parser::Instruction;
use parser::Instruction::{Clear, Loop, Multiply};

pub struct Optimizer {
    instructions: PeekMoreIterator<IntoIter<Instruction>>,
}

impl Optimizer {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        println!(
            "Original:  {:?}",
            instructions
                .iter()
                .map(ToString::to_string)
                .collect::<String>()
        );
        Optimizer {
            instructions: instructions.into_iter().peekmore(),
        }
    }

    pub fn optimize(&mut self) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        while let Some(ins) = self.next() {
            if let Some(new_instruction) = self.optimize_instruction(ins) {
                instructions.push(new_instruction)
            }
        }

        println!(
            "Optimized: {:?}",
            instructions
                .iter()
                .map(ToString::to_string)
                .collect::<String>()
        );

        Self::finish_optimization(instructions.into_iter())
    }

    fn finish_optimization(ins: IntoIter<Instruction>) -> Vec<Instruction> {
        let mut new_instructions = Vec::new();
        let mut iter = ins.peekable();
        while let Some(instruction) = iter.next() {
            if let Some(next_ins) = iter.peek() {
                if let Some(new_ins) = Self::summarize_two_tokens(&instruction, next_ins) {
                    new_instructions.push(new_ins);
                    iter.next();
                }
            }
        }

        new_instructions
    }

    fn optimize_instruction(&mut self, ins: Instruction) -> Option<Instruction> {
        match ins {
            Loop(loop_instructions) => Self::optimize_loops(&loop_instructions),
            Clear => {
                if let Some(Loop(_) | Multiply { .. } | Instruction::Divide { .. } | Clear) =
                    self.peek()
                {
                    self.next();
                }
                Some(Clear)
            }
            Instruction::Left(_)
            | Instruction::Right(_)
            | Instruction::Add(_)
            | Instruction::Subtract(_) => Some(Self::combine_ins(self.combine_tokens(&ins), &ins)),
            token => Some(token),
        }
    }

    fn combine_ins(amt: isize, ins: &Instruction) -> Instruction {
        match ins {
            Instruction::Add(_) => Instruction::Add(amt),
            Instruction::Subtract(_) => Instruction::Subtract(amt),
            Instruction::Left(_) => Instruction::Left(amt),
            Instruction::Right(_) => Instruction::Right(amt),
            _ => unreachable!(),
        }
    }

    fn optimize_loops(ins: &[Instruction]) -> Option<Instruction> {
        let optimized = Self::summarize_tokens(ins.to_vec());
        let mut iter = optimized.iter().peekmore();
        let mut new_instructions: Vec<Instruction> = Vec::new();

        while let Some(ins) = iter.next() {
            match ins {
                Instruction::Add(amount) | Instruction::Subtract(amount)
                    if *amount == 1 && optimized.len() == 1 =>
                {
                    return Some(Clear)
                }
                Clear if optimized.len() == 1 => return Some(Clear),
                Instruction::Left(amt_left) => {
                    if let Some(Instruction::Add(plus)) = iter.peek() {
                        if let Some(Instruction::Right(amt_right)) = iter.peek() {
                            if amt_left == amt_right {
                                if let Some(Instruction::Subtract(1)) = iter.peek() {
                                    return Some(Multiply {
                                        mc: *plus,
                                        offset: -*amt_left,
                                    });
                                }
                            }
                        }
                    }
                }
                Instruction::Right(amt_right) => {
                    if let Some(Instruction::Add(plus)) = iter.peek() {
                        if let Some(Instruction::Left(amt_left)) = iter.peek() {
                            if amt_left == amt_right {
                                if let Some(Instruction::Subtract(1)) = iter.peek() {
                                    return Some(Multiply {
                                        mc: *plus,
                                        offset: *amt_right,
                                    });
                                }
                            }
                        }
                    }
                }
                instruction => new_instructions.push(instruction.clone()),
            }
        }

        if !new_instructions.is_empty() {
            Some(Loop(new_instructions))
        } else {
            None
        }
    }

    fn summarize_tokens(instructions: Vec<Instruction>) -> Vec<Instruction> {
        let mut iter = instructions.into_iter().peekmore();

        let mut new_instructions: Vec<Instruction> = Vec::new();

        while let Some(ins) = iter.next() {
            match ins {
                Instruction::Left(_)
                | Instruction::Right(_)
                | Instruction::Add(_)
                | Instruction::Subtract(_) => new_instructions.push(Self::combine_ins(
                    Self::combine_tokens_iter(&mut iter, &ins),
                    &ins,
                )),
                token => new_instructions.push(token),
            }
        }
        new_instructions
    }

    fn summarize_two_tokens(first: &Instruction, second: &Instruction) -> Option<Instruction> {
        match first {
            Instruction::Add(plus) => {
                if let Instruction::Subtract(minus) = second {
                    return Self::compare_plus_minus(plus, minus);
                }
            }
            Instruction::Subtract(minus) => {
                if let Instruction::Add(plus) = second {
                    return Self::compare_plus_minus(plus, minus);
                }
            }
            Instruction::Left(left) => {
                if let Instruction::Right(right) = second {
                    return Self::compare_left_right(left, right);
                }
            }
            Instruction::Right(right) => {
                if let Instruction::Left(left) = second {
                    return Self::compare_left_right(left, right);
                }
            }
            _ => {}
        }
        None
    }

    fn compare_plus_minus(plus: &isize, minus: &isize) -> Option<Instruction> {
        match plus.cmp(minus) {
            Ordering::Less => Some(Instruction::Subtract(minus - plus)),
            Ordering::Equal => None,
            Ordering::Greater => Some(Instruction::Add(plus - minus)),
        }
    }

    fn compare_left_right(left: &isize, right: &isize) -> Option<Instruction> {
        match left.cmp(right) {
            Ordering::Less => Some(Instruction::Right(right - left)),
            Ordering::Equal => None,
            Ordering::Greater => Some(Instruction::Left(left - right)),
        }
    }

    fn combine_tokens(&mut self, instruction_to_cmp: &Instruction) -> isize {
        Self::combine_tokens_iter(&mut self.instructions, instruction_to_cmp)
    }

    fn combine_tokens_iter(
        ins: &mut PeekMoreIterator<IntoIter<Instruction>>,
        instruction_to_cmp: &Instruction,
    ) -> isize {
        let mut amount = 1;

        while let Some(instruction) = ins.peek_nth((amount) as usize) {
            if *instruction_to_cmp == *instruction {
                amount += 1;
            } else {
                break;
            }

            if amount == isize::MAX {
                break;
            }
        }

        for _ in 0..(amount - 1) {
            ins.next();
        }

        amount
    }

    fn peek(&mut self) -> Option<&Instruction> {
        self.instructions.peek()
    }

    fn next(&mut self) -> Option<Instruction> {
        self.instructions.next()
    }

    fn consume_items(&mut self, amount: usize) {
        for _ in 0..amount {
            self.next();
        }
    }
}
