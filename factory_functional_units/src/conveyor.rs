use crate::{Orientation, PushOrPullError};

pub struct Conveyor {
    name: String,
    current_orientation: Orientation,
    has_paper: bool,
}

impl Conveyor {
    pub fn new(name: &str) -> Conveyor {
        Conveyor {
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
            self.current_orientation = new_orientation;
        }
    }

    pub fn push(&mut self) -> Result<(), PushOrPullError> {
        if !self.has_paper {
            Err(PushOrPullError::Empty)
        } else {
            self.has_paper = false;
            Ok(())
        }
    }

    pub fn pull(&mut self) -> Result<(), PushOrPullError> {
        if self.has_paper {
            Err(PushOrPullError::Full)
        } else {
            self.has_paper = true;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let name = "Left";
        let conv = Conveyor::new(name);
        assert_eq!(name, conv.name());
        assert_eq!(&Orientation::East, conv.orientation());
    }

    #[test]
    fn turn_to() {
        let mut conv = Conveyor::new("Left");
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
    fn pull_and_push() -> Result<(), PushOrPullError> {
        let mut conv = Conveyor::new("Left");
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
        let mut conv = Conveyor::new("Left");
        assert_eq!(Err(PushOrPullError::Empty), conv.push());
    }

    #[test]
    fn full_pull() {
        let mut conv = Conveyor::new("Left");
        assert_eq!(Ok(()), conv.pull());

        assert_eq!(Err(PushOrPullError::Full), conv.pull());
    }
}
