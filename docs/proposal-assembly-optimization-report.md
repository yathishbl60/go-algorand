# Proposal Assembly Optimization Report

Date: 2026-05-14

## Goal

This optimization pass targeted the steady-state block proposal path and the
transaction-pool recompute path.

The main goals were:

- reduce proposal latency in realistic steady-state operation
- reduce allocation count and total allocated bytes
- keep changes localized, measurable, and production-safe

## Benchmarks Used

The work was driven by three benchmarks:

- `go test -run '^$' -bench '^BenchmarkAssembleBlockSteadyState$' -benchmem -benchtime=1x -count=3 ./node`
- `go test -run '^$' -bench '^BenchmarkTransactionPoolRecompute$' -benchmem -benchtime=1x -count=3 ./data/pools`
- `go test -run '^$' -bench '^BenchmarkTxnRoots/(MerkleCommit|SHA256MerkleCommit)$' -benchmem -benchtime=1x -count=3 ./data/bookkeeping`

The steady-state node benchmark was the primary end-to-end signal. The Merkle
benchmark was used to confirm hashing and commitment changes in isolation.

## Benchmark Harness Changes

File: `node/assemble_test.go`

- Fixed `BenchmarkAssembleBlock` to use `NextRound()` instead of a stale-round
  setup.
- Added unique in-memory ledger names with `crypto.RandUint64()` so repeated
  benchmark runs do not collide.
- Added `BenchmarkAssembleBlockSteadyState` to measure the real between-round
  path:
  - `AddBlock`
  - `txPool.OnNewBlock(...)`
  - refill outside the timed section
  - `txPool.AssembleBlock(...)`

These changes made the benchmark stable enough to guide production work.

## Kept Code Changes

### 1. Transaction Pool Hot Locks

File: `data/pools/transactionPool.go`

- Switched the hot internal pool locks from `go-deadlock` mutexes to standard
  `sync.Mutex` and `sync.RWMutex`.
- Kept deadlock instrumentation elsewhere in the repo unchanged.

Why:

- allocation profiles showed `go-deadlock.lockEnabled` and
  `go-deadlock.callers` as major object-allocation sources in the proposal path

### 2. Transaction Pool Recompute Buffer Sizing

File: `data/pools/transactionPool.go`

- Pre-sized replay staging buffers in `recomputeBlockEvaluator`:
  - `rememberedTxGroups`
  - `rememberedTxids`

Why:

- recompute replays the whole pool and was repeatedly growing these structures

### 3. Evaluator Payset Capacity Reservation

Files:

- `data/pools/transactionPool.go`
- `ledger/eval/eval.go`

- Added `ReservePaysetCapacity(capacity int)` to the block-evaluator interface.
- Called it from tx-pool recompute after `StartEvaluator(...)`.

Why:

- `StartEvaluator` intentionally caps `PaysetHint` to a single-block estimate
- tx-pool recompute keeps one evaluator alive across multiple simulated blocks
- profiles showed `slices.Grow` on the evaluator payset as a large remaining
  space hotspot

### 4. TransactionGroup Payset Reuse

File: `ledger/eval/eval.go`

- Changed `BlockEvaluator.TransactionGroup` to write `SignedTxnInBlock` outputs
  directly into reserved space in `eval.block.Payset`.
- Removed the temporary `txibs` allocation plus the final append/copy into the
  block payset.
- Added rollback-to-original-length behavior on error.

Why:

- profiles showed both `txibs := make(...)` and the final payset append as hot
  allocation sites

### 5. NewAppEvalParams Fast Path

Files:

- `data/transactions/logic/eval.go`
- `data/transactions/logic/blackbox_test.go`

- Added a fast path in `NewAppEvalParams` so it only copies the txgroup when
  `ApplyData` is actually non-empty.
- Avoided allocating app-only helpers for groups with no app calls:
  - `appAddrCache`
  - runtime eval constants
  - pooled app budget state

Why:

- `TransactionGroup` was building full app-eval state even for ordinary
  payment-heavy groups

Regression coverage added:

- `TestNewAppEvalParamsClearsApplyData`

### 6. List Free-Node Batching

File: `util/list.go`

- Changed `AllocateFreeNodes` to allocate one contiguous slice of list nodes and
  link them into the free list, instead of calling `new(ListNode[T])` once per
  node.

Why:

- `util.(*List).AllocateFreeNodes` was a top alloc-object site

### 7. Hash Representation Cleanup

Files:

- `crypto/util.go`
- `crypto/hashes.go`

- Changed `HashRep` to build the prefixed representation in one allocation.
- Added `GenericHashable` so selected hot-path objects can hash directly into a
  `hash.Hash` without first materializing a `HashRep` buffer.

Why:

- `HashRep` and generic hash-object wrapping remained visible in allocation
  profiles after the first round of Merkle work

### 8. Merkle Internal Node Fast Path

Files:

- `crypto/merklearray/layer.go`
- `crypto/merklearray/partial.go`

- Added `hashPairTo(...)` and moved internal-node hashing to a stack buffer.
- Stopped allocating a fresh representation buffer for every internal Merkle
  node.

Why:

- internal-node hashing remained a major object-allocation source

### 9. Direct Merkle Leaf Hashing

Files:

- `crypto/merklearray/array.go`
- `crypto/merklearray/merkle.go`
- `data/bookkeeping/txn_merkle.go`

