// @generated
/// MsgRegisterInterchainAccount is used to register an account on a remote zone.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterInterchainAccount {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub interchain_account_id: ::prost::alloc::string::String,
}
/// MsgRegisterInterchainAccountResponse is the response type for
/// MsgRegisterInterchainAccount.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterInterchainAccountResponse {}
/// MsgSubmitTx defines the payload for Msg/SubmitTx
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitTx {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    /// interchain_account_id is supposed to be the unique identifier, e.g.,
    /// lido/kava. This allows contracts to have more than one interchain accounts
    /// on remote zone This identifier will be a part of the portID that we'll
    /// claim our capability for.
    #[prost(string, tag = "2")]
    pub interchain_account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub msgs: ::prost::alloc::vec::Vec<::prost_types::Any>,
    #[prost(string, tag = "5")]
    pub memo: ::prost::alloc::string::String,
    /// timeout in seconds after which the packet times out
    #[prost(uint64, tag = "6")]
    pub timeout: u64,
    #[prost(message, optional, tag = "7")]
    pub fee: ::core::option::Option<super::super::feerefunder::Fee>,
}
/// MsgSubmitTxResponse defines the response for Msg/SubmitTx
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitTxResponse {
    /// channel's sequence_id for outgoing ibc packet. Unique per a channel.
    #[prost(uint64, tag = "1")]
    pub sequence_id: u64,
    /// channel src channel on neutron side trasaction was submitted from
    #[prost(string, tag = "2")]
    pub channel: ::prost::alloc::string::String,
}
include!("neutron.interchaintxs.v1.tonic.rs");
// @@protoc_insertion_point(module)
