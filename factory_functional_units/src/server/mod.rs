use std::sync::Mutex;
use std::time::Duration;

use rand::Rng;
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

pub struct Delayer {
    min: Duration,
    max: Duration,
}

impl Delayer {
    pub fn new(min: Duration, max: Duration) -> Delayer {
        Delayer { min, max }
    }

    pub async fn delay(&self) {
        let dur = rand::thread_rng().gen_range(self.min, self.max);
        println!("Sleeping for {:?}", dur);
        tokio::time::delay_for(dur).await;
    }
}

pub struct PlotterServerState {
    state: Mutex<Plotter>,
    delayer: Delayer,
}

impl PlotterServerState {
    pub fn new(p: Plotter, delayer: Delayer) -> PlotterServerState {
        PlotterServerState {
            state: Mutex::new(p),
            delayer,
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
        let res = {
            let state = self.state.lock().unwrap();
            state.plot()
        };
        self.delayer.delay().await;
        println!("plot - {:?}", res);
        Ok(Response::new(res.into()))
    }

    async fn pull(
        &self,
        _: Request<()>,
    ) -> Result<Response<functional_units::PushOrPullResult>, Status> {
        let res = {
            let mut state = self.state.lock().unwrap();
            state.pull()
        };
        self.delayer.delay().await;
        println!("pull - {:?}", res);
        Ok(Response::new(res.into()))
    }

    async fn push(
        &self,
        _: Request<()>,
    ) -> Result<Response<functional_units::PushOrPullResult>, Status> {
        let res = {
            let mut state = self.state.lock().unwrap();
            state.push()
        };
        self.delayer.delay().await;
        println!("push - {:?}", res);
        Ok(Response::new(res.into()))
    }
}

pub struct ConveyorServerState {
    state: Mutex<Conveyor>,
    delayer: Delayer,
}

impl ConveyorServerState {
    pub fn new(c: Conveyor, delayer: Delayer) -> ConveyorServerState {
        ConveyorServerState {
            state: Mutex::new(c),
            delayer,
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
        let target = functional_units::Orientation::from_i32(req.get_ref().target).unwrap();
        {
            let mut state = self.state.lock().unwrap();
            state.turn_to(target.into());
        }
        self.delayer.delay().await;
        println!("turn_to - {:?}", target);

        Ok(Response::new(()))
    }

    async fn push(
        &self,
        _: Request<()>,
    ) -> Result<Response<functional_units::PushOrPullResult>, Status> {
        let res = {
            let mut state = self.state.lock().unwrap();
            state.push()
        };
        self.delayer.delay().await;
        println!("push - {:?}", res);
        Ok(Response::new(res.into()))
    }

    async fn pull(
        &self,
        _: Request<()>,
    ) -> Result<Response<functional_units::PushOrPullResult>, Status> {
        let res = {
            let mut state = self.state.lock().unwrap();
            state.pull()
        };
        self.delayer.delay().await;
        println!("pull - {:?}", res);
        Ok(Response::new(res.into()))
    }
}

pub struct InputStackServerState {
    state: Mutex<InputStack>,
    delayer: Delayer,
}

impl InputStackServerState {
    pub fn new(s: InputStack, delayer: Delayer) -> InputStackServerState {
        InputStackServerState {
            state: Mutex::new(s),
            delayer,
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
        let res = {
            let mut state = self.state.lock().unwrap();
            state.push()
        };
        self.delayer.delay().await;
        println!("push - {:?}", res);
        Ok(Response::new(res.into()))
    }
}

pub struct OutputStackServerState {
    state: Mutex<OutputStack>,
    delayer: Delayer,
}

impl OutputStackServerState {
    pub fn new(s: OutputStack, delayer: Delayer) -> OutputStackServerState {
        OutputStackServerState {
            state: Mutex::new(s),
            delayer,
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
        {
            let mut state = self.state.lock().unwrap();
            state.pull();
        }
        self.delayer.delay().await;
        println!("pull");
        Ok(Response::new(Ok(()).into()))
    }
}
