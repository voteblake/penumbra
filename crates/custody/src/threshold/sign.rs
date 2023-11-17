use anyhow::{anyhow, Result};
use decaf377_frost as frost;
use ed25519_consensus::{Signature, VerificationKey};
use penumbra_proto::{penumbra::custody::threshold::v1alpha1 as pb, DomainType, TypeUrl};
use penumbra_transaction::plan::TransactionPlan;

use super::config::Config;

/// Represents the message sent by the coordinator at the start of the signing process.
///
/// This is nominally "round 1", even though it's the only message the coordinator ever sends.
#[derive(Debug, Clone)]
pub struct CoordinatorRound1 {
    plan: TransactionPlan,
}

impl From<CoordinatorRound1> for pb::CoordinatorRound1 {
    fn from(value: CoordinatorRound1) -> Self {
        Self {
            plan: Some(value.plan.into()),
        }
    }
}

impl TryFrom<pb::CoordinatorRound1> for CoordinatorRound1 {
    type Error = anyhow::Error;

    fn try_from(value: pb::CoordinatorRound1) -> Result<Self, Self::Error> {
        Ok(Self {
            plan: value.plan.ok_or(anyhow!("missing plan"))?.try_into()?,
        })
    }
}

impl TypeUrl for CoordinatorRound1 {
    const TYPE_URL: &'static str = "/penumbra.custody.threshold.v1alpha1.CoordinatorRound1";
}

impl DomainType for CoordinatorRound1 {
    type Proto = pb::CoordinatorRound1;
}

pub struct CoordinatorRound2 {}

/// The message sent by the followers in round1 of signing.
#[derive(Debug, Clone)]
pub struct FollowerRound1 {
    /// A commitment for each spend we need to authorize.
    commitments: Vec<frost::round1::SigningCommitments>,
    /// A verification key identifying who the sender is.
    pk: VerificationKey,
    /// The signature over the protobuf encoding of the commitments.
    sig: Signature,
}

impl From<FollowerRound1> for pb::FollowerRound1 {
    fn from(value: FollowerRound1) -> Self {
        Self {
            inner: Some(pb::follower_round1::Inner {
                commitments: value.commitments.into_iter().map(|x| x.into()).collect(),
            }),
            pk: Some(pb::VerificationKey {
                inner: value.pk.to_bytes().to_vec(),
            }),
            sig: Some(pb::Signature {
                inner: value.sig.to_bytes().to_vec(),
            }),
        }
    }
}

impl TryFrom<pb::FollowerRound1> for FollowerRound1 {
    type Error = anyhow::Error;

    fn try_from(value: pb::FollowerRound1) -> Result<Self, Self::Error> {
        Ok(Self {
            commitments: value
                .inner
                .ok_or(anyhow!("missing inner"))?
                .commitments
                .into_iter()
                .map(|x| x.try_into())
                .collect::<Result<Vec<_>, _>>()?,
            pk: value
                .pk
                .ok_or(anyhow!("missing pk"))?
                .inner
                .as_slice()
                .try_into()?,
            sig: value
                .sig
                .ok_or(anyhow!("missing sig"))?
                .inner
                .as_slice()
                .try_into()?,
        })
    }
}

impl TypeUrl for FollowerRound1 {
    const TYPE_URL: &'static str = "/penumbra.custody.threshold.v1alpha1.FollowerRound1";
}

impl DomainType for FollowerRound1 {
    type Proto = pb::FollowerRound1;
}

#[derive(Debug, Clone)]
pub struct FollowerRound2 {
    /// One share of the signature for each authorization we need
    shares: Vec<frost::round2::SignatureShare>,
}

impl From<FollowerRound2> for pb::FollowerRound2 {
    fn from(value: FollowerRound2) -> Self {
        Self {
            shares: value.shares.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl TryFrom<pb::FollowerRound2> for FollowerRound2 {
    type Error = anyhow::Error;

    fn try_from(value: pb::FollowerRound2) -> Result<Self, Self::Error> {
        Ok(Self {
            shares: value
                .shares
                .into_iter()
                .map(|x| x.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TypeUrl for FollowerRound2 {
    const TYPE_URL: &'static str = "/penumbra.custody.threshold.v1alpha1.FollowerRound2";
}

impl DomainType for FollowerRound2 {
    type Proto = pb::FollowerRound2;
}

pub struct CoordinatorState {}

pub struct FollowerState {}

pub fn coordinator_round1(
    config: &Config,
    plan: TransactionPlan,
) -> Result<(CoordinatorRound1, CoordinatorState)> {
    todo!()
}

pub fn coordinator_round2(
    config: &Config,
    follower_messages: &[FollowerRound1],
) -> Result<CoordinatorRound2> {
    todo!()
}

pub fn follower_round1(
    config: &Config,
    coordinator: CoordinatorRound1,
) -> Result<(FollowerRound1, FollowerState)> {
    todo!()
}

pub fn follower_round2(config: &Config, state: FollowerState) -> Result<FollowerRound2> {
    todo!()
}
