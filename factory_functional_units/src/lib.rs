use std::cmp::PartialEq;
use std::marker::Copy;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    pub fn inverse(&self) -> Orientation {
        match self {
            Orientation::North => Orientation::South,
            Orientation::East => Orientation::West,
            Orientation::South => Orientation::North,
            Orientation::West => Orientation::East,
        }
    }
}

impl Display for Orientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Orientation::North => write!(f, "North"),
            Orientation::East => write!(f, "East"),
            Orientation::South => write!(f, "South"),
            Orientation::West => write!(f, "West"),
        }
    }
}

pub struct Conveyer {
    name: String,
    current_orientation: Orientation,
    has_paper: bool,
}

impl Conveyer {
    pub fn new(name: &str) -> Conveyer {
        Conveyer {
            name: String::from(name),
            current_orientation: Orientation::East,
            has_paper: false,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn orientation(&self) -> &Orientation {
        &self.current_orientation
    }

    pub fn has_paper(&self) -> bool {
        self.has_paper
    }

    pub fn turn_to(&mut self, new_orientation: Orientation) {
        if self.current_orientation != new_orientation {
            println!("Conveyer '{}'- turn_to {}", self.name, new_orientation);
            self.current_orientation = new_orientation;
        }
    }

    pub fn push(&mut self) -> Result<(), String> {
        if !self.has_paper {
            Err(String::from("Conveyer empty"))
        } else {
            println!("Conveyer '{}' - push", self.name);
            self.has_paper = false;
            Ok(())
        }
    }

    pub fn pull(&mut self) -> Result<(), String> {
        if self.has_paper {
            Err(String::from("Conveyer full"))
        } else {
            println!("Conveyer '{}' - pull", self.name);
            self.has_paper = true;
            Ok(())
        }
    }
}

pub struct Plotter {
    name: String,
    has_paper: bool,
}

impl Plotter {
    pub fn new(name: &str) -> Plotter {
        Plotter {
            name: String::from(name),
            has_paper: false,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn has_paper(&self) -> bool {
        self.has_paper
    }

    pub fn plot(&self) -> Result<(), String> {
        if !self.has_paper {
            Err(String::from("No paper"))
        } else {
            println!("Plotter '{}'- plot", self.name);
            Ok(())
        }
    }

    pub fn push_out(&mut self) -> Result<(), String> {
        if !self.has_paper {
            Err(String::from("No paper"))
        } else {
            println!("Plotter '{}'- push_out", self.name);
            self.has_paper = false;
            Ok(())
        }
    }

    pub fn pull_in(&mut self) -> Result<(), String> {
        if !self.has_paper {
            Err(String::from("Plotter full"))
        } else {
            println!("Plotter '{}'- push_in", self.name);
            self.has_paper = true;
            Ok(())
        }
    }
}

pub struct InputStack {
    name: String,
    paper_count: u32,
}

impl InputStack {
    pub fn new(name: &str, start_count: u32) -> InputStack {
        InputStack {
            name: String::from(name),
            paper_count: start_count,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn paper_count(&self) -> u32 {
        self.paper_count
    }

    pub fn push_out(&mut self) -> Result<(), String> {
        if self.paper_count == 0 {
            Err(String::from("No paper left"))
        } else {
            println!("InputStack '{}'- push_out", self.name);
            self.paper_count -= 1;
            Ok(())
        }
    }
}

pub struct OutputStack {
    name: String,
    paper_count: u32,
}

impl OutputStack {
    pub fn new(name: &str) -> OutputStack {
        OutputStack {
            name: String::from(name),
            paper_count: 0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn paper_count(&self) -> u32 {
        self.paper_count
    }

    pub fn pull_in(&mut self) {
        println!("OutputStack '{}'- pull_in", self.name);
        self.paper_count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod orientation {
        use super::*;

        #[test]
        fn inverse() {
            assert_eq!(Orientation::North.inverse(), Orientation::South);
            assert_eq!(Orientation::East.inverse(), Orientation::West);
            assert_eq!(Orientation::South.inverse(), Orientation::North);
            assert_eq!(Orientation::West.inverse(), Orientation::East);
        }
    }

    mod conveyer {
        use super::*;

        #[test]
        fn new() {
            let name = "Left";
            let conv = Conveyer::new(name);
            assert_eq!(name, conv.name());
            assert_eq!(&Orientation::East, conv.orientation());
        }

        #[test]
        fn turn_to() {
            let mut conv = Conveyer::new("Left");
            conv.turn_to(Orientation::North);
            assert_eq!(&Orientation::North, conv.orientation());

            conv.turn_to(Orientation::East);
            assert_eq!(&Orientation::East, conv.orientation());

            conv.turn_to(Orientation::South);
            assert_eq!(&Orientation::South, conv.orientation());

            conv.turn_to(Orientation::West);
            assert_eq!(&Orientation::West, conv.orientation());
        }

        #[test]
        fn pull_and_push() -> Result<(), String> {
            let mut conv = Conveyer::new("Left");
            conv.pull()?;
            assert_eq!(true, conv.has_paper);
            assert_eq!(&Orientation::East, conv.orientation());

            conv.push()?;
            assert_eq!(false, conv.has_paper);
            assert_eq!(&Orientation::East, conv.orientation());

            Ok(())
        }

        #[test]
        fn empty_push() {
            let mut conv = Conveyer::new("Left");
            assert_eq!(Err(String::from("Conveyer empty")), conv.push());
        }

        #[test]
        fn full_pull() {
            let mut conv = Conveyer::new("Left");
            assert_eq!(Ok(()), conv.pull());

            assert_eq!(Err(String::from("Conveyer full")), conv.pull());
        }
    }
}
