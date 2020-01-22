use clap::{App, Arg, arg_enum, value_t};
use tonic::transport::Server;

use factory_functional_units::*;

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum Unit {
        Plotter,
        Conveyor,
        InputStack,
        OutputStack
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("factory_functional_units")
        .version("0.1.3")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Sets the port the service listens to")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("unit")
                .short("u")
                .long("unit")
                .value_name("UNIT")
                .help("Defines which unit to run (only one unit can run)")
                .possible_values(&Unit::variants())
                .case_insensitive(true)
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("Name of the unit (visible when querying status)")
                .default_value("Unnamed"),
        )
        .get_matches();

    let port = matches.value_of("port").unwrap();
    let unit: Unit = value_t!(matches, "unit", Unit).unwrap();
    let name = matches.value_of("name").unwrap();

    let addr = format!("0.0.0.0:{}", port).parse()?;
    println!("Running unit {} '{}' and binding to {}", unit, name, addr);
    match unit {
        Unit::Plotter => {
            let plotter = Plotter::new(name);
            Server::builder()
                .add_service(PlotterServer::new(PlotterServerState::new(plotter)))
                .serve(addr)
                .await?;
        }
        Unit::Conveyor => {
            let conv = Conveyor::new(name);
            Server::builder()
                .add_service(ConveyorServer::new(ConveyorServerState::new(conv)))
                .serve(addr)
                .await?;
        }
        Unit::InputStack => {
            let stack = InputStack::new(name, 10);
            Server::builder()
                .add_service(InputStackServer::new(InputStackServerState::new(stack)))
                .serve(addr)
                .await?;
        }
        Unit::OutputStack => {
            let stack = OutputStack::new(name);
            Server::builder()
                .add_service(OutputStackServer::new(OutputStackServerState::new(stack)))
                .serve(addr)
                .await?;
        }
    }
    Ok(())
}
