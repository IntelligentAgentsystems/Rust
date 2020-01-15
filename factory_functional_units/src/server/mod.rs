use std::sync::Mutex;

use tonic::{Request, Response, Status};

pub use functional_units::conveyor_server::ConveyorServer;
pub use functional_units::input_stack_server::InputStackServer;
pub use functional_units::output_stack_server::OutputStackServer;
pub use functional_units::plotter_server::PlotterServer;

use crate::{Conveyor, InputStack, Orientation, OutputStack, PlotError, PushOrPullError};

use super::Plotter;

mod functional_units;

impl Into<i32> for &Orientation {
    fn into(self) -> i32 {
        match self {
            Orientation::North => functional_units::Orientation::North.into(),
            Orientation::East => functional_units::Orientation::East.into(),
            Orientation::South => functional_units::Orientation::South.into(),
            Orientation::West => functional_units::Orientation::West.into(),
        }
    }
}

impl Into<Orientation> for functional_units::Orientation {
    fn into(self) -> Orientation {
        match self {
            functional_units::Orientation::North => Orientation::North,
            functional_units::Orientation::East => Orientation::East,
            functional_units::Orientation::South => Orientation::South,
            functional_units::Orientation::West => Orientation::West,
        }
    }
}

impl Into<functional_units::PushOrPullResult> for Result<(), PushOrPullError> {
    fn into(self) -> functional_units::PushOrPullResult {
        functional_units::PushOrPullResult {
            code: match self {
                Ok(_) => functional_units::push_or_pull_result::Code::Ok.into(),
                Err(e) => match e {
                    PushOrPullError::Empty => {
                        functional_units::push_or_pull_result::Code::Empty.into()
                    }
                    PushOrPullError::Full => {
                        functional_units::push_or_pull_result::Code::Full.into()
                    }
                },
            },
        }
    }
}

impl Into<functional_units::PlotResult> for Result<(), PlotError> {
    fn into(self) -> functional_units::PlotResult {
        functional_units::PlotResult {
            code: match self {
                Ok(_) => functional_units::plot_result::Code::Ok.into(),
                Err(e) => match e {
                    PlotError::NoPaper => functional_units::plot_result::Code::NoPaper.into(),
                },
            },
        }
    }
}

pub struct PlotterServerState {
    state: Mutex<Plotter>,
}

impl PlotterServerState {
    pub fn new(p: Plotter) -> PlotterServerState {
        PlotterServerState {
            state: Mutex::new(p),
        }
    }
}

#[tonic::async_trait]
impl functional_units::plotter_server::Plotter for PlotterServerState {
    async fn status(
        &self,
        _: Request<()>,
    ) -> Result<Response<functional_units::PlotterStatus>, Status> {
        let state = self.state.lock().unwrap();
        let reply = functional_units::PlotterStatus {
            name: state.name().to_owned(),
            has_paper: state.has_paper(),
        };
        println!("status - {:?}", reply);
        Ok(Response::new(reply))
    }

    async fn plot(&self, _: Request<()>) -> Result<Response<functional_units::PlotResult>, Status> {
        let state = self.state.lock().unwrap();
        let res = state.plot();
        println!("plot - {:?}", res);
        Ok(Response::new(res.into()))
    }

    async fn pull(
        &self,
        _: Request<()>,
    ) -> Result<Response<functional_units::PushOrPullResult>, Status> {
        let mut state = self.state.lock().unwrap();
        let res = state.pull();
        println!("pull - {:?}", res);
        Ok(Response::new(res.into()))
    }

    async fn push(
        &self,
        _: Request<()>,
    ) -> Result<Response<functional_units::PushOrPullResult>, Status> {
        let mut state = self.state.lock().unwrap();
        let res = state.push();
        println!("push - {:?}", res);
        Ok(Response::new(res.into()))
    }
}

pub struct ConveyorServerState {
    state: Mutex<Conveyor>,
}

impl ConveyorServerState {
    pub fn new(c: Conveyor) -> ConveyorServerState {
        ConveyorServerState {
            state: Mutex::new(c),
        }
    }
}

#[tonic::async_trait]
impl functional_units::conveyor_server::Conveyor for ConveyorServerState {
    async fn status(
        &self,
        _: Request<()>,
    ) -> Result<Response<functional_units::ConveyorStatus>, Status> {
        let state = self.state.lock().unwrap();
        let reply = functional_units::ConveyorStatus {
            name: state.name().to_owned(),
            has_paper: state.has_paper(),
            orientation: state.orientation().into(),
        };
        println!("status - {:?}", reply);
        Ok(Response::new(reply))
    }

    async fn turn_to(
        &self,
        req: Request<functional_units::TurnToRequest>,
    ) -> Result<Response<()>, Status> {
        let mut state = self.state.lock().unwrap();
        let target = functional_units::Orientation::from_i32(req.get_ref().target).unwrap();
        state.turn_to(target.into());
        println!("turn_to - {:?}", target);

        Ok(Response::new(()))
    }

    async fn push(
        &self,
        _: Request<()>,
    ) -> Result<Response<functional_units::PushOrPullResult>, Status> {
        let mut state = self.state.lock().unwrap();
        let res = state.push();
        println!("push - {:?}", res);
        Ok(Response::new(res.into()))
    }

    async fn pull(
        &self,
        _: Request<()>,
    ) -> Result<Response<functional_units::PushOrPullResult>, Status> {
        let mut state = self.state.lock().unwrap();
        let res = state.pull();
        println!("pull - {:?}", res);
        Ok(Response::new(res.into()))
    }
}

pub struct InputStackServerState {
    state: Mutex<InputStack>,
}

impl InputStackServerState {
    pub fn new(s: InputStack) -> InputStackServerState {
        InputStackServerState {
            state: Mutex::new(s),
        }
    }
}

#[tonic::async_trait]
impl functional_units::input_stack_server::InputStack for InputStackServerState {
    async fn status(
        &self,
        _: Request<()>,
    ) -> Result<Response<functional_units::InputStackStatus>, Status> {
        let state = self.state.lock().unwrap();
        let reply = functional_units::InputStackStatus {
            name: state.name().to_owned(),
            paper_count: state.paper_count(),
        };
        println!("status - {:?}", reply);
        Ok(Response::new(reply))
    }

    async fn push(
        &self,
        _: Request<()>,
    ) -> Result<Response<functional_units::PushOrPullResult>, Status> {
        let mut state = self.state.lock().unwrap();
        let res = state.push();
        println!("push - {:?}", res);
        Ok(Response::new(res.into()))
    }
}

pub struct OutputStackServerState {
    state: Mutex<OutputStack>,
}

impl OutputStackServerState {
    pub fn new(s: OutputStack) -> OutputStackServerState {
        OutputStackServerState {
            state: Mutex::new(s),
        }
    }
}

#[tonic::async_trait]
impl functional_units::output_stack_server::OutputStack for OutputStackServerState {
    async fn status(
        &self,
        _: Request<()>,
    ) -> Result<Response<functional_units::OutputStackStatus>, Status> {
        let state = self.state.lock().unwrap();
        let reply = functional_units::OutputStackStatus {
            name: state.name().to_owned(),
            paper_count: state.paper_count(),
        };
        println!("status - {:?}", reply);
        Ok(Response::new(reply))
    }

    async fn pull(
        &self,
        _: Request<()>,
    ) -> Result<Response<functional_units::PushOrPullResult>, Status> {
        let mut state = self.state.lock().unwrap();
        state.pull();
        println!("pull");
        Ok(Response::new(Ok(()).into()))
    }
}
