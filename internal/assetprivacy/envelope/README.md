# Candidate note encryption

Available only with `-tags algorand_native_privacy`. This local cryptographic
component is not called by native transactions yet. It follows
[candidate profile revision 1](../../../docs/native-asset-privacy/CANDIDATE_PROFILE.md).

Use `GenerateKey` for an ML-KEM-768 recipient key. Store the key's `Bytes()` seed
only in encrypted local wallet storage; `RestoreKey` accepts that canonical
64-byte seed. Publish only `key.EncapsulationKey().Bytes()`. Viewing keys do not
grant spending authority; the future note relation must preserve that separation.
These helpers generate or restore standard ML-KEM keys without resampling.
Call `CheckPublicKey(key.EncapsulationKey().Bytes())` before candidate key
registration or use; generation and restoration do not imply admission.

To encrypt:

1. Build a `Context` from pre-encryption values, including the network,
   asset, policy digest, key epoch, role, fresh transaction nonce, output index
   and output commitment. The caller generates the transaction nonce securely
   and never reuses an output identifier.
2. Encode and pad the note to exactly `PayloadSize` (512) bytes. The canonical
   [note API](../note/README.md) defines the candidate layout and checks zero
   padding. Its matching proof constraints remain pending.
3. Call `Seal(recipientPublicKey, context, payload)`. Each call generates fresh
   ML-KEM encapsulation randomness and a fresh GCM nonce internally.
4. Finalize the group using the returned ciphertext, then prove the full relation.
   Keep the key epoch private with the recipient key; output records omit it.
   Do not include the resulting ciphertext or final group ID in `Context`.

Call `Open(secretKey, context, envelope)` locally to authenticate and decrypt.
The returned payload is exactly 512 bytes; no plaintext is returned on an
authentication failure. Treat plaintext and decapsulation seeds as secrets.
The package does not log them or send them over RPC. Clearing temporary derived
keys is best effort, not a guarantee of complete Go heap erasure.

`CheckPublicKey` requires 1184 bytes, checks all 768 packed 12-bit public
coefficients are below 3329, and checks the final 32-byte seed's nine matrix
polynomials. For each `first,second` in `0..2`, it reads exactly
`MatrixPrefixBlocks * 168` bytes of SHAKE128(seed || first || second), with
`MatrixPrefixBlocks=16`, and requires at least 256 contiguous 12-bit candidates
below 3329. The fixed bound cannot come from a proof or caller. The work is
bounded to nine 2688-byte prefixes, each containing 1792 candidates; no partial
polynomial, padding or alternate rejection rule is accepted.

Matrix-prefix exhaustion is rejected even for otherwise standard-valid keys.
This candidate protocol admits only keys whose nine matrices finish within the
fixed prefix. The bounded matrix relation covers admitted keys; coverage of all
standard keys is not established.
The native proof profile must fix the same maximum 16 blocks and bind the
key's seed, ordered matrix coordinates, exact SHAKE prefix and first-accepted
sampling semantics. Admission does not prove the whole encryption relation.
`ParsePublicKey` runs `CheckPublicKey` before standard ML-KEM parsing, bounding
that parser's internal SampleNTT work. Viewing-key records use this parser.

Envelope size and version are checked before decapsulation. All altered-envelope,
wrong-key and AEAD authentication failures return `ErrCiphertext`. Invalid caller
context and nil keys have separate errors. Use `errors.Is` to classify errors.

The fixed wire length is `EncodedSize` (1629 bytes). The format carries no public
recipient key or recipient identifier. This does not prove key privacy or
recipient anonymity. Sending a valid encrypted blob also does not prove it
contains the correct note: the composed STARK relation must enforce that.

Build/static checks, without running tests:

```sh
go build -mod=readonly -tags algorand_native_privacy ./internal/assetprivacy/envelope
go vet -mod=readonly -tags algorand_native_privacy ./internal/assetprivacy/envelope
```

Tests, cryptographic vectors, interoperability and adversarial validation are
deferred by user instruction. Do not treat those commands as their substitute.
