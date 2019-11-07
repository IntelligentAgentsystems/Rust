use std::time::Duration;
use actix::prelude::*;

// https://crates.io/crates/actix
// https://actix.rs/book/actix/
// https://docs.rs/actix/0.8.3/actix/
// https://github.com/actix/actix/tree/master/examples

// this is our Message

struct Sum(u32, u32);
// we have to define the response type for `Sum` message
impl Message for Sum {
    type Result = u32;
}

struct PrintMessage(String);
impl Message for PrintMessage {
    type Result = ();
}

struct Summator {
    printer: Recipient<PrintMessage>
}
impl Actor for Summator {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Started Summator");
    }
}
impl Handler<Sum> for Summator {
    type Result = u32;   // <- Message response type

    fn handle(&mut self, msg: Sum, ctx: &mut Context<Self>) -> Self::Result {
        let result = msg.0 + msg.1;
        ctx.run_later(Duration::new(0, 10), move |act, _| {
            act.printer.do_send(PrintMessage(result.to_string()))
                .unwrap_or_else(|err| { println!("Could not send PrintMessage! Error: {}", err); });
        });
        result
    }
}

struct Printer;
impl Actor for Printer {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Started Printer");
    }
}
impl Handler<PrintMessage> for Printer {
    type Result = ();

    fn handle(&mut self, msg: PrintMessage, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Got: {}", msg.0);
    }
}

fn main() -> std::io::Result<()> {
    println!("Starting system");
    let sys = actix::System::new("test");
    
    let printer = Printer.start();
    let summator = Summator{printer: printer.recipient()}.start();
    
    println!("Sending Sum(10,5)...");
    let res = summator.send(Sum(10, 5));  // <- send message and get future for result
    Arbiter::spawn(
        res.map(|res| {
            println!("Sum(10,5): {}", res);
        })
        .map_err(|_| ())
    );

    println!("Sending Sum(10,1)...");
    let res = summator.send(Sum(10, 1));  // <- send message and get future for result
    Arbiter::spawn(
        res.map(|res| {
            println!("Sum(10,1): {}", res);
        })
        .map_err(|_| ())
    );

    println!("Entering event loop");
    sys.run()
}