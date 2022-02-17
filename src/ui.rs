use std::sync::{mpsc, Arc, RwLock};

use cursive::{CursiveRunnable, CursiveRunner};

use crate::{
    aview::AView,
    controller::{Chain, ControllerMessage},
};

pub struct UI {
    pub siv: CursiveRunner<CursiveRunnable>,
}

impl UI {
    pub fn new(
        c_tx: mpsc::Sender<ControllerMessage>,
        chains: &Vec<String>,
        selected_chain: Arc<RwLock<Option<Arc<RwLock<Chain>>>>>,
    ) -> Self {
        let mut siv = cursive::default().into_runner();

        siv.add_layer(AView::new(selected_chain));

        crate::menubar::menubar(&mut siv, c_tx, chains);

        UI { siv }
    }

    pub fn step(&mut self) -> bool {
        if !self.siv.is_running() {
            return false;
        };

        self.siv.step();

        true
    }
}
