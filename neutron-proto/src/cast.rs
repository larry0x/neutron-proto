use cosmos_sdk_proto::traits::{Message, TypeUrl};
use cosmwasm_std::CosmosMsg;

use crate::neutron;

macro_rules! impl_into_cosmos_msg {
    ($($ty: ty),*, $(,)*) => {
        $(
            impl From<$ty> for CosmosMsg {
                fn from(msg: $ty) -> CosmosMsg {
                    CosmosMsg::Stargate {
                        type_url: <$ty>::TYPE_URL.into(),
                        value: msg.encode_to_vec().into(),
                    }
                }
            }
        )*
    }
}

impl_into_cosmos_msg!(
    neutron::interchainqueries::MsgRegisterInterchainQuery,
    neutron::interchainqueries::MsgRemoveInterchainQueryRequest,
    neutron::interchainqueries::MsgSubmitQueryResult,
    neutron::interchainqueries::MsgUpdateInterchainQueryRequest,
    neutron::transfer::MsgTransfer,
);

#[test]
fn casting() {
    use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;

    // if this compiles then we're succesful
    let _: CosmosMsg = neutron::transfer::MsgTransfer {
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
    }
    .into();
}
