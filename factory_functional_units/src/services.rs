use std::sync::Mutex;

use tonic::{Request, Response, Status};

pub use proto::conveyor_server::ConveyorServer;
pub use proto::input_stack_server::InputStackServer;
pub use proto::output_stack_server::OutputStackServer;
pub use proto::plotter_server::PlotterServer;

use crate::{Conveyor, InputStack, Orientation, OutputStack, PlotError, PushOrPullError};

use super::Plotter;

pub mod proto {
    tonic::include_proto!("functional_units");
}

impl Into<i32> for &Orientation {
    fn into(self) -> i32 {
        match self {
            Orientation::North => proto::Orientation::North.into(),
            Orientation::East => proto::Orientation::East.into(),
            Orientation::South => proto::Orientation::South.into(),
            Orientation::West => proto::Orientation::West.into(),
        }
    }
}

impl Into<Orientation> for proto::Orientation {
    fn into(self) -> Orientation {
        match self {
            proto::Orientation::North => Orientation::North,
            proto::Orientation::East => Orientation::East,
            proto::Orientation::South => Orientation::South,
            proto::Orientation::West => Orientation::West,
        }
    }
}

impl Into<proto::PushOrPullResult> for Result<(), PushOrPullError> {
    fn into(self) -> proto::PushOrPullResult {
        proto::PushOrPullResult {
            code: match self {
                Ok(_) => proto::push_or_pull_result::Code::Ok.into(),
                Err(e) => match e {
                    PushOrPullError::Empty => proto::push_or_pull_result::Code::Empty.into(),
                    PushOrPullError::Full => proto::push_or_pull_result::Code::Full.into(),
                },
            },
        }
    }
}

impl Into<proto::PlotResult> for Result<(), PlotError> {
    fn into(self) -> proto::PlotResult {
        proto::PlotResult {
            code: match self {
                Ok(_) => proto::plot_result::Code::Ok.into(),
                Err(e) => match e {
                    PlotError::NoPaper => proto::plot_result::Code::NoPaper.into(),
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
impl proto::plotter_server::Plotter for PlotterServerState {
    async fn status(&self, _: Request<()>) -> Result<Response<proto::PlotterStatus>, Status> {
        let state = self.state.lock().unwrap();
        let reply = proto::PlotterStatus {
            name: state.name().to_owned(),
            has_paper: state.has_paper(),
        };
        println!("status - {:?}", reply);
        Ok(Response::new(reply))
    }

    async fn plot(&self, _: Request<()>) -> Result<Response<proto::PlotResult>, Status> {
        let state = self.state.lock().unwrap();
        let res = state.plot();
        println!("plot - {:?}", res);
        Ok(Response::new(res.into()))
    }

    async fn pull(&self, _: Request<()>) -> Result<Response<proto::PushOrPullResult>, Status> {
        let mut state = self.state.lock().unwrap();
        let res = state.pull();
        println!("pull - {:?}", res);
        Ok(Response::new(res.into()))
    }

    async fn push(&self, _: Request<()>) -> Result<Response<proto::PushOrPullResult>, Status> {
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
impl proto::conveyor_server::Conveyor for ConveyorServerState {
    async fn status(&self, _: Request<()>) -> Result<Response<proto::ConveyorStatus>, Status> {
        let state = self.state.lock().unwrap();
        let reply = proto::ConveyorStatus {
            name: state.name().to_owned(),
            has_paper: state.has_paper(),
            orientation: state.orientation().into(),
        };
        println!("status - {:?}", reply);
        Ok(Response::new(reply))
    }

    async fn pull(&self, _: Request<()>) -> Result<Response<proto::PushOrPullResult>, Status> {
        let mut state = self.state.lock().unwrap();
        let res = state.pull();
        println!("pull - {:?}", res);
        Ok(Response::new(res.into()))
    }

    async fn push(&self, _: Request<()>) -> Result<Response<proto::PushOrPullResult>, Status> {
        let mut state = self.state.lock().unwrap();
        let res = state.push();
        println!("push - {:?}", res);
        Ok(Response::new(res.into()))
    }

    async fn turn_to(&self, req: Request<proto::TurnToRequest>) -> Result<Response<()>, Status> {
        let mut state = self.state.lock().unwrap();
        let target = proto::Orientation::from_i32(req.get_ref().target).unwrap();
        state.turn_to(target.into());
        println!("turn_to - {:?}", target);

        Ok(Response::new(()))
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
impl proto::input_stack_server::InputStack for InputStackServerState {
    async fn status(&self, _: Request<()>) -> Result<Response<proto::InputStackStatus>, Status> {
        let state = self.state.lock().unwrap();
        let reply = proto::InputStackStatus {
            name: state.name().to_owned(),
            paper_count: state.paper_count(),
        };
        println!("status - {:?}", reply);
        Ok(Response::new(reply))
    }

    async fn push(&self, _: Request<()>) -> Result<Response<proto::PushOrPullResult>, Status> {
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
impl proto::output_stack_server::OutputStack for OutputStackServerState {
    async fn status(&self, _: Request<()>) -> Result<Response<proto::OutputStackStatus>, Status> {
        let state = self.state.lock().unwrap();
        let reply = proto::OutputStackStatus {
            name: state.name().to_owned(),
            paper_count: state.paper_count(),
        };
        println!("status - {:?}", reply);
        Ok(Response::new(reply))
    }

    async fn pull(&self, _: Request<()>) -> Result<Response<proto::PushOrPullResult>, Status> {
        let mut state = self.state.lock().unwrap();
        state.pull();
        println!("pull");
        Ok(Response::new(Ok(()).into()))
    }
}
