use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;
use neutron_proto::neutron::transfer::MsgTransfer;
// must import the Message trait so that we get the encode/decode methods
use prost::Message;

#[test]
fn proto_encoding() {
    let msg = MsgTransfer {
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

    // encode the message to binary
    let msg_bytes = msg.encode_to_vec();

    // try decoding, should restore the same message
    let msg_decoded = MsgTransfer::decode(msg_bytes.as_slice()).unwrap();
    assert_eq!(msg, msg_decoded);
}
