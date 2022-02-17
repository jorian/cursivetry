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
    pub fn new(selected_chain: Arc<RwLock<Option<Arc<RwLock<Chain>>>>>) -> Self {
        AView {
            view: Dialog::new()
                .content(TextView::new("Hello"))
                .button("save", move |_siv| {
                    dbg!(&selected_chain);
                    if let Ok(read_selected_chain) = selected_chain.read() {
                        if let Some(chain) = read_selected_chain.as_ref() {
                            if let Ok(read_chain) = chain.read() {
                                dbg!(&read_chain.name);
                            }
                        }
                    }
                }),
        }
    }
}

impl ViewWrapper for AView {
    cursive::wrap_impl!(self.view: Dialog);
}