- Added optional Merkle-array fast-path interfaces:
  - `HashableArray`
  - `HashableArrayInto`
- Implemented direct leaf hashing for `txnMerkleArray`.
- Added `hashTxnMerkleLeaf` and `hashTxnMerkleLeafTo` so leaf hashing can write
  directly into caller-provided digest storage.

Why:

- `txnMerkleArray.Marshal` was still the dominant alloc-space site after early
  hashing improvements

### 10. Contiguous Merkle Layer Storage

Files:

- `crypto/merklearray/layer.go`
- `crypto/merklearray/merkle.go`

- Added `makeLayer(...)` so Merkle layers are backed by one contiguous digest
  buffer rather than one separately allocated digest slice per entry.
- Updated leaf and internal-node hashing to write directly into those digest
  slices.

Why:

- digest backing storage itself had become a large remaining allocator

### 11. Vector Commitment Fast Path

File: `crypto/merklearray/vectorCommitmentArray.go`

- Propagated direct hashing through the vector-commitment wrapper by adding a
  `HashInto(...)` implementation.
- This removed the fallback to `Marshal(...)` for the SHA-256/SHA-512 vector
  commitment paths.

Why:

- after improving the regular Merkle path, the SHA-256 vector-commitment path
  was still paying the old wrapper allocation cost

## Results

### End-to-End Steady-State Proposal Benchmark

Benchmark:

- `BenchmarkAssembleBlockSteadyState`

Original baseline:

- `83.24M ns/op`
- `236.7M B/op`
- `537,422 allocs/op`

Final validation range:

- `59.9M - 62.8M ns/op`
- `73.9M - 74.0M B/op`
- `178,187 - 178,277 allocs/op`

Net improvement:

- about `24% - 28%` faster
- about `69%` lower allocated bytes
- about `67%` fewer allocations

### Transaction Pool Recompute Benchmark

Benchmark:

- `BenchmarkTransactionPoolRecompute`

Original baseline:

- `540.95M - 573.20M ns/op`
- `1,494,911,600 - 1,495,167,920 B/op`
- `2,503,868 - 2,503,909 allocs/op`

Final validation range:

- `480.0M - 505.1M ns/op`
- about `590.6M B/op`
- `1,084,865 - 1,084,948 allocs/op`

Net improvement:

- about `7% - 16%` faster
- about `60%` lower allocated bytes
- about `57%` fewer allocations

### Merkle Micro-Benchmark Highlights

Benchmark:

- `BenchmarkTxnRoots/MerkleCommit`
- `BenchmarkTxnRoots/SHA256MerkleCommit`

Final observed ranges after the Merkle work:

- `MerkleCommit`: `2.96 - 2.99 ms/op`, `10.4 - 11.4 MB/op`,
  `61,696 - 61,809 allocs/op`
- `SHA256MerkleCommit`: `2.59 - 2.69 ms/op`, about `10.9 MB/op`, about
  `65,781 allocs/op`

These were the strongest micro-level signals that the direct-hash and
contiguous-layer changes were doing real work.

## Validation Performed

Focused tests run on the final kept change set:

- `go test ./data/transactions/logic -run '^(TestNewAppEvalParams|TestNewAppEvalParamsClearsApplyData)$'`
- `go test ./data/bookkeeping -run '^(TestTxnMerkleElemHash|TestTxnMerkle|TestBlock_TxnMerkleTreeSHA256)$'`
- `go test ./crypto/merklearray -run '^(TestBuildPrefersHashInto|TestBuildVectorCommitmentPrefersHashInto|TestVectorCommitmentHashIntoMatchesMarshal)$'`
- `go test ./util -run 'TestList|TestFreeList|TestMoveToFront|TestListWithPointers'`
- `go test ./ledger/eval -run '^(TestPrivateTransactionGroup|TestTestTransactionGroup|TestEvalAppStateCountsWithTxnGroup|TestEvalAppAllocStateWithTxnGroup|TestReservePaysetCapacity|TestTransactionGroupRollbackResetsPayset)$'`

All of the above passed.

Broader package validation also passed:

- `go test ./crypto/merklearray ./data/bookkeeping ./data/pools ./data/transactions/logic ./ledger/eval ./util -count=1`
- `go test ./node -run '^$' -bench '^BenchmarkAssembleBlockSteadyState$' -benchmem -benchtime=1x -count=1`

Latest steady-state benchmark confirmation from that final run:

- `59,961,167 ns/op`
- `74,060,648 B/op`
- `178,268 allocs/op`

## Experiment Tried And Reverted

The session also tested switching the outer ledger tracker locks from
`go-deadlock` to `sync.RWMutex`.

That experiment was benchmarked and then reverted because:

- allocation reduction was small
- timing impact was mixed
- the debugging tradeoff was larger than the measured benefit

It is not part of the final kept change set.

## Remaining Hotspots After This Pass

After the final kept changes, the top remaining steady-state allocation sites
were no longer small local bugs. They were broader infrastructure costs such
as:

- one-time payset growth and reservation behavior
- ledger LRU cache initialization
- verified transaction cache setup
- remaining `go-deadlock` overhead outside the already-optimized tx-pool slice
- timer churn inside `go-deadlock`

Those are still valid optimization targets, but they are no longer in the same
category as the low-risk, localized wins implemented in this pass.