use crate::PushOrPullError;

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

    pub fn push(&mut self) -> Result<(), PushOrPullError> {
        if self.paper_count == 0 {
            Err(PushOrPullError::Empty)
        } else {
            self.paper_count -= 1;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let name = "Main";
        let paper_count = 10;
        let stack = InputStack::new(name, 10);
        assert_eq!(name, stack.name());
        assert_eq!(paper_count, stack.paper_count());
    }

    #[test]
    fn push() -> Result<(), PushOrPullError> {
        let paper_count = 10;
        let mut stack = InputStack::new("Main", paper_count);

        stack.push()?;
        assert_eq!(paper_count - 1, stack.paper_count());

        Ok(())
    }

    #[test]
    fn push_till_empty() {
        let paper_count = 10;
        let mut stack = InputStack::new("Main", paper_count);

        for i in 1..paper_count + 1 {
            assert_eq!(Ok(()), stack.push());
            assert_eq!(paper_count - i, stack.paper_count());
        }

        assert_eq!(Err(PushOrPullError::Empty), stack.push());
    }
}
