use neovim_lib::{Neovim, Session};

pub struct EventHandler {
    neovim: Neovim,
}

impl Default for EventHandler {
    fn default() -> Self {
        let mut session = Session::new_parent().unwrap();
        session.set_infinity_timeout();

        let neovim = Neovim::new(session);

        Self { neovim }
    }
}
impl EventHandler {
    pub fn receive_event(&mut self) {
        let receiver = self.neovim.session.start_event_loop_channel();
        for (event, values) in receiver {
            println!("Event: {:?}", event);
            println!("Values: {:?}", values);
        }
    }
}
