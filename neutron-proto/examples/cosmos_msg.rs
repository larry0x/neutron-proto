//! In this example, we show how to build a CosmosMsg with proto types.

use cosmos_sdk_proto::{
    cosmos::base::v1beta1::Coin,
    // we can either import prost::Message or cosmos_sdk_proto::Message
    // they both provide the encode_to_vec method
    traits::{Message, TypeUrl},
};
use cosmwasm_std::CosmosMsg;
use neutron_proto::neutron;

fn build_transfer_msg() -> CosmosMsg {
    let msg = neutron::transfer::MsgTransfer {
        source_port: "transfer".into(),
        source_channel: "channel-0".into(),
        token: Some(Coin {
            denom: "utoken".into(),
            amount: "12345".into(),
        }),
        sender: "larry".into(),
        receiver: "pumpkin".into(),
        timeout_height: None,
        timeout_timestamp: 1700000000,
        memo: "hello".into(),
        fee: None,
    };

    CosmosMsg::Stargate {
        type_url: neutron::transfer::MsgTransfer::TYPE_URL.into(),
        value: msg.encode_to_vec().into(),
    }
}

fn main() {
    let cosmos_msg = build_transfer_msg();
    dbg!(cosmos_msg);
}
