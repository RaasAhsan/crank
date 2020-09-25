pub mod parser;

#[derive(Debug)]
pub enum Formula {
  Variable(String), // propositional variable
  Not(Box<Formula>), // logical negation
  Or(Box<Formula>, Box<Formula>), // logical disjunction
  And(Box<Formula>, Box<Formula>), // logical conjunction
  Conditional(Box<Formula>, Box<Formula>), // logical conditional
  Biconditional(Box<Formula>, Box<Formula>) // logical biconditional
}
