use std::str::FromStr;

use rand_core::OsRng;
use wasm_bindgen::prelude::*;

use penumbra_keys::keys::{Bip44Path, SeedPhrase, SpendKey};
use penumbra_keys::{Address, FullViewingKey};
use penumbra_proto::{core::keys::v1alpha1 as pb, serializers::bech32str, DomainType};
use wasm_bindgen_futures::js_sys::Uint8Array;
use wasm_bindgen_test::console_log;
use penumbra_proof_params::{
    SPEND_PROOF_PROVING_KEY, 
    OUTPUT_PROOF_PROVING_KEY, 
    DELEGATOR_VOTE_PROOF_PROVING_KEY, 
    NULLIFIER_DERIVATION_PROOF_PROVING_KEY, 
    SWAP_PROOF_PROVING_KEY, 
    SWAPCLAIM_PROOF_PROVING_KEY, 
    UNDELEGATECLAIM_PROOF_PROVING_KEY
};

use crate::error::WasmResult;
use crate::utils;

#[wasm_bindgen]
pub fn load_proving_key(parameters: JsValue, key_type: &str) {
    // Deserialize JsValue into &[u8]. 
    let parameters_array = Uint8Array::new(&parameters);
    let parameters_bytes: Vec<u8> = parameters_array.to_vec();
    let parameters_slice: &[u8] = &parameters_bytes;

    match key_type {
        "spend" => {
            SPEND_PROOF_PROVING_KEY.try_load(parameters_slice);
        }
        "output" => {
            OUTPUT_PROOF_PROVING_KEY.try_load(parameters_slice);
        }
        "delegator_vote" => {
            DELEGATOR_VOTE_PROOF_PROVING_KEY.try_load(parameters_slice);
        }
        "nullifier_derivation" => {
            NULLIFIER_DERIVATION_PROOF_PROVING_KEY.try_load(parameters_slice);
        }
        "swap" => {
            SWAP_PROOF_PROVING_KEY.try_load(parameters_slice);
        }
        "swap_claim" => {
            SWAPCLAIM_PROOF_PROVING_KEY.try_load(parameters_slice);
        }
        "undelegate_claim" => {
            UNDELEGATECLAIM_PROOF_PROVING_KEY.try_load(parameters_slice);
        }
        _ => {
            console_log!("Unknown key type!");
        }
    }
}

/// generate a spend key from a seed phrase
/// Arguments:
///     seed_phrase: `string`
/// Returns: `bech32 string`
#[wasm_bindgen]
pub fn generate_spend_key(seed_phrase: &str) -> WasmResult<JsValue> {
    utils::set_panic_hook();

    let seed = SeedPhrase::from_str(seed_phrase)?;
    let path = Bip44Path::new(0);
    let spend_key = SpendKey::from_seed_phrase_bip44(seed, &path);

    let proto = spend_key.to_proto();

    let spend_key_str = bech32str::encode(
        &proto.inner,
        bech32str::spend_key::BECH32_PREFIX,
        bech32str::Bech32m,
    );

    Ok(JsValue::from_str(&spend_key_str))
}

/// get full viewing key from spend key
/// Arguments:
///     spend_key_str: `bech32 string`
/// Returns: `bech32 string`
#[wasm_bindgen]
pub fn get_full_viewing_key(spend_key: &str) -> WasmResult<JsValue> {
    utils::set_panic_hook();

    let spend_key = SpendKey::from_str(spend_key)?;

    let fvk: &FullViewingKey = spend_key.full_viewing_key();

    let proto = fvk.to_proto();

    let fvk_bech32 = bech32str::encode(
        &proto.inner,
        bech32str::full_viewing_key::BECH32_PREFIX,
        bech32str::Bech32m,
    );
    Ok(JsValue::from_str(&fvk_bech32))
}

/// Wallet id: the hash of a full viewing key, used as an account identifier
/// Arguments:
///     full_viewing_key: `bech32 string`
/// Returns: `bech32 string`
#[wasm_bindgen]
pub fn get_wallet_id(full_viewing_key: &str) -> WasmResult<String> {
    utils::set_panic_hook();

    let fvk = FullViewingKey::from_str(full_viewing_key)?;
    Ok(fvk.wallet_id().to_string())
}

/// get address by index using FVK
/// Arguments:
///     full_viewing_key: `bech32 string`
///     index: `u32`
/// Returns: `pb::Address`
#[wasm_bindgen]
pub fn get_address_by_index(full_viewing_key: &str, index: u32) -> WasmResult<JsValue> {
    utils::set_panic_hook();

    let fvk = FullViewingKey::from_str(full_viewing_key)?;
    let (address, _dtk) = fvk.incoming().payment_address(index.into());
    let proto = address.to_proto();
    let result = serde_wasm_bindgen::to_value(&proto)?;
    Ok(result)
}

/// get ephemeral (randomizer) address using FVK
/// The derivation tree is like "spend key / address index / ephemeral address" so we must also pass index as an argument
/// Arguments:
///     full_viewing_key: `bech32 string`
///     index: `u32`
/// Returns: `pb::Address`
#[wasm_bindgen]
pub fn get_ephemeral_address(full_viewing_key: &str, index: u32) -> WasmResult<JsValue> {
    utils::set_panic_hook();

    let fvk = FullViewingKey::from_str(full_viewing_key)?;
    let (address, _dtk) = fvk.ephemeral_address(OsRng, index.into());
    let proto = address.to_proto();
    let result = serde_wasm_bindgen::to_value(&proto)?;
    Ok(result)
}

/// Check if the address is FVK controlled
/// Arguments:
///     full_viewing_key: `bech32 String`
///     address: `bech32 String`
/// Returns: `Option<pb::AddressIndex>`
#[wasm_bindgen]
pub fn is_controlled_address(full_viewing_key: &str, address: &str) -> WasmResult<JsValue> {
    utils::set_panic_hook();

    let fvk = FullViewingKey::from_str(full_viewing_key)?;
    let index: Option<pb::AddressIndex> = fvk
        .address_index(&Address::from_str(address)?)
        .map(Into::into);
    let result = serde_wasm_bindgen::to_value(&index)?;
    Ok(result)
}

/// Get canonical short form address by index
/// This feature is probably redundant and will be removed from wasm in the future
/// Arguments:
///     full_viewing_key: `bech32 string`
///     index: `u32`
/// Returns: `String`
#[wasm_bindgen]
pub fn get_short_address_by_index(full_viewing_key: &str, index: u32) -> WasmResult<JsValue> {
    utils::set_panic_hook();

    let fvk = FullViewingKey::from_str(full_viewing_key)?;
    let (address, _dtk) = fvk.incoming().payment_address(index.into());
    let short_address = address.display_short_form();
    Ok(JsValue::from_str(&short_address))
}
