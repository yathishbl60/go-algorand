# Compact private Keccak round component

This generic component is behind the existing native-privacy feature. It is not
a selected profile, native verifier, approved artifact, or complete hash proof.

`CompactKeccakAir::<F>::new(permutations)` accepts 1–1365 independent Keccak-f
permutations and a prime field with order greater than 65535. Its 1920 main
columns contain 1600 Boolean post-Theta bits and 320 Boolean original column
parities. The constructor fixes 24 rounds per permutation, actual pinned round
constants, continuation flags, and row identities. Rows are the next power of
two of 24 times the count, with a minimum of 512 and maximum of 32768.

Ungated parity constraints equate each post-Theta column's XOR5 to the XOR3 of
original column parity and the two Theta correction parities. This ensures the
reconstructed input has the declared original parity. Original bits are XOR3
expressions; Rho/Pi, Chi and Iota output expressions use trusted round constants.
Only rounds 0–22 constrain the next row's reconstructed input to this output.
No transition joins independent permutations. Padding rows force every main
column and round-constant bit to zero. Maximum symbolic constraint degree is 5.

`trace(&[[u64;25]])` uses the existing native serial-Keccak stage functions and
requires exactly the constructor's input count. `position(row)` exposes fixed
permutation and round identities, returning `None` for padding. The checked
`input_word(row, word)` and `output_word(row, program, word)` helpers return
little-endian 16-bit boundary expressions for word indices 0–99. The output
helper's program must be the corresponding row of this constructor's trusted
preprocessing, never witness-selected constants or row metadata.

Private lookup buses, consumer multiplicities, sponge absorption, SHA3 padding,
message binding, final digest binding, private masks and lookup auxiliaries are
absent. These obligations must be supplied by a containing proof before this
can establish a hash relation. Future combined bus fractions may raise degree
to 8; they are not implemented here. No automated tests or selected consensus
parameters are added by this component.
