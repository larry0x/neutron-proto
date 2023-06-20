use cosmos_sdk_proto::traits::TypeUrl;

use crate::neutron;

impl TypeUrl for neutron::contractmanager::Failure {
    const TYPE_URL: &'static str = "/neutron.contractmanager.Failure";
}

impl TypeUrl for neutron::contractmanager::GenesisState {
    const TYPE_URL: &'static str = "/neutron.contractmanager.GenesisState";
}

impl TypeUrl for neutron::contractmanager::Params {
    const TYPE_URL: &'static str = "/neutron.contractmanager.Params";
}

impl TypeUrl for neutron::contractmanager::QueryFailuresRequest {
    const TYPE_URL: &'static str = "/neutron.contractmanager.QueryFailuresRequest";
}

impl TypeUrl for neutron::contractmanager::QueryFailuresResponse {
    const TYPE_URL: &'static str = "/neutron.contractmanager.QueryFailuresResponse";
}

impl TypeUrl for neutron::contractmanager::QueryParamsRequest {
    const TYPE_URL: &'static str = "/neutron.contractmanager.QueryParamsRequest";
}

impl TypeUrl for neutron::contractmanager::QueryParamsResponse {
    const TYPE_URL: &'static str = "/neutron.contractmanager.QueryParamsResponse";
}

impl TypeUrl for neutron::cron::GenesisState {
    const TYPE_URL: &'static str = "/neutron.contractmanager.QueryParamsResponse";
}

impl TypeUrl for neutron::cron::MsgExecuteContract {
    const TYPE_URL: &'static str = "/neutron.contractmanager.MsgExecuteContract";
}

impl TypeUrl for neutron::cron::Params {
    const TYPE_URL: &'static str = "/neutron.contractmanager.Params";
}

impl TypeUrl for neutron::cron::QueryGetScheduleRequest {
    const TYPE_URL: &'static str = "/neutron.contractmanager.QueryGetScheduleRequest";
}

impl TypeUrl for neutron::cron::QueryGetScheduleResponse {
    const TYPE_URL: &'static str = "/neutron.contractmanager.QueryGetScheduleResponse";
}

impl TypeUrl for neutron::cron::QueryParamsRequest {
    const TYPE_URL: &'static str = "/neutron.contractmanager.QueryParamsRequest";
}

impl TypeUrl for neutron::cron::QueryParamsResponse {
    const TYPE_URL: &'static str = "/neutron.contractmanager.QueryParamsResponse";
}

impl TypeUrl for neutron::cron::QuerySchedulesRequest {
    const TYPE_URL: &'static str = "/neutron.contractmanager.QuerySchedulesRequest";
}

impl TypeUrl for neutron::cron::QuerySchedulesResponse {
    const TYPE_URL: &'static str = "/neutron.contractmanager.QuerySchedulesResponse";
}

impl TypeUrl for neutron::cron::Schedule {
    const TYPE_URL: &'static str = "/neutron.contractmanager.Schedule";
}

impl TypeUrl for neutron::cron::ScheduleCount {
    const TYPE_URL: &'static str = "/neutron.contractmanager.ScheduleCount";
}

impl TypeUrl for neutron::feeburner::GenesisState {
    const TYPE_URL: &'static str = "/neutron.feeburner.GenesisState";
}

impl TypeUrl for neutron::feeburner::Params {
    const TYPE_URL: &'static str = "/neutron.feeburner.Params";
}

impl TypeUrl for neutron::feeburner::QueryParamsRequest {
    const TYPE_URL: &'static str = "/neutron.feeburner.QueryParamsRequest";
}

impl TypeUrl for neutron::feeburner::QueryParamsResponse {
    const TYPE_URL: &'static str = "/neutron.feeburner.QueryParamsResponse";
}

impl TypeUrl for neutron::feeburner::QueryTotalBurnedNeutronsAmountRequest {
    const TYPE_URL: &'static str = "/neutron.feeburner.QueryTotalBurnedNeutronsAmountRequest";
}

impl TypeUrl for neutron::feeburner::QueryTotalBurnedNeutronsAmountResponse {
    const TYPE_URL: &'static str = "/neutron.feeburner.QueryTotalBurnedNeutronsAmountResponse";
}

impl TypeUrl for neutron::feeburner::TotalBurnedNeutronsAmount {
    const TYPE_URL: &'static str = "/neutron.feeburner.TotalBurnedNeutronsAmount";
}

impl TypeUrl for neutron::feerefunder::Fee {
    const TYPE_URL: &'static str = "/neutron.feerefunder.Fee";
}

impl TypeUrl for neutron::feerefunder::FeeInfo {
    const TYPE_URL: &'static str = "/neutron.feerefunder.FeeInfo";
}

impl TypeUrl for neutron::feerefunder::FeeInfoRequest {
    const TYPE_URL: &'static str = "/neutron.feerefunder.FeeInfoRequest";
}

impl TypeUrl for neutron::feerefunder::FeeInfoResponse {
    const TYPE_URL: &'static str = "/neutron.feerefunder.FeeInfoResponse";
}

