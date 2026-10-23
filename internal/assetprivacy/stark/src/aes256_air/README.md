# Candidate AES-256 block constraints

`Aes256<T>` contains a complete AES-256 key schedule and block-cipher constraint
component. Reference: [NIST FIPS 197-upd1](https://doi.org/10.6028/NIST.FIPS.197-upd1),
updated May 9, 2023, sections 5.1 and 5.2. Bytes use column-major AES state ordering;
bits within each byte are least-significant first.

`ExpandedKey` constrains 60 four-byte words. The first eight words are the private
256-bit key. All subsequent words are bound to earlier words and the required
SubWord/RotWord cases, including the additional AES-256 substitution every fourth
word within an eight-word group. Seven round constants 01 through 40 are included.
All 52 key-schedule S-boxes invoke the existing algebraic S-box evaluator, with
inputs explicitly bound to the selected prior-word bytes.

`eval_cipher` constrains initial AddRoundKey, then 14 SubBytes and ShiftRows
stages. The first 13 rounds include MixColumns; the final round omits it. Every
round key is taken from the same constrained schedule, and every SubBytes input
is bound to the previous state. Ciphertext bits are `state[14]`.

`MixColumn` uses the standard equivalent construction for input bytes a, b, c, d:
form their XOR total, then XOR each byte with that total and with xtime of its
XOR with the next byte. Boolean intermediate bytes break XOR chains into
degree-two constraints. No high-degree polynomial expansion or trusted host
ciphertext equality is substituted for cipher constraints.

`Aes256<Value>::witness(key, plaintext)` constructs all private intermediate
columns. Witness construction does not validate a proof. An enclosing AIR must
allocate and populate columns matching these typed components, call
`eval_cipher`, bind the first eight key words and plaintext to the intended
secret/input, and bind final ciphertext bits to its claimed result. Helpers do
not introduce public values, trace serialization, or row transitions.

The ungated component has 78,388 field columns: 14,452 for the expanded key and
its 52 S-boxes, 128 plaintext bits, 1,920 state bits, 53,984 round S-box columns and
7,904 MixColumns columns. At eight bytes per Goldilocks element, one witness
row occupies 627,104 bytes of elements. Repeating it over 512 rows would consume
321,077,248 bytes before hiding, commitments or prover temporaries. This is
static element arithmetic; no runtime benchmark or proof was executed.
Multi-block constructions can share the constrained key schedule using modular
evaluators rather than allocating a fresh schedule for each block.

Every ungated constraint has degree at most two. Current backend parameters
remain unchanged. Adding gates or new composition must re-evaluate degree,
quotient openings and the hiding budget.

GCM counter/nonce construction, GF(2^128) GHASH, associated-data and length
binding, tag equality, ML-KEM/KDF correctness, disclosure requirements and
transfer-statement integration remain separate obligations. This block
component does not establish complete note-encryption correctness.

Enabled Clippy and default-disabled compilation pass. Functions have at most
20 physical lines and files at most 100. No tests or proofs were executed;
independent cryptographic review and runtime validation remain required.
