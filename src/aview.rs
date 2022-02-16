use std::sync::{Arc, RwLock};

use cursive::{
    view::ViewWrapper,
    views::{Dialog, TextView},
};

use crate::controller::Chain;

pub struct AView {
    view: Dialog,
}

impl AView {
    pub fn new(selected_chain: Option<Arc<RwLock<Chain>>>) -> Self {
        if let Some(selected_chain) = selected_chain {
            let chain = Arc::clone(&selected_chain);
            AView {
                view: Dialog::new()
                    .content(TextView::new("Hello"))
                    .button("save", move |_siv| {
                        dbg!(&chain);
                        // it is here that i require the current `selected_chain`, but it 'sticks' to the one with which it was initiated
                    }),
            }
        } else {
            AView {
                view: Dialog::new(),
            }
        }
    }
}

impl ViewWrapper for AView {
    cursive::wrap_impl!(self.view: Dialog);
}
