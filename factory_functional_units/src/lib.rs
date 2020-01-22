use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use std::marker::Copy;

pub use self::conveyor::*;
pub use self::input_stack::*;
pub use self::output_stack::*;
pub use self::plotter::*;
pub use self::server::{
    ConveyorServer, ConveyorServerState, InputStackServer, InputStackServerState,
    OutputStackServer, OutputStackServerState, PlotterServer, PlotterServerState,
};

mod conveyor;
mod input_stack;
mod output_stack;
mod plotter;
mod server;

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

#[derive(PartialEq, Debug)]
pub enum PushOrPullError {
    Empty,
    Full,
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
}
