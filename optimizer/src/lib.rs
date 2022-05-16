use std::cmp::Ordering;
use std::slice::Iter;
use std::vec::IntoIter;

use peekmore::{PeekMore, PeekMoreIterator};

use parser::Instruction;
use parser::Instruction::{Clear, Loop};

struct Optimizer {
    instructions: PeekMoreIterator<IntoIter<Instruction>>,
}

impl Optimizer {
    pub fn new(instructions: Vec<Instruction>) -> Self {
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

        instructions
    }

    fn optimize_instruction(&mut self, ins: Instruction) -> Option<Instruction> {
        match ins {
            Loop(loop_instructions) => Self::optimize_loops(&loop_instructions),
            Instruction::Add(_) => Some(Instruction::Add(self.combine_tokens(Instruction::Add(1)))),
            Instruction::Subtract(_) => Some(Instruction::Add(
                self.combine_tokens(Instruction::Subtract(1)),
            )),
            Instruction::Left(_) => {
                Some(Instruction::Add(self.combine_tokens(Instruction::Left(1))))
            }
            Instruction::Right(_) => {
                Some(Instruction::Add(self.combine_tokens(Instruction::Right(1))))
            }
            _ => None,
        }
    }

    fn optimize_loops(ins: &Vec<Instruction>) -> Option<Instruction> {
        let mut optimized = Self::summarize_tokens(ins);
        let mut iter = optimized.iter().peekmore();
        let mut new_instructions: Vec<Instruction> = Vec::new();

        if let Some(ins) = iter.next() {
            match ins {
                Instruction::Add(amount) | Instruction::Subtract(amount)
                    if *amount == 1 && optimized.len() == 1 =>
                {
                    return Some(Clear)
                }
                Loop(ins) => {
                    if let Some(instruction) = Self::optimize_loops(ins) {
                        match instruction {
                            Clear if optimized.len() == 1 => return Some(Clear),
                            token => new_instructions.push(token),
                        }
                    } else if optimized.len() == 1 {
                        return None;
                    }
                }
                Instruction::Left(_) => {}
                Instruction::Right(_) => {}
                Clear => {}
                Instruction::Output => {}
                Instruction::Input => {}
                Instruction::Multiply { .. } => {}
                Instruction::Divide { .. } => {}
                instruction => new_instructions.push(instruction.clone()),
            }

            return Some(Loop(new_instructions));
        }

        None
    }

    fn summarize_tokens(instructions: &Vec<Instruction>) -> Vec<Instruction> {
        let mut iter = instructions.iter().peekable();

        let mut new_instructions: Vec<Instruction> = Vec::new();

        while let Some(ins) = iter.next() {
            if let Some(next_token) = iter.peek() {
                match ins {
                    Instruction::Add(plus) => {
                        if let Some(new_add) =
                            Self::summarize_two_tokens(&Instruction::Add(*plus), *next_token)
                        {
                            new_instructions.push(new_add);
                        }
                    }
                    Instruction::Subtract(minus) => {
                        if let Some(new_add) =
                            Self::summarize_two_tokens(*next_token, &Instruction::Subtract(*minus))
                        {
                            new_instructions.push(new_add);
                        }
                    }
                    Instruction::Left(amount_left) => {
                        if let Some(new_left) = Self::summarize_two_tokens(
                            &Instruction::Left(*amount_left),
                            *next_token,
                        ) {
                            new_instructions.push(new_left);
                        }
                    }
                    Instruction::Right(amount_right) => {
                        if let Some(new_right) = Self::summarize_two_tokens(
                            &Instruction::Right(*amount_right),
                            *next_token,
                        ) {
                            new_instructions.push(new_right)
                        }
                    }
                    _ => {}
                }
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
                None
            }
            Instruction::Subtract(minus) => {
                if let Instruction::Add(plus) = second {
                    return Self::compare_plus_minus(plus, minus);
                }
                None
            }
            Instruction::Left(left) => {
                if let Instruction::Right(right) = second {
                    return Self::compare_left_right(left, right);
                }
                None
            }
            Instruction::Right(right) => {
                if let Instruction::Left(left) = second {
                    return Self::compare_left_right(left, right);
                }
                None
            }
            _ => None,
        }
    }

    fn compare_plus_minus(plus: &u8, minus: &u8) -> Option<Instruction> {
        match plus.cmp(minus) {
            Ordering::Less => Some(Instruction::Subtract(minus - plus)),
            Ordering::Equal => None,
            Ordering::Greater => Some(Instruction::Add(plus - minus)),
        }
    }

    fn compare_left_right(left: &u8, right: &u8) -> Option<Instruction> {
        match left.cmp(right) {
            Ordering::Less => Some(Instruction::Right(right - left)),
            Ordering::Equal => None,
            Ordering::Greater => Some(Instruction::Left(left - right)),
        }
    }

    fn combine_tokens(&mut self, instruction_to_cmp: Instruction) -> u8 {
        let mut amount: u8 = 1;

        while let Some(instruction) = self.peek() {
            if instruction_to_cmp == *instruction {
                amount += 1;
            }

            if amount == u8::MAX {
                break;
            }
        }

        if amount > 1 {
            self.consume_items((amount - 1) as usize);
        }

        amount
    }

    fn consume_iter_elements(&self, iter: &mut PeekMoreIterator<Iter<Instruction>>, amount: usize) {
        for _ in 0..amount {
            iter.next();
        }
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

    fn peek_nth(&mut self, amount: usize) -> Option<&Instruction> {
        self.instructions.peek_nth(amount)
    }
}