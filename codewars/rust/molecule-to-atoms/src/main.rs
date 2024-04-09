use thiserror::Error;

pub type Atom = (String, usize);
pub type Molecule = Vec<Atom>;

#[derive(Error, Debug)]
pub enum ParseError {
    // variants
}

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    todo!();
}

fn main() {
    todo!();
}
