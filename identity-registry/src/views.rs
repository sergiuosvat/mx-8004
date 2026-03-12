multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use common::structs::{AgentDetails, AgentListEntry, MetadataEntry, ServiceConfigEntry};

#[multiversx_sc::module]
pub trait ViewsModule: crate::storage::StorageModule {
    #[view(get_agent)]
    fn get_agent(&self, nonce: u64) -> AgentDetails<Self::Api> {
        require!(self.agents().contains_id(&nonce), "Agent not found");
        self.agent_details(nonce).get()
    }

    /// Paginated list of agents. `from` = start index, `size` = max items (capped at 100).
    #[view(get_agents)]
    fn get_agents(&self, from: u64, size: u64) -> ManagedVec<AgentListEntry<Self::Api>> {
        let size = size.min(100) as usize;
        let from = from.min(usize::MAX as u64) as usize;
        let mut result = ManagedVec::new();
        for (nonce, owner) in self.agents().iter().skip(from).take(size) {
            let details = self.agent_details(nonce).get();
            result.push(AgentListEntry {
                nonce,
                owner,
                details,
            });
        }
        result
    }

    #[view(get_agent_count)]
    fn get_agent_count(&self) -> u64 {
        self.agents().len() as u64
    }

    #[view(get_agent_owner)]
    fn get_agent_owner(&self, nonce: u64) -> ManagedAddress {
        require!(self.agents().contains_id(&nonce), "Agent not found");
        self.agents().get_value(&nonce)
    }

    #[view(get_metadata)]
    fn get_metadata(&self, nonce: u64, key: ManagedBuffer) -> OptionalValue<ManagedBuffer> {
        let mapper = self.agent_metadata(nonce);
        if let Some(value) = mapper.get(&key) {
            OptionalValue::Some(value)
        } else {
            OptionalValue::None
        }
    }

    /// Paginated metadata entries for an agent. `from` = start index, `size` = max items (capped at 100).
    #[view(get_agent_metadata_page)]
    fn get_agent_metadata_page(
        &self,
        nonce: u64,
        from: u64,
        size: u64,
    ) -> ManagedVec<MetadataEntry<Self::Api>> {
        let size = size.min(100) as usize;
        let from = from.min(usize::MAX as u64) as usize;
        let mut result = ManagedVec::new();
        for (key, value) in self.agent_metadata(nonce).iter().skip(from).take(size) {
            result.push(MetadataEntry { key, value });
        }
        result
    }

    #[view(get_agent_service_config)]
    fn get_agent_service_config(
        &self,
        nonce: u64,
        service_id: u32,
    ) -> OptionalValue<EgldOrEsdtTokenPayment<Self::Api>> {
        let mapper = self.agent_service_config(nonce);
        if let Some(payment) = mapper.get(&service_id) {
            OptionalValue::Some(EgldOrEsdtTokenPayment::new(
                EgldOrEsdtTokenIdentifier::from(payment.token_identifier),
                payment.token_nonce,
                payment.amount.into_big_uint(),
            ))
        } else {
            OptionalValue::None
        }
    }

    /// Paginated service configs for an agent. `from` = start index, `size` = max items (capped at 100).
    #[view(get_agent_service_configs_page)]
    fn get_agent_service_configs_page(
        &self,
        nonce: u64,
        from: u64,
        size: u64,
    ) -> ManagedVec<ServiceConfigEntry<Self::Api>> {
        let size = size.min(100) as usize;
        let from = from.min(usize::MAX as u64) as usize;
        let mut result = ManagedVec::new();
        for (service_id, payment) in self.agent_service_config(nonce).iter().skip(from).take(size) {
            result.push(ServiceConfigEntry {
                service_id,
                payment: EgldOrEsdtTokenPayment::new(
                    EgldOrEsdtTokenIdentifier::from(payment.token_identifier),
                    payment.token_nonce,
                    payment.amount.into_big_uint(),
                ),
            });
        }
        result
    }
}
