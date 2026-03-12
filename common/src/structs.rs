multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// ── Job types (used by validation-registry and reputation-registry) ──

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Debug)]
pub enum JobStatus {
    New,
    Pending,
    Verified,
    ValidationRequested,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Debug)]
pub struct JobData<M: ManagedTypeApi> {
    pub status: JobStatus,
    pub proof: ManagedBuffer<M>,
    pub employer: ManagedAddress<M>,
    pub creation_timestamp: TimestampMillis,
    pub agent_nonce: u64,
}

// ── Validation types (ERC-8004 validationRequest/Response) ──

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Debug)]
pub struct ValidationRequestData<M: ManagedTypeApi> {
    pub validator_address: ManagedAddress<M>,
    pub agent_nonce: u64,
    pub job_id: ManagedBuffer<M>,
    pub response: u8,
    pub response_hash: ManagedBuffer<M>,
    pub tag: ManagedBuffer<M>,
    pub last_update: TimestampSeconds,
}

// ── Agent types (used by identity-registry) ──

#[type_abi]
#[derive(
    TopEncode, TopDecode, ManagedVecItem, NestedEncode, NestedDecode, Clone, PartialEq, Debug,
)]
pub struct MetadataEntry<M: ManagedTypeApi> {
    pub key: ManagedBuffer<M>,
    pub value: ManagedBuffer<M>,
}

#[type_abi]
#[derive(
    TopEncode, TopDecode, ManagedVecItem, NestedEncode, NestedDecode, Clone, PartialEq, Debug,
)]
pub struct AgentDetails<M: ManagedTypeApi> {
    pub name: ManagedBuffer<M>,
    pub public_key: ManagedBuffer<M>,
}

/// Paginated agent entry: nonce, owner, and details.
#[type_abi]
#[derive(
    TopEncode, TopDecode, ManagedVecItem, NestedEncode, NestedDecode, Clone, PartialEq, Debug,
)]
pub struct AgentListEntry<M: ManagedTypeApi> {
    pub nonce: u64,
    pub owner: ManagedAddress<M>,
    pub details: AgentDetails<M>,
}

#[type_abi]
#[derive(
    TopEncode, TopDecode, ManagedVecItem, NestedEncode, NestedDecode, Clone, PartialEq, Debug,
)]
pub struct ServiceConfigInput<M: ManagedTypeApi> {
    pub service_id: u32,
    pub price: BigUint<M>,
    pub token: TokenId<M>,
    pub nonce: u64,
}

/// Paginated service config entry: service_id and payment.
#[type_abi]
#[derive(
    TopEncode, TopDecode, ManagedVecItem, NestedEncode, NestedDecode, Clone, PartialEq, Debug,
)]
pub struct ServiceConfigEntry<M: ManagedTypeApi> {
    pub service_id: u32,
    pub payment: EgldOrEsdtTokenPayment<M>,
}

#[type_abi]
#[derive(
    TopEncode, TopDecode, ManagedVecItem, NestedEncode, NestedDecode, Clone, PartialEq, Debug,
)]
pub struct AgentRegisteredEventData<M: ManagedTypeApi> {
    pub name: ManagedBuffer<M>,
    pub uri: ManagedBuffer<M>,
}

#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq, Debug,
)]
pub struct AgentUpdatedEventData<M: ManagedTypeApi> {
    pub new_name: ManagedBuffer<M>,
    pub new_uri: ManagedBuffer<M>,
    pub metadata_updated: bool,
    pub services_updated: bool,
}
