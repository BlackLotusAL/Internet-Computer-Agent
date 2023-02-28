use std::sync::Arc;
use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
use ic_agent::Agent;
use ic_agent::identity::BasicIdentity;
use candid::{Decode, Encode, Principal};

#[tokio::main]
async fn main() {
    let url = "https://ic0.app".to_string();
    let identity = BasicIdentity::from_pem_file("identity.pem").unwrap();
    let transport = ReqwestHttpReplicaV2Transport::create(url).expect("transport error");
    let agent = Arc::new(Agent::builder().with_transport(transport).with_identity(identity).build().expect("agent failed to build"));
    let canister_id = Arc::new(Principal::from_text("u7pov-aqaaa-aaaap-qa3iq").unwrap());
    let controller = Principal::from_text("u7pov-aqaaa-aaaap-qa3iq-u7pov-aqaaa-aaaap-qa3iq-mq5wt-leaja-aqe").unwrap();
    let ids = vec![Principal::from_text("u7pov-aqaaa-aaaap-qa3iq-cai").unwrap()];

    let res = agent
        .update(&canister_id, "addControllerToCanisters")
        .with_arg(&Encode!(&controller, &ids).unwrap())
        .call_and_wait()
        .await;
    println!("{:#?}", res);

    if let Ok(res) = res{
        let res = Decode!(&res, bool).unwrap();
        println!("{}", res);
    }
}