impl TypeUrl for neutron::feerefunder::GenesisState {
    const TYPE_URL: &'static str = "/neutron.feerefunder.GenesisState";
}

impl TypeUrl for neutron::feerefunder::PacketId {
    const TYPE_URL: &'static str = "/neutron.feerefunder.PacketId";
}

impl TypeUrl for neutron::feerefunder::Params {
    const TYPE_URL: &'static str = "/neutron.feerefunder.Params";
}

impl TypeUrl for neutron::feerefunder::QueryParamsRequest {
    const TYPE_URL: &'static str = "/neutron.feerefunder.QueryParamsRequest";
}

impl TypeUrl for neutron::feerefunder::QueryParamsResponse {
    const TYPE_URL: &'static str = "/neutron.feerefunder.QueryParamsResponse";
}

impl TypeUrl for neutron::interchainqueries::Block {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.Block";
}

impl TypeUrl for neutron::interchainqueries::GenesisState {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.GenesisState";
}

impl TypeUrl for neutron::interchainqueries::KvKey {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.KvKey";
}

impl TypeUrl for neutron::interchainqueries::MsgRegisterInterchainQuery {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.MsgRegisterInterchainQuery";
}

impl TypeUrl for neutron::interchainqueries::MsgRegisterInterchainQueryResponse {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.MsgRegisterInterchainQueryResponse";
}

impl TypeUrl for neutron::interchainqueries::MsgRemoveInterchainQueryRequest {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.MsgRemoveInterchainQuery";
}

impl TypeUrl for neutron::interchainqueries::MsgRemoveInterchainQueryResponse {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.MsgRemoveInterchainQueryResponse";
}

impl TypeUrl for neutron::interchainqueries::MsgSubmitQueryResult {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.MsgSubmitQueryResult";
}

impl TypeUrl for neutron::interchainqueries::MsgSubmitQueryResultResponse {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.MsgSubmitQueryResultResponse";
}

impl TypeUrl for neutron::interchainqueries::MsgUpdateInterchainQueryRequest {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.MsgUpdateInterchainQueryRequest";
}

impl TypeUrl for neutron::interchainqueries::MsgUpdateInterchainQueryResponse {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.MsgUpdateInterchainQueryResponse";
}

impl TypeUrl for neutron::interchainqueries::Params {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.Params";
}

impl TypeUrl for neutron::interchainqueries::QueryLastRemoteHeight {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.QueryLastRemoteHeight";
}

impl TypeUrl for neutron::interchainqueries::QueryLastRemoteHeightResponse {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.QueryLastRemoteHeightResponse";
}

impl TypeUrl for neutron::interchainqueries::QueryParamsRequest {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.QueryParamsRequest";
}

impl TypeUrl for neutron::interchainqueries::QueryParamsResponse {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.QueryParamsResponse";
}

impl TypeUrl for neutron::interchainqueries::QueryRegisteredQueriesRequest {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.QueryRegisteredQueriesRequest";
}

impl TypeUrl for neutron::interchainqueries::QueryRegisteredQueriesResponse {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.QueryRegisteredQueriesResponse";
}

impl TypeUrl for neutron::interchainqueries::QueryRegisteredQueryResultRequest {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.QueryRegisteredQueryResultRequest";
}

impl TypeUrl for neutron::interchainqueries::QueryRegisteredQueryResultResponse {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.QueryRegisteredQueryResultResponse";
}

impl TypeUrl for neutron::interchainqueries::QueryResult {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.QueryResult";
}

impl TypeUrl for neutron::interchainqueries::RegisteredQuery {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.RegisteredQuery";
}

impl TypeUrl for neutron::interchainqueries::StorageValue {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.StorageValue";
}

impl TypeUrl for neutron::interchainqueries::Transaction {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.Transaction";
}

impl TypeUrl for neutron::interchainqueries::TxValue {
    const TYPE_URL: &'static str = "/neutron.interchainqueries.TxValue";
}

impl TypeUrl for neutron::interchaintxs::v1::MsgRegisterInterchainAccount {
    const TYPE_URL: &'static str = "/neutron.interchaintxs.v1.MsgRegisterInterchainAccount";
}

impl TypeUrl for neutron::interchaintxs::v1::MsgRegisterInterchainAccountResponse {
    const TYPE_URL: &'static str = "/neutron.interchaintxs.v1.MsgRegisterInterchainAccountResponse";
}

impl TypeUrl for neutron::interchaintxs::v1::MsgSubmitTx {
    const TYPE_URL: &'static str = "/neutron.interchaintxs.v1.MsgSubmitTx";
}

impl TypeUrl for neutron::interchaintxs::v1::MsgSubmitTxResponse {
    const TYPE_URL: &'static str = "/neutron.interchaintxs.v1.MsgSubmitTxResponse";
}

impl TypeUrl for neutron::transfer::MsgTransfer {
    const TYPE_URL: &'static str = "/neutron.transfer.MsgTransfer";
}

impl TypeUrl for neutron::transfer::MsgTransferResponse {
    const TYPE_URL: &'static str = "/neutron.transfer.MsgTransferResponse";
}
