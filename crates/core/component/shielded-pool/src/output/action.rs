use std::convert::{TryFrom, TryInto};

use anyhow::{Context, Error};
use bytes::Bytes;
use penumbra_asset::balance;
use penumbra_keys::symmetric::{OvkWrappedKey, WrappedMemoKey};
use penumbra_proto::{
    core::crypto::v1alpha1 as pbc, core::transaction::v1alpha1 as pb, DomainType, TypeUrl,
};
use serde::{Deserialize, Serialize};

use crate::{NotePayload, OutputProof};

#[derive(Clone, Debug)]
pub struct Output {
    pub body: Body,
    pub proof: OutputProof,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "pb::OutputBody", into = "pb::OutputBody")]
pub struct Body {
    pub note_payload: NotePayload,
    pub balance_commitment: balance::Commitment,
    pub ovk_wrapped_key: OvkWrappedKey,
    pub wrapped_memo_key: WrappedMemoKey,
}

impl TypeUrl for Output {
    const TYPE_URL: &'static str = "/penumbra.core.transaction.v1alpha1.Output";
}

impl DomainType for Output {
    type Proto = pb::Output;
}

impl From<Output> for pb::Output {
    fn from(output: Output) -> Self {
        let proof: pbc::ZkOutputProof = output.proof.into();
        pb::Output {
            body: Some(output.body.into()),
            proof: Some(proof),
        }
    }
}

impl TryFrom<pb::Output> for Output {
    type Error = Error;

    fn try_from(proto: pb::Output) -> anyhow::Result<Self, Self::Error> {
        Ok(Output {
            body: proto
                .body
                .ok_or_else(|| anyhow::anyhow!("missing output body"))?
                .try_into()?,
            proof: proto
                .proof
                .ok_or_else(|| anyhow::anyhow!("missing output proof"))?
                .try_into()
                .context("output proof malformed")?,
        })
    }
}

impl TypeUrl for Body {
    const TYPE_URL: &'static str = "/penumbra.core.transaction.v1alpha1.OutputBody";
}

impl DomainType for Body {
    type Proto = pb::OutputBody;
}

impl From<Body> for pb::OutputBody {
    fn from(output: Body) -> Self {
        pb::OutputBody {
            note_payload: Some(output.note_payload.into()),
            balance_commitment: Some(output.balance_commitment.into()),
            wrapped_memo_key: Bytes::copy_from_slice(&output.wrapped_memo_key.0),
            ovk_wrapped_key: Bytes::copy_from_slice(&output.ovk_wrapped_key.0),
        }
    }
}

impl TryFrom<pb::OutputBody> for Body {
    type Error = Error;

    fn try_from(proto: pb::OutputBody) -> anyhow::Result<Self, Self::Error> {
        let note_payload = proto
            .note_payload
            .ok_or_else(|| anyhow::anyhow!("missing note payload"))?
            .try_into()
            .context("malformed note payload")?;

        let wrapped_memo_key = proto.wrapped_memo_key[..]
            .try_into()
            .context("malformed wrapped memo key")?;

        let ovk_wrapped_key: OvkWrappedKey = proto.ovk_wrapped_key[..]
            .try_into()
            .context("malformed ovk wrapped key")?;

        let balance_commitment = proto
            .balance_commitment
            .ok_or_else(|| anyhow::anyhow!("missing value commitment"))?
            .try_into()
            .context("malformed balance commitment")?;

        Ok(Body {
            note_payload,
            wrapped_memo_key,
            ovk_wrapped_key,
            balance_commitment,
        })
    }
}
