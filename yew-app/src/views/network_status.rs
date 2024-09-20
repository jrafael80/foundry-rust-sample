use ethers::etherscan::account;
use ethers_web::{yew::UseEthereum, WalletType};
use yew::prelude::*;
use yew::html;



#[function_component(NetworkStatus)]
pub fn network_status() -> Html {
    let ethereum = use_context::<UseEthereum>().expect(
        "No ethereum found. You must wrap your components in an <EthereumContextProvider />",
    );

    let wc = use_state(|| false);

    let onclick = {
        let wc = wc.clone();
        Callback::from(move |_: MouseEvent| wc.set(!(*wc)))
    };
    let chains = &*crate::model::chain::CHAINS;
    let chainid = ethereum.chain_id().to_string();
    let chain = chains.get(&chainid);
    let connected = ethereum.is_connected();
    let accounts = ethereum.accounts().map(|account| account.first().map(|a|a.to_string()).unwrap_or("None".into()));
    
    let onclick_ethereum = {
        Callback::from(move |_: MouseEvent| {
            if ethereum.is_connected() {
                ethereum.clone().disconnect();
            } else {
                if *wc {
                    ethereum.clone().connect(WalletType::WalletConnect);
                } else {
                    ethereum.clone().connect(WalletType::Injected);
                }
            }
        })
    };
    html! {
        <>
        <p>{"chainid: "}{chainid}</p>
        <p>{"chain: "}{format!("{:?}", chain)}</p>
        <p>{"connected: "}{connected}</p>
        <p>{"accounts: "}{accounts}</p>
        
        </>
    }
}
