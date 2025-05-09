//! Reusable functionality related to [wasmCloud hosts][docs-wasmcloud-hosts]
//!
//! [docs-wasmcloud-hosts]: <https://wasmcloud.com/docs/concepts/hosts>

use std::collections::HashMap;

use secrecy::zeroize::{Zeroize, ZeroizeOnDrop};
use serde::{Deserialize, Serialize};

use crate::link::InterfaceLinkDefinition;
use crate::logging::Level;
use crate::otel::OtelConfig;
use crate::secrets::SecretValue;
use crate::wit::{deserialize_wit_map, serialize_wit_map, WitMap};

/// Environment settings for initializing a capability provider
pub type HostEnvValues = WitMap<String>;

/// initialization data for a capability provider
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct HostData {
    #[serde(default)]
    pub host_id: String,
    #[serde(default)]
    pub lattice_rpc_prefix: String,
    #[serde(default)]
    pub link_name: String,
    #[serde(default)]
    pub lattice_rpc_user_jwt: String,
    #[serde(default)]
    pub lattice_rpc_user_seed: String,
    #[serde(default)]
    pub lattice_rpc_url: String,
    #[serde(default)]
    pub provider_key: String,
    #[serde(
        serialize_with = "serialize_wit_map",
        deserialize_with = "deserialize_wit_map"
    )]
    pub env_values: HostEnvValues,
    #[serde(default)]
    pub instance_id: String,
    /// initial list of links for provider
    pub link_definitions: Vec<InterfaceLinkDefinition>,
    /// list of cluster issuers.
    #[serde(default)]
    pub cluster_issuers: Vec<String>,
    /// Merged named configuration set for this provider at runtime
    #[serde(default)]
    pub config: HashMap<String, String>,
    /// Secrets given to this provider at runtime
    #[serde(default)]
    pub secrets: HashMap<String, SecretValue>,
    /// The public key xkey of the host, used for decrypting secrets
    #[serde(default)]
    pub host_xkey_public_key: String,
    /// The private key xkey of the provider, used for decrypting secrets
    #[serde(default)]
    pub provider_xkey_private_key: String,
    /// Host-wide default RPC timeout for rpc messages, in milliseconds.  Defaults to 2000.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_rpc_timeout_ms: Option<u64>,
    /// True if structured logging is enabled for the host. Providers should use the same setting as the host.
    #[serde(default)]
    pub structured_logging: bool,
    /// The log level providers should log at
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_level: Option<Level>,
    #[serde(default)]
    pub otel_config: OtelConfig,
}

// Trait implementations that ensure we zeroize the memory of secrets when they are dropped
impl ZeroizeOnDrop for HostData {}
impl Zeroize for HostData {
    fn zeroize(&mut self) {
        self.provider_xkey_private_key.zeroize();
    }
}
