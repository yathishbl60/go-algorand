# Global Transaction Safety Mode Demo Proof

Date: 2026-05-09

## Executive Summary

This live demonstration validates end-to-end Global Transaction Safety Mode behavior on a running local Algorand private network.

Both demo workflows completed successfully:

1. Single transaction flow: simulate, evaluate, submit, and negative safety validation.
2. Atomic group flow: simulate, evaluate, submit, and round confirmation.

Final status: Pass.

## Environment

1. Repository: go-algorand
2. Node data dir: /tmp/gtsm-net-demo/Node
3. CLI runtime: goal with ALGORAND_DATA pointed to the local node

## Verified Outcomes

1. Script exit codes are green:
   - gtsm_demo_e2e.sh = 0
   - gtsm_demo_group_e2e.sh = 0
2. Deterministic outcome hashes were generated and recorded for both scenarios.
3. Group transaction was committed on-chain in round 273.
4. Negative safety path was rejected as expected by attestation/CLI validation gating.

## Evidence Links

1. Canonical summary: [SUMMARY.md](SUMMARY.md)
2. Single transaction log: [gtsm_demo_e2e.log](gtsm_demo_e2e.log)
3. Atomic group log: [gtsm_demo_group_e2e.log](gtsm_demo_group_e2e.log)
4. Captured run artifacts: [artifacts](artifacts)

## Key Evidence Extracts

1. Single flow outcome hash: 67184564196487a8e7927e3d5edda43ab7e19a3309e83eeeda109b7d806b8ebf
2. Group flow outcome hash: 4ebf1bd9ff0759927cf413d07911fc3f502d374e4efd48e25bc65ac811dbf01e
3. Group commit proof: committed in round 273
4. Negative-path proof: negative test failed as expected

## Repro Command

Use this to regenerate a fresh proof bundle:

```bash
chmod +x scripts/gtsm_demo_e2e.sh scripts/gtsm_demo_group_e2e.sh
export PATH="$HOME/go/bin:$PATH"
export ALGORAND_DATA=/tmp/gtsm-net-demo/Node
./scripts/gtsm_demo_e2e.sh
./scripts/gtsm_demo_group_e2e.sh
```

## Conclusion

The demo confirms that Global Transaction Safety Mode is functioning end-to-end for:

1. Deterministic simulation and outcome hashing.
2. Policy evaluation and decisioning.
3. Successful transaction submission paths.
4. Safety enforcement on invalid attestation-related submission paths.