use factory_functional_units::Plotter;
use factory_functional_units::PlotterServer;
use tonic::{transport::Server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let plotter = Plotter::new("Plotter 1");

    Server::builder()
        .add_service(PlotterServer::new(plotter))
        .serve(addr)
        .await?;

    Ok(())
}