multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::structs::ValidationRequestData;

#[multiversx_sc::module]
pub trait EventsModule {
    #[event("jobInitialized")]
    fn job_initialized_event(
        &self,
        #[indexed] job_id: ManagedBuffer,
        #[indexed] employer: ManagedAddress,
        #[indexed] agent_nonce: u64,
        service_id: Option<u32>,
    );

    #[event("validationRequest")]
    fn validation_request_event(
        &self,
        #[indexed] job_id: ManagedBuffer,
        #[indexed] validator_address: ManagedAddress,
        #[indexed] agent_nonce: u64,
        #[indexed] request_hash: ManagedBuffer,
        request_uri: ManagedBuffer,
    );

    #[event("validationResponse")]
    fn validation_response_event(
        &self,
        #[indexed] job_id: ManagedBuffer,
        #[indexed] validator_address: ManagedAddress,
        #[indexed] agent_nonce: u64,
        #[indexed] request_hash: ManagedBuffer,
        data: ValidationRequestData<Self::Api>,
    );
}
