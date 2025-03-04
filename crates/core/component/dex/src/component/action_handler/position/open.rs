use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use penumbra_component::ActionHandler;
use penumbra_storage::{StateRead, StateWrite};

use crate::{
    component::{PositionManager, PositionRead},
    event,
    lp::action::PositionOpen,
};

#[async_trait]
/// Debits the initial reserves and credits an opened position NFT.
impl ActionHandler for PositionOpen {
    type CheckStatelessContext = ();
    async fn check_stateless(&self, _context: ()) -> Result<()> {
        // TODO(chris, erwan, henry): brainstorm safety on `TradingFunction`.
        // Check:
        //  + reserves are at most 80 bits wide,
        //  + the trading function coefficients are at most 80 bits wide.
        //  + at least some assets are provisioned.
        //  + the trading function coefficients are non-zero,
        //  + the trading function doesn't specify a cyclic pair,
        //  + the fee is <=50%.
        self.position.check_stateless()?;
        Ok(())
    }

    async fn check_stateful<S: StateRead + 'static>(&self, state: Arc<S>) -> Result<()> {
        // Validate that the position ID doesn't collide
        state.check_position_id_unused(&self.position.id()).await?;

        Ok(())
    }

    async fn execute<S: StateWrite>(&self, mut state: S) -> Result<()> {
        state.put_position(self.position.clone()).await?;
        state.record(event::position_open(self));
        Ok(())
    }
}
