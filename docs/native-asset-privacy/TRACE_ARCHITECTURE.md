# Private trace scheduling direction

The fixed [GCM component](../../internal/assetprivacy/stark/src/gcm_air/README.md)
defines constraints, not a practical full trace. Repeating all logical fields
over 512 rows would consume about 16.49 GB of base elements before overhead.
The next layout uses vertical stages and direct private register transitions
inside one AIR. No native verifier or production resource budget exists yet.

## Supported pinned mechanisms

At Plonky3 revision `1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f`:

- [AirBuilder](https://github.com/Plonky3/Plonky3/blob/1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f/air/src/builder.rs)
  provides current/next windows and boundary/transition/conditional filters.
- [BaseAir](https://github.com/Plonky3/Plonky3/blob/1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f/air/src/air.rs)
  provides `preprocessed_trace` and matching `preprocessed_width`.
- [Preprocessing setup](https://github.com/Plonky3/Plonky3/blob/1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f/uni-stark/src/preprocessed.rs)
  returns prover data and a verifier key containing width, degree and commitment.
  Use `prove_with_preprocessed`/`verify_with_preprocessed` for those schedules.

The verifier must obtain the expected schedule commitment from trusted suite
configuration, never from a transaction's advertised key. Hiding PCS preprocessing
uses zero padding, but underlying hiding Merkle salts may still be randomized.
Do not assume independent setups yield identical roots. Canonical preparation,
artifact binding and private-prover randomness separation remain to be specified.

Periodic columns also exist. Their documentation requests `periodic_statement`
transcript binding, but source inspection found no invocation in this pinned
uni-STARK implementation. Do not assume table contents are automatically bound
merely because periodic count appears in the transcript shape. Initial vertical
work uses committed preprocessed schedules; the full suite must bind all semantics.

## Private buses need additional work

The pinned [batch proof](https://github.com/Plonky3/Plonky3/blob/1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f/batch-stark/src/proof.rs)
serializes one lookup terminal per AIR. The [LogUp generator](https://github.com/Plonky3/Plonky3/blob/1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f/lookup/src/logup.rs)
sets it to the full row-total sum without a terminal mask. Hiding polynomial
commitments do not hide an explicitly serialized witness-dependent value.

For a singleton private transfer with multiplicity one, the terminal is
`1 / (alpha - compressed_tuple)`. Known challenges and terminal reveal the
compressed tuple; a one-field tuple reveals that field. This is an inference
from the inspected expression, not an executed attack or a claim about every bus.
Therefore private cross-AIR buses are not an approved composition mechanism here.

Balanced same-AIR local lookups can have an identically zero terminal, avoiding
that particular disclosure. They still require constrained multiplicities,
selectors, tuple widths/count bounds and a hiding analysis for every auxiliary
commitment/opening. Zero terminal alone does not establish zero knowledge.
No new lookup dependency or acceptance path has been added.

## First implementation stages

1. [Vertical GHASH multiplication](../../internal/assetprivacy/stark/src/ghash_vertical/README.md)
   is implemented: private operands/output, 128 shift/select/XOR steps and padding.
2. The [37-block chain](../../internal/assetprivacy/stark/src/ghash_chain/README.md)
   now uses direct accumulator transitions and 1,537 columns over 8,192 rows.
3. [Staged GCM](../../internal/assetprivacy/stark/src/gcm_vertical/README.md)
   reuses AES scratch for GHASH and binds root constants and the complete tag.
4. Schedule KDF/hash/KEM/note operations and all shared private values coherently;
   avoid globally repeating entire paths or expanded block witnesses.
5. [Matrix sampling](MATRIX_SAMPLING.md) now joins SHAKE128 and rejection in one
   AIR with physically shared private prefix columns and sliced preprocessing.
   Direct writers avoid retaining separate full component matrices. The enclosing
   KEM must still bind all key seeds, coordinates and polynomial operations.

Recalculate polynomial degree, quotient chunks, masking/opening budgets and
trusted dimensions after each composition. Builds and static counts do not
establish practical proving time/memory, quantum soundness or witness privacy.
Tests, proofs and measurements remain deferred by user instruction.
