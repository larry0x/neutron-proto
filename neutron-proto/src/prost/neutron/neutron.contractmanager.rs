// @generated
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {}
/// Failure message contains information about ACK failures and can be used to
/// replay ACK in case of requirement.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Failure {
    /// ChannelId
    #[prost(string, tag = "1")]
    pub channel_id: ::prost::alloc::string::String,
    /// Address of the failed contract
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    /// id of the failure under specific address
    #[prost(uint64, tag = "3")]
    pub id: u64,
    /// ACK id to restore
    #[prost(uint64, tag = "4")]
    pub ack_id: u64,
    /// Acknowledgement type
    #[prost(string, tag = "5")]
    pub ack_type: ::prost::alloc::string::String,
}
/// GenesisState defines the contractmanager module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// List of the contract failures
    ///
    /// this line is used by starport scaffolding # genesis/proto/state
    #[prost(message, repeated, tag = "2")]
    pub failures_list: ::prost::alloc::vec::Vec<Failure>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFailuresRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFailuresResponse {
    #[prost(message, repeated, tag = "1")]
    pub failures: ::prost::alloc::vec::Vec<Failure>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
include!("neutron.contractmanager.tonic.rs");
// @@protoc_insertion_point(module)
