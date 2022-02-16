use std::sync::mpsc;

use cursive::{menu::MenuTree, traits::*, Cursive};

use crate::controller::ControllerMessage;

pub fn menubar(siv: &mut Cursive, c_tx: mpsc::Sender<ControllerMessage>, chains: &Vec<String>) {
    siv.menubar()
        .add_subtree(
            "File",
            MenuTree::new()
                .leaf("Quit", |s| s.quit())
                .leaf("Help", |_| {}),
        )
        .add_subtree("Select", new_select_tree(c_tx.clone(), chains));

    siv.set_autohide_menu(false);
}

fn new_select_tree(
    c_tx: mpsc::Sender<ControllerMessage>,
    installed_chains: &Vec<String>,
) -> MenuTree {
    MenuTree::new().with(move |tree| {
        for chain in installed_chains.iter() {
            let c_tx_clone = c_tx.clone();
            let name = chain.clone();
            tree.add_leaf(name.clone(), move |_s| {
                c_tx_clone
                    .send(ControllerMessage::SetActiveChain(name.clone()))
                    .unwrap();
            });
        }
    })
}
