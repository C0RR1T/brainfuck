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
            Loop(loop_instructions) => None,
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

    fn summarize_tokens(instructions: Vec<Instruction>) -> Vec<Instruction> {
        let mut iter = instructions.iter().peekable();

        while let Some(ins) = iter.next() {
            match ins {
                Instruction::Add(plus) => {
                    if let Some(Instruction::Subtract(minus)) = iter.peek() {
                        if plus > minus {
                            iter.next();
                        } else {
                            continue;
                        }
                    }
                }
                Instruction::Subtract(minus) => {
                    if let Some(Instruction::Add(plus)) = iter.peek() {
                        if plus <= minus {
                            iter.next();
                        } else {
                            continue;
                        }
                    }
                }
                Instruction::Left(amount_left) => {}
                Instruction::Right(amount_right) => {}
                Clear => {}
                _ => {}
            }
        }

        iter.cloned().collect::<Vec<_>>()
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
