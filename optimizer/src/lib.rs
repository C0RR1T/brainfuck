use std::cmp::Ordering;
use std::vec::IntoIter;

use peekmore::PeekMore;

use parser::Instruction;

pub struct Optimizer<'a> {
    instructions: &'a [Instruction],
}

impl<'a> Optimizer<'a> {
    pub fn new(instructions: &'a [Instruction]) -> Self {
        #[cfg(debug_assertions)]
        println!(
            "Original:  {:?}",
            instructions
                .iter()
                .map(ToString::to_string)
                .collect::<String>()
        );

        Optimizer {
            instructions,
        }
    }

    pub fn optimize(&mut self) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        while let Some(instruction) = self.next() {
            match instruction {
                Instruction::Add(_) => {
                    let amt = self.combine_tokens(&Instruction::Add(1));
                    self.consume_elements(amt - 1);
                    instructions.push(Instruction::Add(amt as isize));
                },
                Instruction::Subtract(_) => {
                    let amt = self.combine_tokens(&Instruction::Subtract(1));
                    self.consume_elements(amt - 1);
                    instructions.push(Instruction::Subtract(amt as isize));
                },
                Instruction::Left(_) => {
                    let amt = self.combine_tokens(&Instruction::Left(1));
                    self.consume_elements(amt - 1);
                    instructions.push(Instruction::Left(amt as isize));
                },
                Instruction::Right(_) => {
                    let amt = self.combine_tokens(&Instruction::Right(1));
                    self.consume_elements(amt - 1);
                    instructions.push(Instruction::Right(amt as isize));
                },

                _ => instructions.push(instruction.clone())
            }
        }

        #[cfg(debug_assertions)]
        println!(
            "Optimized: {:?}",
            instructions
                .iter()
                .map(ToString::to_string)
                .collect::<String>()
        );

        instructions
    }

    fn combine_tokens(&self, ins: &Instruction) -> usize {
        self.instructions.into_iter().position(|second_ins| second_ins != ins).map(|val| val + 1).unwrap_or(1)
    }

    fn next(&mut self) -> Option<&'a Instruction> {
        if self.instructions.len() == 0 {
            return None;
        }

        let (element, rest) = self.instructions.split_at(1);
        self.instructions = rest;

        element.get(0)
    }

    fn peek(&self) -> Option<&'a Instruction> {
        self.instructions.get(1)
    }

    fn peek_nth(&self, index: usize) -> Option<&'a Instruction> {
        self.instructions.get(index)
    }

    fn consume_elements(&mut self, to: usize) {
        let (_, items) = self.instructions.split_at(to);
        self.instructions = items;
    }
}


