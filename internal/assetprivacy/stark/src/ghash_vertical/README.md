# Private vertical GHASH product

`GhashProductAir` constrains one SP800-38D field multiplication using a fixed
512-row schedule. No input or output is a public value. `wires(row)` exposes
references to the private operands and result for an enclosing relation to bind;
a valid standalone multiplication does not authorize encryption or a transfer.

The main trace has exactly 769 Boolean columns, with transmitted-MSB-first bits:

| Columns | Contents |
| --- | --- |
| 0–127 | Constant left operand |
| 128–255 | Constant right operand |
| 256–383 | Current shifted right operand V |
| 384–511 | Selected V bits |
| 512–639 | Current XOR accumulator Z |
| 640–767 | Constant declared product |
| 768 | Selected left-input bit |

The preprocessed trace has 128 one-hot selector columns: row i selects input bit
i for rows 0 through 127; rows 128 through 511 contain only zero selectors.
The first row fixes V to the right operand and Z to zero. Each active transition
XORs the selected V into Z, shifts V right, and conditionally XORs the top-byte
`0xe1` reduction constant. At selector 127 the declared product equals Z XOR
selected V. Padding freezes V and Z, with zero input and selected bits.
Operands and product are bound across every transition; all main bits are ranged.

The caller must set up this exact fixed schedule with pinned Plonky3
`setup_preprocessed(config, air, 9)` and retain the resulting trusted verifier
key. Use the matching `prove_with_preprocessed` and `verify_with_preprocessed`
APIs and the 512-row trace; do not accept a submitter-selected preprocessing key
or shape. For the current hiding PCS, setup converts the base log-height 9 to
extended log-height 10. No native verifier or proof encoding is installed here.

The maximum algebraic degree is four with boundary/transition and preprocessed
selectors counted: the V-feedback XOR has degree two, active selection adds
one, and transition gating adds one. The final product gate has degree three;
Boolean checks and selected-bit multiplication have degree two. The enclosing
relation must derive its actual quotient/opening and hiding budgets, including
preprocessing. Both PCS commitment schemes must remain hiding and masks fresh.

Raw main values occupy 3,149,824 bytes at eight bytes per field element; the
preprocessed values occupy 524,288 bytes. These are static matrix sizes only,
excluding masking, commitments, proving workspace and enclosing relations.
There are no private cross-AIR messages or disclosed LogUp terminals. This does
not establish complete zero knowledge or practical proving resource limits.

Witness generation and constraints implement the same 128-step multiplication.
No tests, runtime constraint evaluation, proof generation or verification were
executed. This component remains behind the disabled native-privacy feature.
