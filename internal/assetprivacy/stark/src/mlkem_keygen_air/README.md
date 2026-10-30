# Exact public-key generation from a private seed

`MlKemKeygenAir` constrains candidate ML-KEM-768 public-key generation from a
private32-byte derivation seed. It binds the exact1184-byte public key and a
finalized64-byte group digest. Native viewing registration now selects this
relation as kind22 through the real verifier bridge. It remains disabled source
integration, without approved artifacts or a demonstrated accepted registration.

## Public constructor and private relation

The constructor parses every packed12-bit public coefficient canonically below
3329 and extracts public rho from the key's final32 bytes. Because registration
publishes this key, the trusted verifier can derive the public matrix directly
from rho using the candidate's bounded SHAKE/rejection sampler. It fixes all nine
polynomials with key-generation ordering `A[row,column]=SampleNTT(rho||column||row)`.
This does not replace a private witness constraint: the seed stage's proven rho
must equal that same public rho throughout the seed/noise/transform frame.

The existing seed composition proves SHA3-512 of `d32||byte3`, six eta2 noise
polynomials from the resulting sigma/nonces0..5, and six forward NTTs. The first
three transformed vectors are secret polynomials; the last three are errors.
The parent binds the same secret vectors and public matrix to the existing
modular product child. Its matrix adapter transposes the child's column-oriented
input so that the output is the required key-generation row product.

For every coefficient the parent checks `product+error=t_hat+carry*3329`, with a
Boolean carry and canonical child operands. This binds all public packed
coefficients to those proven vectors. A host preparation check cannot substitute
for these constraints. Every public-key byte is also an exact public field value;
the remaining32 public values encode the finalized group as little-endian16-bit
limbs. Group registers and transformed vectors stay constant across all rows.

## Schedule, interfaces and bounds

| Rows | Main columns | Preprocessing columns | Public values |
| ---: | ---: | ---: | ---: |
| 16384 | 9394 | 723 | 1216 |

The first10240 rows contain the compact seed/noise/six-NTT frame. A2048-row
product frame follows, then4096 inactive rows. Preprocessing combines570 core
columns,145 product columns and two sets of four trusted parent frame flags.
Workspace reuse preserves the transformed tail while product computation runs.

`new(public_key)` rejects noncanonical or bounded-sampling-inadmissible public
keys. `public_values(statement)` rejects a zero group. `trace(seed,statement)`
prepares matching private inputs before allocating its matrix, then writes child
frames directly and fills the shared global registers. No complete child main
matrix is allocated alongside it. No trace constructor has been executed.

Native selection reconstructs this exact key from the expected canonical
lifecycle statement. The finalized group binds role/account/policy/epoch and
execution requires current PQ account authorization. The selected relation
identity includes the entire public key and native domain. Independently supplied
keys or statements cannot establish these native bindings. See the
[viewing registration contract](../../../../../docs/native-asset-privacy/VIEWING_REGISTRATION.md).

## Evidence limits

The implementation follows the candidate interpretation of
[FIPS 203 Algorithm13](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.203.pdf).
It is an unreviewed source relation. Knowledge of `d` determines the K-PKE secret
polynomials; this component does not generate a full ML-KEM decapsulation-key
encoding or validate entropy. All current consensus flags remain false and all
suite catalogs empty. No artifact approval is implied.

Build/static checks establish compilation and source structure only. No tests,
vectors, traces, constraint evaluations, setup, proofs, complete key pairs or node
runs occurred. Runtime feasibility, interoperability and cryptographic security
remain unverified. See the [complete possession contract](../../../../../docs/native-asset-privacy/KEYGEN_POSSESSION.md).
