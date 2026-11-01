# Canonical note byte stream

This generic component is compiled only with `native-privacy`. It validates
canonical opening shape. It is not selected by a proof profile or native suite.
No byte/word lookup, commitment hash, encryption, ownership, membership,
nullifier, eligibility or authorization relation is implemented here.

`NoteBytesAir<F>::new(asset,policy,count)` fixes nonzero public asset/policy and
one to four notes. It rejects fields with modulus at most65535. Every complete
512-byte note is processed, including its reserved tail; height is512,1024 or
2048. Three notes receive512 zero dummy rows. Each note has:

- Eight Boolean current-byte bits, one previous byte, one bit-count accumulator
  and one nonzero end inverse:11 main columns, without pads or lookup auxiliaries.
- Version1 and exactly the constructor's asset/policy bytes at offsets0..72.
- Zero bytes at241..511. The uint64 value bytes169..176 may all be zero.
- Nonzero owner73..136, account137..168, serial177..208 and blinding209..240.
  The accumulator counts bits only in those fields, resets after each end and
  begins at zero. Counts are at most512; inverses are zero off end rows.

The previous byte begins at zero and follows the current byte across rows.
Each reserved zero tail makes the next note begin with previous=0 naturally.
Dummy rows force all11 main cells to zero. The maximum constraint degree is3;
only previous-byte and accumulator columns are read on the next row.

Nine trusted preprocessing columns fix expected byte/public flag, nonzero-active
and end flags, dummy flag, note index, byte offset and byte/word-active flags.
The verifier must bind this exact constructor-specific schedule. A prover-chosen
schedule or a matching width alone cannot identify the intended statement.

`position(row)` returns immutable logical metadata only for actual note rows.
`byte_value` and `overlapping_word` expose fixed linear expressions with checked
row width. They accept no caller-provided identities. The overlap word is
previous+256*current, at byte offset minus one, and is inactive at every note
start. Future bus consumers must authenticate note/offset/type and exact fixed
fanout counts. Metadata and expression helpers do not implement that lookup.
Big-endian amount fields need explicit byte ordering at their consumers.

`trace` accepts exactly the configured number of fixed-size openings, validates
all canonical/public/nonzero fields before allocation, and returns `None` on
invalid input or allocation failure. Private traces and openings must stay local.
This component has no security approval or native acceptance path; tests remain deferred.
