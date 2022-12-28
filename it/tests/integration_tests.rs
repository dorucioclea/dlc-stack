use cucumber::{given, when, then, Parameter, World};
use dlc_clients::{AcceptMessage, ApiResult, OfferRequest, OracleBackendClient, StorageApiClient, WalletBackendClient};
use derive_more::{Deref, FromStr};
use tokio::runtime::Runtime;

#[derive(Deref, FromStr, Parameter)]
#[param(regex = r"\d+", name = "u64")]
struct CustomU64(u64);

#[derive(Debug, Default, World)]
pub struct DlcLinkWorld {
    wallet_client: WalletBackendClient,
    oracle_client: OracleBackendClient,
    storage_api_client: StorageApiClient,
    collected_responses: Vec<ApiResult>,
}

#[given(expr="a wallet backend client with address {word}")]
fn create_wallet_client(world: &mut DlcLinkWorld, address: String) {
    world.wallet_client = WalletBackendClient::new(address);
}

#[given(expr="an oracle backend client with address {word}")]
fn create_oracle_client(world: &mut DlcLinkWorld, address: String) {
    world.oracle_client = OracleBackendClient::new(address);
}

#[when(expr="accept message: {word}")]
fn wallet_accept_message(world: &mut DlcLinkWorld, accept_message: String) {
    let accept_msg_request = AcceptMessage {
        accept_message: accept_message.to_string()
    };
    let mut runtime = Runtime::new().unwrap();
    let _res = runtime.block_on(world.wallet_client.put_accept(accept_msg_request));
}

#[when(expr="creating an offer request with uuid {word}, accept_collateral: {u64} and offer_collateral: {u64}")]
fn create_offer(world: &mut DlcLinkWorld, uuid: String, accept_collateral: CustomU64, offer_collateral: CustomU64) {
    let offer_request = OfferRequest {
        uuid: uuid.to_string(),
        accept_collateral: *accept_collateral,
        offer_collateral: *offer_collateral,
        total_outcomes: 2,
    };
    let mut runtime = Runtime::new().unwrap();
    let res = runtime.block_on(world.wallet_client.post_offer_and_accept(offer_request));
    world.collected_responses.push(res.unwrap());
}

#[when(expr="creating a new oracle event with uuid {word}")]
fn create_event(world: &mut DlcLinkWorld, uuid: String) {
    let mut runtime = Runtime::new().unwrap();
    let res = runtime.block_on(world.oracle_client.create_event(uuid.to_string()));
    world.collected_responses.push(res.unwrap());
}

#[when(expr="getting an attestation with uuid {word} and outcome: {word}")]
fn get_attest(world: &mut DlcLinkWorld, uuid: String, outcome: String) {
    let mut runtime = Runtime::new().unwrap();
    let res = runtime.block_on(world.oracle_client.get_attestation(uuid.to_string(), outcome.to_string()));
    world.collected_responses.push(res.unwrap());
}

#[then(expr = "expected status code is {u64}")]
fn expected_offer_result(world: &mut DlcLinkWorld, status_code: CustomU64) {
    let api_res = world.collected_responses.get(0).unwrap();
    assert_eq!(*status_code, api_res.status as u64);
}

fn main() {
    futures::executor::block_on(DlcLinkWorld::run("tests/features"));
}