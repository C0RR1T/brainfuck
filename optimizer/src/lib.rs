use std::mem::transmute;
use std::slice::Iter;
use std::vec::IntoIter;

use peekmore::{PeekMore, PeekMoreIterator};

use parser::Instruction;
use parser::Instruction::{Clear, InfiniteLoop, Loop};

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
            match ins {
                Instruction::Loop(loop_instructions) => {}
                Instruction::Add(_) => {}
                Instruction::Subtract(_) => {}
                Instruction::Left(_) => {}
                Instruction::Right(_) => {}
                _ => {}
            }
        }

        instructions
    }

    fn consume_iter_elements(&self, iter: &mut PeekMoreIterator<Iter<Instruction>>, amount: usize) {
        for _ in 0..amount {
            iter.next();
        }
    }

    fn optimize_loops(&mut self, loop_instructions: Vec<Instruction>) -> Instruction {
        let mut iter = loop_instructions.iter().peekmore();
        let mut new_instructions = Vec::new();
        let mut is_pure = true;
        let mut amount_plus = 0;
        let mut amount_minus = 0;
        while let Some(i) = iter.next() {
            match i {
                Instruction::Add(_) => {
                    let amount =
                        self.combine_instructions_vec(&loop_instructions, Instruction::Add(1));
                    amount_plus += amount;
                    self.consume_iter_elements(&mut iter, amount as usize);
                    new_instructions.push(Instruction::Add(amount))
                }
                Instruction::Subtract(_) => {
                    let amount =
                        self.combine_instructions_vec(&loop_instructions, Instruction::Add(1));
                    amount_minus += amount;
                    new_instructions.push(Instruction::Subtract(amount))
                }
                Instruction::Left(_) => {
                    let amount =
                        self.combine_instructions_vec(&loop_instructions, Instruction::Left(1));
                }
                Instruction::Right(_) => {}
                Instruction::Clear => {}
                Instruction::InfiniteLoop => return InfiniteLoop,
                Instruction::Skip => {}
                other => {
                    is_pure = false;
                    new_instructions.push(other.clone());
                }
            }
        }
        if is_pure {
            return if amount_minus == amount_plus {
                InfiniteLoop
            } else {
                Clear
            };
        }
        Loop(new_instructions)
    }

    fn combine_instructions_vec(
        &self,
        instructions: &Vec<Instruction>,
        ins_to_match: Instruction,
    ) -> u8 {
        let mut iter = instructions.iter().peekmore();
        if let Some(i) = iter.peek() {
            if **i == ins_to_match {
                let mut amount = 2u8;
                while let Some(i) = iter.peek_nth(amount as usize) {
                    if **i == ins_to_match && amount < u8::MAX {
                        amount += 1
                    } else {
                        break;
                    }
                }
                return amount;
            }
        }
        1
    }

    fn combine_instructions(&mut self, ins_to_match: Instruction, iter: impl Iterator) -> usize {
        if let Some(i) = self.peek() {
            if *i == ins_to_match {
                let mut amount = 2;
                while let Some(i) = self.peek_nth(amount) {
                    if *i == ins_to_match {
                        amount += 1
                    } else {
                        break;
                    }
                }
                return amount;
            }
        }
        1
    }

    fn peek(&mut self) -> Option<&Instruction> {
        self.instructions.peek()
    }

    fn next(&mut self) -> Option<Instruction> {
        self.instructions.next()
    }

    fn consume_items(&mut self, amount: usize) {
        for _ in 0..amount {
            self.next()
        }
    }

    fn peek_nth(&mut self, amount: usize) -> Option<&Instruction> {
        self.instructions.peek_nth(amount)
    }
}
