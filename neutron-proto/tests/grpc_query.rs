use neutron_proto::neutron::interchainqueries::{self as icq, query_client::QueryClient};

const GRPC_ENDPOINT: &str = "http://grpc-kralum.neutron-1.neutron.org:80";

/// In this test, we make an gRPC query using the Rust code generated from
/// Protobuf. If the query returns a valid result then we consider the test
/// passes.
#[tokio::test]
async fn querying_icq_params() {
    let params = QueryClient::connect(GRPC_ENDPOINT)
        .await
        .unwrap()
        .params(icq::QueryParamsRequest {})
        .await
        .unwrap()
        .into_inner()
        .params
        .unwrap();

    assert!(!params.query_deposit.is_empty());
    assert!(params.query_submit_timeout > 0);
    assert!(params.tx_query_removal_limit > 0);
}
