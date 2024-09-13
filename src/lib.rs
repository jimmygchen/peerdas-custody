mod node_id;

use std::collections::HashSet;
use std::str::FromStr;

use crate::node_id::NodeId;
use itertools::Itertools;
use libp2p::identity::{KeyType, PublicKey};
use libp2p::PeerId;
use primitive_types::U256;
use tiny_keccak::{Hasher, Keccak};
use wasm_bindgen::prelude::wasm_bindgen;

const DEFAULT_SUBNET_COUNT: u32 = 4;
const NUM_OF_COLUMNS: u32 = 128;
const DATA_COLUMN_SIDECAR_SUBNET_COUNT: u32 = 128;
const DATA_COLUMNS_PER_SUBNET: u32 = NUM_OF_COLUMNS / DATA_COLUMN_SIDECAR_SUBNET_COUNT;

#[wasm_bindgen]
pub fn get_data_column_sidecar_subnet_count() -> u32 {
    DATA_COLUMN_SIDECAR_SUBNET_COUNT
}

#[wasm_bindgen]
pub fn get_custody_subnets(node_id: &str, subnet_count: Option<u32>) -> Result<Vec<u32>, String> {
    let node_id = U256::from_str_radix(node_id, 16)
        .map_err(|e| format!("Unable to parse node id {node_id}: {e:?}"))?;
    let subnet_count = subnet_count.unwrap_or(DEFAULT_SUBNET_COUNT);
    Ok(compute_custody_subnets(node_id, subnet_count as u64)
        .map(|n| n as u32)
        .sorted()
        .collect::<Vec<_>>())
}

#[wasm_bindgen]
pub fn get_custody_subnets_from_peer_id(
    peer_id: &str,
    subnet_count: Option<u32>,
) -> Result<Vec<u32>, String> {
    let peer_id = PeerId::from_str(peer_id)
        .map_err(|e| format!("Unable to parse peer id {peer_id}: {e:?}"))?;
    let node_id = peer_id_to_node_id(&peer_id)?;
    let subnet_count = subnet_count.unwrap_or(DEFAULT_SUBNET_COUNT);
    Ok(
        compute_custody_subnets(node_id.raw().into(), subnet_count as u64)
            .map(|n| n as u32)
            .sorted()
            .collect::<Vec<_>>(),
    )
}

#[wasm_bindgen]
pub fn get_custody_columns(custody_subnets: Vec<u32>) -> Vec<u32> {
    custody_subnets
        .into_iter()
        .flat_map(columns)
        .sorted()
        .collect::<Vec<_>>()
}

/// Compute required subnets to subscribe to given the node id.
#[allow(clippy::arithmetic_side_effects)]
fn compute_custody_subnets(node_id: U256, custody_subnet_count: u64) -> impl Iterator<Item = u64> {
    let mut subnets: HashSet<u64> = HashSet::new();
    let mut current_id = node_id;
    while (subnets.len() as u64) < custody_subnet_count {
        let mut node_id_bytes = [0u8; 32];
        current_id.to_little_endian(&mut node_id_bytes);
        let hash = hash_fixed(&node_id_bytes);
        let hash_prefix = [
            hash[0], hash[1], hash[2], hash[3], hash[4], hash[5], hash[6], hash[7],
        ];
        let hash_prefix_u64 = u64::from_le_bytes(hash_prefix);
        let subnet = hash_prefix_u64 % (DATA_COLUMN_SIDECAR_SUBNET_COUNT as u64);

        if !subnets.contains(&subnet) {
            subnets.insert(subnet);
        }

        if current_id == U256::MAX {
            current_id = U256::zero()
        }
        current_id += U256::one()
    }
    subnets.into_iter()
}

fn hash_fixed(bytes: &[u8]) -> [u8; 32] {
    let mut context = ring::digest::Context::new(&ring::digest::SHA256);
    context.update(bytes);

    let mut output = [0; 32];
    output.copy_from_slice(context.finish().as_ref());
    output
}

