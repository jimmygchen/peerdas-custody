//! The identifier for an ENR record. This is the keccak256 hash of the public key (for secp256k1
//! keys this is the uncompressed encoded form of the public key).

type RawNodeId = [u8; 32];

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
/// The `NodeId` of an ENR (a 32 byte identifier).
pub struct NodeId {
    #[cfg_attr(feature = "serde", serde(with = "serde_hex_prfx"))]
    raw: RawNodeId,
}

impl NodeId {
    /// Creates a new node record from 32 bytes.
    #[must_use]
    pub const fn new(raw_input: &[u8; 32]) -> Self {
        Self { raw: *raw_input }
    }

    /// Parses a byte slice to form a node Id. This fails if the slice isn't of length 32.
    pub fn parse(raw_input: &[u8]) -> Result<Self, &'static str> {
        if raw_input.len() > 32 {
            return Err("Input too large");
        }

        let mut raw: RawNodeId = [0_u8; 32];
        raw[..std::cmp::min(32, raw_input.len())].copy_from_slice(raw_input);

        Ok(Self { raw })
    }

    /// Returns a `RawNodeId` which is a 32 byte list.
    #[must_use]
    pub const fn raw(&self) -> RawNodeId {
        self.raw
    }
}

impl AsRef<[u8]> for NodeId {
    fn as_ref(&self) -> &[u8] {
        &self.raw[..]
    }
}

impl PartialEq<RawNodeId> for NodeId {
    fn eq(&self, other: &RawNodeId) -> bool {
        self.raw.eq(other)
    }
}

impl From<RawNodeId> for NodeId {
    fn from(raw: RawNodeId) -> Self {
        Self { raw }
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let hex_encode = hex::encode(self.raw);
        write!(
            f,
            "0x{}..{}",
            &hex_encode[0..4],
            &hex_encode[hex_encode.len() - 4..]
        )
    }
}

impl std::fmt::Debug for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{}", hex::encode(self.raw))
    }
}
