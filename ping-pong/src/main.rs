use std::time::Duration;

fn main()  -> std::io::Result<()> {
    let sys = actix::System::new("ping-pong");

    sys.run()
}