# Candidate GHASH constraints

Reference: [NIST SP800-38D](https://doi.org/10.6028/NIST.SP.800-38D), November 2007,
sections 6.3 and 6.4, Algorithms 1 and 2. `Gf128` constrains block multiplication and
`Ghash<T,N>` constrains a positive, verifier-fixed number of complete blocks.

Block index 0 is the most significant bit of the first transmitted byte; index 127
is the last byte's least significant bit. This is deliberately different from
the AES component's per-byte bit ordering. The first transmitted bit represents
the constant polynomial coefficient in the GHASH convention.
`from_aes_bits` and `to_aes_bits` reorder existing column references between these
layouts without adding witness columns or trusting a new claimed value.

Multiplication starts with zero accumulator and right operand as the first
power. For each left-input bit, a separately constrained selected block contains
the corresponding power or zero. Boolean XOR intermediates accumulate that
selection. The next power shifts right and conditionally XORs
`0xe1000000000000000000000000000000`, using the previous least significant bit.
Every bit is Boolean; all ungated constraints have degree at most two.

`Gf128::sums[127]` is the product. Its 49,280 columns comprise two input blocks,
127 shifted powers, 128 selected blocks and 128 sums. An enclosing AIR must bind
`left`, `right` and that final product to its intended values; no public digest
or independent duplicate output is silently trusted.

`Ghash` explicitly binds each product's right operand to the same private hash
subkey H. Its left operand is bound to the previous product XOR the current
block, starting from zero. The declared output is bound to the last product.
All input-block, subkey and output bits are range constrained. Zero blocks and
zero H are legitimate inputs. N=0 is rejected by witness construction and makes
the evaluator unsatisfiable. N must come from the verifier's relation definition.

`Gf128<Value>::witness` accepts two 16-byte arrays. `Ghash<Value,N>::witness`
accepts a 16-byte H and an array of N complete 16-byte blocks. Witness arithmetic
uses the specified right-shift convention, but verifier constraints independently
enforce all intermediate operations. Witness construction is not verification.

A product's elements occupy 394,240 bytes. A GHASH component occupies
`256 + 49,408*N` field columns, including explicit input/output bindings.
Repeating one product over 512 rows alone would occupy 201,850,880 bytes; a full
chain scales with N. These are static element counts, not measured process or
prover memory. The product array is heap-backed and generated one product at a
time, avoiding a whole-chain stack temporary. Individual product construction
and enclosing layouts still need stack/allocation measurements. Backend parameters
are unchanged.

An enclosing GCM relation must additionally bind H to AES_K(0^128), construct the
correct padded AAD/ciphertext/length blocks, and bind GHASH output into the tag.
Nonce/counter construction, AES-CTR ciphertext, tag masking/truncation, KEM/KDF
correctness and the transfer statement remain separate. This module does not
establish complete AES-GCM encryption correctness or expose H publicly.

No tests or proofs were executed. Compile/static checks and source review are the
current validation boundary; cryptographic review and resource validation remain
required before activation.

This is a typed constraint component; trace serialization and a standalone AIR
wrapper have not been implemented here.
