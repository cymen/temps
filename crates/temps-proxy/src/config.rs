// SPDX-FileCopyrightText: 2024-2026 Temps Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ProxyConfig {
    pub address: String,
    pub console_address: String,
    pub tls_address: Option<String>,
    pub preview_domain: Option<String>, // e.g., "preview.example.com"
    /// When true, HTTP requests are served directly without redirecting to HTTPS.
    /// Useful for local development without TLS certificates.
    pub disable_https_redirect: bool,
    /// Trust client IP headers from a loopback reverse proxy only when the
    /// operator has explicitly enabled and configured that proxy.
    pub trust_loopback_forwarded_ip: bool,
    /// On-demand HTTP-01 TLS certificate manager (ADR-018). `None` (default)
    /// disables on-demand issuance entirely — the proxy's TLS callback behaves
    /// exactly as before, returning `Ok(None)` with no side effect when no cert
    /// is found. When wired (only when `on_demand_tls_enabled` is set in
    /// settings), the TLS callback asks this manager to provision a cert in the
    /// background for allowlisted, stable, in-zone hostnames.
    pub on_demand_cert_manager: Option<Arc<crate::on_demand_cert::OnDemandCertManager>>,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            address: "127.0.0.1:8080".to_string(),
            console_address: "127.0.0.1:3000".to_string(),
            tls_address: None,
            preview_domain: Some("localhost".to_string()), // Default for local development
            disable_https_redirect: false,
            trust_loopback_forwarded_ip: false,
            on_demand_cert_manager: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ProxyConfig;

    #[test]
    fn forwarded_ip_trust_is_off_by_default() {
        assert!(!ProxyConfig::default().trust_loopback_forwarded_ip);
    }
}
