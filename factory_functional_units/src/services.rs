use tonic::{Request, Response, Status};
use functional_units::{PlotterStatus, PlotResult, ConveyorStatus};
pub use functional_units::plotter_server::PlotterServer;
pub use functional_units::conveyor_server::ConveyorServer;

use super::Plotter;
use crate::{Conveyor, Orientation};

pub mod functional_units {
    tonic::include_proto!("functional_units");
}

#[tonic::async_trait]
impl functional_units::plotter_server::Plotter for Plotter {
    async fn status(&self, _: Request<()>) -> Result<Response<PlotterStatus>, Status> {
        let reply = functional_units::PlotterStatus {
            name: self.name.clone(),
            has_paper: self.has_paper
        };

        Ok(Response::new(reply))
    }

    async fn plot(&self, _: Request<()>) -> Result<Response<PlotResult>, Status> {
        let res = self.plot();

        let reply = functional_units::PlotResult {
            code: match res {
                Ok(_) => 0,
                Err(_) => 1,
            }
        };
        Ok(Response::new(reply))
    }
}

#[tonic::async_trait]
impl functional_units::conveyor_server::Conveyor for Conveyor {
    async fn status(&self, _: Request<()>) -> Result<Response<ConveyorStatus>, Status> {
        let reply = functional_units::ConveyorStatus {
            name: self.name.clone(),
            has_paper: self.has_paper,
            orientation: match self.current_orientation {
                Orientation::North => 0,
                Orientation::East => 1,
                Orientation::South => 2,
                Orientation::West => 3,
            }
        };

        Ok(Response::new(reply))
    }
}