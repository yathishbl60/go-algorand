# Coefficient-backed opening

This candidate uses the pinned stock hiding-PCS proof and verifier. It does not
change the proof encoding, FRI folding, or verification mathematics. It supports
the fixed compact Fp31/D10 parameters; the caller must select it explicitly.

The opening path:

1. Checks every request, matrix, point, hiding budget and scalar-buffer cap.
2. Atomically consumes the shared one-use leases for all requested commitments.
3. Evaluates all coefficient columns, including ten random codewords, sharing
   each point power across the columns of a coefficient row.
4. Binds claims through the stock `PcsProverTranscript` and samples its alpha.
5. Evaluates bounded column tiles, compresses them into one extension-field
   codeword, and reduces point quotients in stock commitment/matrix/point order.
6. Runs the stock generic FRI prover against actual coefficient-backed matrices.
7. Moves the authenticated opening fields into the stock proof type and places
   the ten random-codeword claims in the stock hiding wrapper.

Alpha offsets are independent for each LDE height. Reduced codewords are handed
to FRI in descending height order. FFT tiles arrive in natural order and are
scattered into bit-reversed order before quotient reduction. Only one point's
denominator and inverse vectors exist at a time; no wide main LDE or global
opening-point denominator cache is materialized.

All extension-field buffer checks count ten base-field coordinates per element.
The reduction cap conservatively covers the retained codeword at each height,
one compressed codeword, and one denominator/inverse pair at the largest height.
Tile-alpha weights have their own checked extension-field allocation. Caps do
not include vector metadata, FFT scratch, authentication paths, or total RSS.

Malformed or over-budget requests leave leases and the transcript unchanged.
After preflight succeeds, allocation or prover failures may consume leases;
retrying requires a fresh randomized commitment. Cloning prover data shares its
lease and does not authorize another disclosure. Repeated points count separately
against the hiding budget. Points in the committed LDE coset are rejected before
state changes because their quotient denominator would vanish.

Public preprocessing is explicitly unsupported by this candidate opening path.
The current universal relation uses periodic public columns without a separate
preprocessing commitment. Supporting a public preprocessing commitment requires
an explicit protocol-compatible path; it must not silently use dense storage.
