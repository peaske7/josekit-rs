use thiserror::Error;

#[derive(Error, Debug)]
pub enum JwtError {
    #[error("Unsupported signature algorithm: {0}")]
    UnsupportedSignatureAlgorithm(#[source] anyhow::Error),

    #[error("Invalid JWT format: {0}")]
    InvalidJwtFormat(#[source] anyhow::Error),

    #[error("Invalid key format: {0}")]
    InvalidKeyFormat(#[source] anyhow::Error),

    #[error("Invalid signature: {0}")]
    InvalidSignature(#[source] anyhow::Error),

    #[error("Invalid claim: {0}")]
    InvalidClaim(#[source] anyhow::Error),
}
