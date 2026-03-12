multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::structs::FeedbackData;
pub use common::structs::{JobData, JobStatus};

#[multiversx_sc::module]
pub trait StorageModule: common::cross_contract::CrossContractModule {
    // ── Local storage (giveFeedbackSimple — on-chain scoring) ──

    #[view(get_reputation_score)]
    #[storage_mapper("reputationScore")]
    fn reputation_score(&self, agent_nonce: u64) -> SingleValueMapper<BigUint>;

    #[view(get_total_jobs)]
    #[storage_mapper("totalJobs")]
    fn total_jobs(&self, agent_nonce: u64) -> SingleValueMapper<u64>;

    #[view(get_validation_contract_address)]
    #[storage_mapper("validationContractAddress")]
    fn validation_contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(get_identity_contract_address)]
    #[storage_mapper("identityContractAddress")]
    fn identity_contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(has_given_feedback)]
    #[storage_mapper("hasGivenFeedback")]
    fn has_given_feedback(&self, job_id: ManagedBuffer) -> SingleValueMapper<bool>;

    #[view(get_agent_response)]
    #[storage_mapper("agentResponse")]
    fn agent_response(&self, job_id: ManagedBuffer) -> SingleValueMapper<ManagedBuffer>;

    // ── ERC-8004 feedback storage (giveFeedback — raw signals) ──

    #[storage_mapper("feedbackData")]
    fn feedback_data(
        &self,
        agent_nonce: u64,
        client: &ManagedAddress,
        index: u64,
    ) -> SingleValueMapper<FeedbackData<Self::Api>>;

    #[view(getLastIndex)]
    #[storage_mapper("lastFeedbackIndex")]
    fn last_feedback_index(
        &self,
        agent_nonce: u64,
        client: &ManagedAddress,
    ) -> SingleValueMapper<u64>;

    #[view(getClients)]
    #[storage_mapper("feedbackClients")]
    fn feedback_clients(&self, agent_nonce: u64) -> UnorderedSetMapper<ManagedAddress>;

    /// Paginated list of client addresses who gave feedback to an agent. `from` = start index, `size` = max items (capped at 100).
    #[view(getFeedbackClientsPage)]
    fn get_feedback_clients_page(
        &self,
        agent_nonce: u64,
        from: u64,
        size: u64,
    ) -> ManagedVec<ManagedAddress<Self::Api>> {
        let size = size.min(100) as usize;
        let from = from.min(usize::MAX as u64) as usize;
        let mut result = ManagedVec::new();
        for addr in self.feedback_clients(agent_nonce).iter().skip(from).take(size) {
            result.push(addr);
        }
        result
    }
}
