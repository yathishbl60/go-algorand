# Typed candidate artifact selection

This opt-in module replaces arbitrary public AIR/key/relation-label construction.
It provides `Prepared`, `Domain`, `Layout`, `PublicSetup`, `Entry`, `Pin`, `Registry`,
`encode_registry` and `registry_digest`. No approved registry/pins or runtime proof
evidence exists. The [native bridge](../../../../../../docs/native-asset-privacy/NATIVE_VERIFIER_BRIDGE.md) links; approved suite admission/transaction routing remain.

Use `Prepared::transfer(domain, public_conversion)`, `unshield(domain, credit)`,
`recovery(domain, operation, authority)`, or output factories `recipient(context)`,
`disclosure(context, authority)`, `retained(context)` and `recovered(context)`.
Each produces the exact concrete AIR and canonical full relation ID. There is no
generic public constructor or mutable AIR accessor. `air()` borrows for proving;
`layout()` and `relation_id()` return public metadata.

`Registry::load(bytes, Pin{profile,implementation,digest})` requires independent
trusted pins and the current compiled source identity. It validates the fixed
954-byte format, exact ten layout records and file digest before publishing keys.
Unknown kinds, reordered records, incorrect dimensions/next-row flags or trailing
bytes fail. Root words are raw SHA3 words; zero is a valid output, not a key default.

`Registry::bind(prepared)` returns a verifier with no caller key/row/label override.
The complete artifact ID binds profile, whole registry, compiled source, complete
relation, dimensions/configuration and selected root. Bundle selection must use the
same registry/profile for every component; equal preprocessing roots do not merge
different relation kinds or permit a weaker output proof.

For offline public setup only, call `prepared.setup_public()` and retain its public
`data` and immutable `entry()`. This executes preprocessing and can require large
resources; it has not been run. Gather one Entry per kind in canonical order for
`encode_registry(profile, entries)`, then arrange independent approval of file pins.
Encoding or hashing a file does not approve its source, relation, parameters or keys.
`Registry::check_setup(&setup)` compares public width/extended-degree/commitment
before private proving; it does not validate every element of reusable prover data.

Each public setup call resets private-to-the-crate deterministic RNGs, used only
for public schedule commitments. Those RNGs/Config are never returned. Salt width
remains five. Private proof masking and salts must use fresh OS-backed
`prover_config()` separately. The native verifier and build checks never invoke setup.

[The selection contract](../../../../../../docs/native-asset-privacy/ARTIFACT_SELECTION.md)
defines exact relation preimages, source hashing, profile trust and missing native
integration. [The Go registry reader](../../../../artifactregistry/README.md)
documents the shared file format and dimensions. Cross-language conformance,
preprocessing, constraint/proof correctness, security and resource use remain
unverified; no tests or proofs have been run. Both feature defaults remain off.
