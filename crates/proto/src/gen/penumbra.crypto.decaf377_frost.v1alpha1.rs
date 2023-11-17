/// A commitment to a polynomial, as a list of group elements.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifiableSecretSharingCommitment {
    /// Each of these bytes should be the serialization of a group element.
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub elements: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// The public package sent in round 1 of the DKG protocol.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DkgRound1Package {
    /// A commitment to the polynomial for secret sharing.
    #[prost(message, optional, tag = "1")]
    pub commitment: ::core::option::Option<VerifiableSecretSharingCommitment>,
    /// A proof of knowledge of the underlying secret being shared.
    #[prost(bytes = "vec", tag = "2")]
    pub proof_of_knowledge: ::prost::alloc::vec::Vec<u8>,
}
/// A share of the final signing key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SigningShare {
    /// These bytes should be a valid scalar.
    #[prost(bytes = "vec", tag = "1")]
    pub scalar: ::prost::alloc::vec::Vec<u8>,
}
/// The per-participant package sent in round 2 of the DKG protocol.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DkgRound2Package {
    /// This is the share we're sending to that participant.
    #[prost(message, optional, tag = "1")]
    pub signing_share: ::core::option::Option<SigningShare>,
}
/// Represents a commitment to a nonce value.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NonceCommitment {
    /// These bytes should be a valid group element.
    #[prost(bytes = "vec", tag = "1")]
    pub element: ::prost::alloc::vec::Vec<u8>,
}
/// Represents the commitments to nonces needed for signing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SigningCommitments {
    /// One nonce to hide them.
    #[prost(message, optional, tag = "1")]
    pub hiding: ::core::option::Option<NonceCommitment>,
    /// Another to bind them.
    #[prost(message, optional, tag = "2")]
    pub binding: ::core::option::Option<NonceCommitment>,
}
/// Represents a package we use to produce a signature.
///
/// This is basically the package used for round 2 of signing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SigningPackage {
    /// Commitments along with an identifier for the person who made them.
    #[prost(message, repeated, tag = "1")]
    pub all_commitments: ::prost::alloc::vec::Vec<signing_package::Inner>,
    /// The message to be signed.
    #[prost(bytes = "vec", tag = "2")]
    pub message: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `SigningPackage`.
pub mod signing_package {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Inner {
        /// The identifier of the person who produced this commitment.
        ///
        /// This should not repeat across the package. If it does,
        /// the last appearance takes precedence.
        #[prost(bytes = "vec", tag = "1")]
        pub identifier: ::prost::alloc::vec::Vec<u8>,
        #[prost(message, optional, tag = "2")]
        pub commitments: ::core::option::Option<super::SigningCommitments>,
    }
}
/// A share of the final signature. These get aggregated to make the actual thing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignatureShare {
    /// These bytes should be a valid scalar.
    #[prost(bytes = "vec", tag = "1")]
    pub scalar: ::prost::alloc::vec::Vec<u8>,
}
