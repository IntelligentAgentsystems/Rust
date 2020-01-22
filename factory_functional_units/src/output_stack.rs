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

    pub fn pull(&mut self) {
        self.paper_count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let name = "Main";
        let stack = OutputStack::new(name);
        assert_eq!(name, stack.name());
        assert_eq!(0, stack.paper_count());
    }

    #[test]
    fn pull() {
        let mut stack = OutputStack::new("Main");

        stack.pull();
        assert_eq!(1, stack.paper_count());

        stack.pull();
        assert_eq!(2, stack.paper_count());
    }
}
