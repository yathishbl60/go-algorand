# Candidate note encryption

Available only with `-tags algorand_native_privacy`. This local cryptographic
component is not called by native transactions yet. It follows
[candidate profile revision 1](../../../docs/native-asset-privacy/CANDIDATE_PROFILE.md).

Use `GenerateKey` for an ML-KEM-768 recipient key. Store the key's `Bytes()` seed
only in encrypted local wallet storage; `RestoreKey` accepts that canonical
64-byte seed. Publish only `key.EncapsulationKey().Bytes()`. Viewing keys do not
grant spending authority; the future note relation must preserve that separation.

To encrypt:

1. Build a `Context` from public pre-encryption values, including the network,
   asset, policy digest, key epoch, role, fresh transaction nonce, output index
   and output commitment. The caller generates the transaction nonce securely
   and never reuses an output identifier.
2. Encode and pad the note to exactly `PayloadSize` (512) bytes. The canonical
   [note API](../note/README.md) defines the candidate layout and checks zero
   padding. Its matching proof constraints remain pending.
3. Call `Seal(recipientPublicKey, context, payload)`. Each call generates fresh
   ML-KEM encapsulation randomness and a fresh GCM nonce internally.
4. Finalize the group using the returned ciphertext, then prove the full relation.
   Do not include the resulting ciphertext or final group ID in `Context`.

Call `Open(secretKey, context, envelope)` locally to authenticate and decrypt.
The returned payload is exactly 512 bytes; no plaintext is returned on an
authentication failure. Treat plaintext and decapsulation seeds as secrets.
The package does not log them or send them over RPC. Clearing temporary derived
keys is best effort, not a guarantee of complete Go heap erasure.

Public keys undergo ML-KEM length/modulus validation before encapsulation.
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
