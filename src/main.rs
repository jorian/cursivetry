use cursive::traits::*;
use std::{cell::RefCell, rc::Rc};
use vrsc_rpc::{Auth, Client};
struct Data {
    pub active_chain: Rc<RefCell<Chain>>,
    pub local_chains: Rc<Vec<Rc<RefCell<Chain>>>>,
}

impl Data {
    pub fn new() -> Self {
        let chain = Rc::new(RefCell::new(Chain::new("VRSC")));
        let another_chain = Rc::new(RefCell::new(Chain::new("VRSCTEST")));

        let data = Data {
            active_chain: Rc::clone(&chain),
            local_chains: Rc::new(vec![chain, another_chain]),
        };

        data
    }
}
struct Chain {
    name: String,
    rpc_client: Client,
}

impl Chain {
    pub fn new<S: Into<String>>(name: S) -> Self {
        let rpc_client = Client::default();
        Chain {
            name: name.into(),
            rpc_client,
        }
    }
}

impl std::fmt::Debug for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Chain").field("name", &self.name).finish()
    }
}

fn main() {
    let mut siv = cursive::default();

    let data = Rc::new(RefCell::new(Data::new()));
    siv.set_user_data(Rc::clone(&data));

    let data_clone = Rc::clone(&data);

    let menutree = cursive::menu::MenuTree::new().with(move |tree| {
        for chain in data_clone.borrow().local_chains.iter() {
            let chain = Rc::clone(chain);
            let name = chain.borrow_mut().name.clone();

            tree.add_leaf(name, move |s| {
                s.with_user_data(|data: &mut Rc<RefCell<Data>>| {
                    data.borrow_mut().active_chain = chain.clone();
                    dbg!(&data.borrow().active_chain);
                });
            });
        }
    });

    siv.menubar().add_subtree("Select", menutree);
    siv.set_autohide_menu(false);

    siv.run();
}
// let local_chains = vec![
//     {
//         let rpc_client = Client::chain("VRSC".into(), Auth::ConfigFile).expect("A client");
//         Rc::new(RefCell::new(Chain {
//             name: "VRSC".into(),
//             rpc_client,
//         }))
//     },
//     {
//         let rpc_client =
//             Client::chain("vrsctest".into(), Auth::ConfigFile).expect("A client");
//         Rc::new(RefCell::new(Chain {
//             name: "vrsctest".into(),
//             rpc_client,
//         }))
//     },
// ];
// the problem: by borrowing local_chains here from Data to iterate through them, we call a &mut twice on Data in the add_leaf function
// which isn't possible.