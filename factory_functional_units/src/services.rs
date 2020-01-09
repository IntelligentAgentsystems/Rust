use tonic::{Request, Response, Status};
use functional_units::{StatusResponse};
pub use functional_units::plotter_server::PlotterServer;

use super::Plotter;

pub mod functional_units {
    tonic::include_proto!("functional_units");
}

#[tonic::async_trait]
impl functional_units::plotter_server::Plotter for Plotter {
    async fn status(&self, _: Request<()>) -> Result<Response<StatusResponse>, Status> {
        let reply = functional_units::StatusResponse {
            name: self.name.clone(),
            has_paper: self.has_paper
        };

        Ok(Response::new(reply))
    }
}