// helper function to convert a peer_id to a node_id. This is only possible for secp256k1/ed25519 libp2p
// peer_ids
pub fn peer_id_to_node_id(peer_id: &PeerId) -> Result<NodeId, String> {
    // A libp2p peer id byte representation should be 2 length bytes + 4 protobuf bytes + compressed pk bytes
    // if generated from a PublicKey with Identity multihash.
    let pk_bytes = &peer_id.to_bytes()[2..];

    let public_key = PublicKey::try_decode_protobuf(pk_bytes).map_err(|e| {
        format!(
            " Cannot parse libp2p public key public key from peer id: {}",
            e
        )
    })?;

    match public_key.key_type() {
        KeyType::Secp256k1 => {
            let pk = public_key
                .clone()
                .try_into_secp256k1()
                .expect("right key type");
            let uncompressed_key_bytes = &pk.to_bytes_uncompressed()[1..];
            let mut output = [0_u8; 32];
            let mut hasher = Keccak::v256();
            hasher.update(uncompressed_key_bytes);
            hasher.finalize(&mut output);
            Ok(NodeId::parse(&output).expect("Must be correct length"))
        }
        KeyType::Ed25519 => {
            let pk = public_key
                .clone()
                .try_into_ed25519()
                .expect("right key type");
            let uncompressed_key_bytes = pk.to_bytes();
            let mut output = [0_u8; 32];
            let mut hasher = Keccak::v256();
            hasher.update(&uncompressed_key_bytes);
            hasher.finalize(&mut output);
            Ok(NodeId::parse(&output).expect("Must be correct length"))
        }

        _ => Err(format!("Unsupported public key from peer {}", peer_id)),
    }
}

#[allow(clippy::arithmetic_side_effects)]
fn columns(subnet: u32) -> impl Iterator<Item = u32> {
    (0..DATA_COLUMNS_PER_SUBNET).map(move |i| DATA_COLUMN_SIDECAR_SUBNET_COUNT * i + subnet)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_compute_subnets_for_data_column() {
        let node_id = "5e17a23d36023ab1106e4ef1cd8657f4214f60776a2602a5ea081fcee2c72b88";
        let peer_id = "16Uiu2HAmQH8aoyiLSo1JwhZ1fHVLhHsVYXiMumffa8DhwTgMkdRF";

        let custody_requirement = 4;
        let computed_subnets = get_custody_subnets(node_id, Some(custody_requirement)).unwrap();
        let computed_subnets_from_peer_id =
            get_custody_subnets_from_peer_id(peer_id, Some(custody_requirement)).unwrap();

        // results should match
        assert_eq!(computed_subnets, computed_subnets_from_peer_id);

        // the number of subnets is equal to the custody requirement
        assert_eq!(computed_subnets.len() as u32, custody_requirement);
        println!("subnets: {computed_subnets:?}");

        let subnet_count = DATA_COLUMN_SIDECAR_SUBNET_COUNT;
        for subnet in computed_subnets {
            let columns: Vec<_> = columns(subnet).collect();
            // the number of columns is equal to the specified number of columns per subnet
            assert_eq!(columns.len(), DATA_COLUMNS_PER_SUBNET as usize);

            for pair in columns.windows(2) {
                // each successive column index is offset by the number of subnets
                assert_eq!(pair[1] - pair[0], subnet_count);
            }
            println!("columns for subnet {subnet:?}: {columns:?}");
        }
    }

    #[test]
    fn test_peer_id_to_node_id() {
        let peer_id =
            PeerId::from_str("16Uiu2HAmQH8aoyiLSo1JwhZ1fHVLhHsVYXiMumffa8DhwTgMkdRF").unwrap();
        let expected_node_id = "0x5e17a23d36023ab1106e4ef1cd8657f4214f60776a2602a5ea081fcee2c72b88";
        let node_id = peer_id_to_node_id(&peer_id).unwrap();
        assert_eq!(format!("{node_id:?}"), expected_node_id);
    }
}
