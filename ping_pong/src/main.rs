use actix::prelude::*;

struct Ping {
    id: u32,
    message: String
}
impl Message for Ping {
    type Result = ();
}

struct Pinger {
    id: u32,
    sends: String,
    other: Recipient<Ping>
}
impl Actor for Pinger {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Started Pinger #{}", self.id);
    }
}
impl Handler<Ping> for Pinger {
    type Result = ();

     fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Pinger #{}: Got Ping #{} '{}'", self.id, msg.id, msg.message);

        self.other.do_send(Ping{id: msg.id + 1, message: self.sends.clone()})
            .unwrap_or_else(|err| { println!("Could not send PrintMessage! Error: {}", err); });
    }
}

fn main()  -> std::io::Result<()> {
    let sys = actix::System::new("ping-pong");

     let addr = Pinger::create(|ctx| {
        let addr = ctx.address();
        let addr2 = Pinger{id: 2, other: addr.recipient(), sends: String::from("Ping")}.start();

        Pinger{id: 1, other: addr2.recipient(), sends: String::from("Pong")}
    });
    addr.do_send(Ping{id: 1, message: String::from("Ping")}); // initial ping

    sys.run()
}