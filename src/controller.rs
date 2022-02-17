use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{mpsc, Arc, RwLock},
};

use crate::ui::UI;

pub struct Controller {
    pub c_rx: mpsc::Receiver<ControllerMessage>,
    pub ui: UI,
    pub chains: HashMap<String, Arc<RwLock<Chain>>>,
    pub selected_chain: Arc<RwLock<Option<Arc<RwLock<Chain>>>>>,
}

impl Controller {
    pub fn new() -> Self {
        let (c_tx, c_rx) = mpsc::channel::<ControllerMessage>();

        let mut chains = HashMap::new();
        for i in 0..5 {
            chains.insert(
                format!("{}", i),
                Arc::new(RwLock::new(Chain {
                    name: format!("{}", i),
                })),
            );
        }

        let installed_chains = chains.keys().cloned().collect();
        let selected_chain = Arc::new(RwLock::new(chains.get("1").cloned()));

        Controller {
            c_rx,
            ui: UI::new(c_tx.clone(), &installed_chains, Arc::clone(&selected_chain)),
            chains,
            selected_chain: selected_chain,
        }
    }

    pub fn start(&mut self) {
        while self.ui.step() {
            if let Some(message) = self.c_rx.try_iter().next() {
                match message {
                    ControllerMessage::SetActiveChain(chain) => {
                        if let Ok(mut write_ref) = self.selected_chain.write() {
                            if let Some(_selected_chain) = write_ref.as_mut() {
                                let new_chain = self.chains.get(&chain);
                                dbg!(&new_chain);

                                *write_ref = new_chain.cloned();
                            }
                        }
                        dbg!(&self.selected_chain);
                    }
                }
            }
        }
    }
}

impl Debug for Controller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Controller")
            .field("chains", &self.chains)
            .field("selected_chain", &self.selected_chain)
            .finish()
    }
}

pub enum ControllerMessage {
    SetActiveChain(String),
}

#[derive(Debug, Clone)]
pub struct Chain {
    pub name: String,
}
