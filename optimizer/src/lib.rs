use std::cmp::Ordering;

use std::vec::IntoIter;

use peekmore::{PeekMore, PeekMoreIterator};

use parser::Instruction;
use parser::Instruction::{Clear, Loop, Multiply};

pub struct Optimizer<'a> {
    instructions: &'a [Instruction],
    index: usize,
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
            instructions: &instructions[..],
            index: 0,
        }
    }

    pub fn optimize(&mut self) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        while let Some(ins) = self.instructions.get(self.index) {
            if let Some(new_instruction) = self.optimize_instruction(ins.clone()) {
                instructions.push(new_instruction)
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
                    self.instructions.get(self.index + 1)
                {
                    self.index += 1;
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
        let optimized = Self::summarize_tokens(ins);

        match &optimized[..] {
            // Clear operation
            [Instruction::Add(amount)] | [Instruction::Subtract(amount)] if *amount == 1 => {
                return Some(Clear);
            }
            // Multiply to the left operation
            [Instruction::Left(amt_left), Instruction::Add(plus), Instruction::Right(amt_right), Instruction::Subtract(1)]
            | [Instruction::Right(amt_right), Instruction::Add(plus), Instruction::Left(amt_left), Instruction::Subtract(1)]
                if amt_left == amt_right =>
            {
                return Some(Multiply {
                    mc: *plus,
                    offset: -(*amt_left as isize),
                });
            }
            // Division operation
            _ => {
                if ins.is_empty() {
                    return None;
                }

                return Some(Loop(ins.to_vec()));
            }
        };
    }

    fn summarize_tokens(instructions: &[Instruction]) -> Vec<Instruction> {
        let mut iter = instructions.iter();

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
                token => new_instructions.push(token.clone()),
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
        Self::combine_tokens_iter(&mut self.instructions.iter(), instruction_to_cmp)
    }

    fn combine_tokens_iter(
        ins: &mut std::slice::Iter<'_, Instruction>,
        instruction_to_cmp: &Instruction,
    ) -> isize {
        let mut amount = 1;

        while let Some(instruction) = ins.peekmore().peek_nth((amount) as usize) {
            if *instruction_to_cmp == **instruction {
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
}
