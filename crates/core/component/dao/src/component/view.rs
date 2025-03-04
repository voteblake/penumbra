use std::{collections::BTreeMap, str::FromStr};

use anyhow::Result;
use async_trait::async_trait;

use futures::{StreamExt, TryStreamExt};
use penumbra_asset::{asset, Value};
use penumbra_num::Amount;
use penumbra_proto::{StateReadProto, StateWriteProto};
use penumbra_storage::{StateRead, StateWrite};

use crate::params::DaoParameters;

use super::state_key;

#[async_trait]
pub trait StateReadExt: StateRead {
    /// Indicates if the DAO parameters have been updated in this block.
    fn dao_params_updated(&self) -> bool {
        self.object_get::<()>(state_key::dao_params_updated())
            .is_some()
    }

    /// Gets the DAO parameters from the JMT.
    async fn get_dao_params(&self) -> Result<DaoParameters> {
        self.get(state_key::dao_params())
            .await?
            .ok_or_else(|| anyhow::anyhow!("Missing DaoParameters"))
    }

    async fn dao_asset_balance(&self, asset_id: asset::Id) -> Result<Amount> {
        Ok(self
            .get(&state_key::balance_for_asset(asset_id))
            .await?
            .unwrap_or_else(|| Amount::from(0u64)))
    }

    async fn dao_balance(&self) -> Result<BTreeMap<asset::Id, Amount>> {
        let prefix = state_key::all_assets_balance();
        self.prefix(prefix)
            .map(|result| {
                let (key, amount) = result?;
                let asset_id = key.rsplit('/').next().expect("key is well-formed");
                let asset_id = asset::Id::from_str(asset_id)?;
                Ok((asset_id, amount))
            })
            .try_collect()
            .await
    }
}

impl<T> StateReadExt for T where T: StateRead + ?Sized {}

#[async_trait]
pub trait StateWriteExt: StateWrite {
    /// Writes the provided DAO parameters to the JMT.
    fn put_dao_params(&mut self, params: DaoParameters) {
        // Note that the dao params have been updated:
        self.object_put(state_key::dao_params_updated(), ());

        // Change the DAO parameters:
        self.put(state_key::dao_params().into(), params)
    }

    async fn dao_deposit(&mut self, value: Value) -> Result<()> {
        let key = state_key::balance_for_asset(value.asset_id);
        let current = self.get(&key).await?.unwrap_or_else(|| Amount::from(0u64));
        self.put(key, current + value.amount);
        Ok(())
    }

    async fn dao_withdraw(&mut self, value: Value) -> Result<()> {
        let key = state_key::balance_for_asset(value.asset_id);
        let current = self.get(&key).await?.unwrap_or_else(|| Amount::from(0u64));
        if let Some(remaining) = u128::from(current).checked_sub(u128::from(value.amount)) {
            if remaining > 0 {
                self.put(key, Amount::from(remaining));
            } else {
                self.delete(key);
            }
        } else {
            anyhow::bail!(
                "insufficient balance to withdraw {} of asset ID {} from the DAO",
                value.amount,
                value.asset_id
            );
        }
        Ok(())
    }
}

impl<T> StateWriteExt for T where T: StateWrite + ?Sized {}
