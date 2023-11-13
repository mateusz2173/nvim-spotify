pub mod event_handler;

fn main() {
    let mut event_handler = event_handler::EventHandler::default();
    event_handler.receive_event();
}
