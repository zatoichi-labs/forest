mod cli;
mod log;

use self::cli::cli;
use ferret_libp2p::service::NetworkEvent;
use futures::prelude::*;
use network::service::NetworkMessage;
use network::service::NetworkService;
use slog::info;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::prelude::*;
fn main() {
    let log = log::setup_logging();
    // logger::set_max_level(LevelFilter::Debug);
    info!(log, "Starting Ferret");

    // Capture CLI inputs
    let config = cli(&log).expect("CLI error");

    // Create the tokio runtime
    let rt = Runtime::new().unwrap();
    let exec = rt.executor();

    // Create the channel so we can receive messages from NetworkService
    let (tx, mut _rx) = mpsc::unbounded_channel::<NetworkEvent>();
    // Create the default libp2p config
    // Start the NetworkService. Returns net_tx so  you can pass messages in.
    let (_network_service, _net_tx, _exit_tx) =
        NetworkService::new(&config.network, &log, tx, &rt.executor());

    exec.spawn(futures::future::poll_fn(move || -> Result<_, _> {
        loop{
            match _rx.poll() {
                Ok(Async::Ready(Some(event))) => match event {
                    NetworkEvent::PubsubMessage { source, topics, message } => {
                        println!("source: {:?} message: {:?}", source, message);
                    }
                },
                Ok(Async::NotReady) => break,
                _ => break,
            }
        }
        task::current().notify();
        Ok(Async::NotReady)
    }));

    rt.shutdown_on_idle().wait().unwrap();
    info!(log, "Ferret finish shutdown");
}
