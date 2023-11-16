use anyhow::anyhow;
use penumbra_proto::{penumbra::custody::threshold::v1alpha1 as pb, DomainType, TypeUrl};
use penumbra_transaction::plan::TransactionPlan;

/// Represents the message sent by the coordinator at the start of the signing process.
///
/// This is nominally "round 1", even though it's the only message the coordinator ever sends.
#[derive(Debug, Clone)]
pub struct CoordinatorRound1 {
    plan: TransactionPlan,
}

impl CoordinatorRound1 {
    /// Construct a new round1 package given a transaction plan.
    pub fn new(plan: TransactionPlan) -> Self {
        Self { plan }
    }
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
