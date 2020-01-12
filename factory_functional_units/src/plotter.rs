use crate::PushOrPullError;

pub struct Plotter {
    name: String,
    has_paper: bool,
}

#[derive(PartialEq, Debug)]
pub enum PlotError {
    NoPaper,
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

    pub fn plot(&self) -> Result<(), PlotError> {
        if !self.has_paper {
            Err(PlotError::NoPaper)
        } else {
            Ok(())
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
        let name = "Plotter 1";
        let plot = Plotter::new(name);
        assert_eq!(name, plot.name());
    }

    #[test]
    fn pull_and_push() -> Result<(), PushOrPullError> {
        let mut plot = Plotter::new("Plotter 1");
        plot.pull()?;
        assert_eq!(true, plot.has_paper);

        plot.push()?;
        assert_eq!(false, plot.has_paper);

        Ok(())
    }

    #[test]
    fn empty_push() {
        let mut plot = Plotter::new("Plotter 1");
        assert_eq!(Err(PushOrPullError::Empty), plot.push());
    }

    #[test]
    fn full_pull() {
        let mut plot = Plotter::new("Plotter 1");
        assert_eq!(Ok(()), plot.pull());

        assert_eq!(Err(PushOrPullError::Full), plot.pull());
    }

    #[test]
    fn plot() {
        let mut plot = Plotter::new("Plotter 1");
        assert_eq!(Ok(()), plot.pull());

        assert_eq!(Ok(()), plot.plot());
    }

    #[test]
    fn empty_plot() {
        let plot = Plotter::new("Plotter 1");
        assert_eq!(Err(PlotError::NoPaper), plot.plot());
    }
}
