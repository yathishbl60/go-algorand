# GTSM Demo Cases and How-To

This guide provides practical end-to-end demo scenarios for Global Transaction Safety Mode (GTSM) on a local or dev network.

## Prerequisites

1. `algod` is running and reachable by `goal`.
2. You have at least one funded account.
3. GTSM code from this branch is built in your running binaries.
4. Optional but recommended: `jq` for easy JSON extraction.

Quick health checks:

```bash
goal node status
goal account list
```

## Fast Path: Use the Demo Script

Run the helper script for an automated positive and negative flow:

```bash
scripts/gtsm_demo_e2e.sh
```

Optional flags:

```bash
scripts/gtsm_demo_e2e.sh --amount 100000 --fee 1000 --safety-mode 1
```

Note:

1. By default, the script auto-generates a permissive policy in its work directory for first-run success.
2. Use `--policy <path>` to force a stricter policy (for example, `docs/examples/gtsm-policy.json`).

What the script does:

1. Creates and signs a payment transaction file.
2. Runs `goal clerk safety simulate` and captures `outcome-hash`.
3. Runs `goal clerk safety evaluate` against the policy example.
4. Submits the simulated signed transaction (`rawsend`) as a positive baseline.
5. Runs a negative attestation test with an intentionally wrong hash and expects failure.

## Fast Path: Atomic Group Demo Script

Use this to demonstrate grouped transactions end-to-end:

```bash
scripts/gtsm_demo_group_e2e.sh
```

Optional flags:

```bash
scripts/gtsm_demo_group_e2e.sh --amount1 100000 --amount2 200000 --fee 1000
```

Note:

1. By default, this script also auto-generates a permissive policy to avoid allow-list failures on newly created recipient accounts.
2. Use `--policy <path>` to exercise stricter policy behavior.

What this script does:

1. Creates two unsigned payment transactions.
2. Concatenates and groups them with `goal clerk group`.
3. Signs the grouped payload.
4. Runs `goal clerk safety simulate` and `goal clerk safety evaluate` on the signed group.
5. Submits the signed group via `goal clerk rawsend`.

## Manual Demo Cases

## Case 1: Baseline Safety Simulation

Create and sign a deterministic transaction payload:

```bash
FROM=$(goal account list | awk 'NR==1{print $3}')
TO=$(goal account new | awk '{print $NF}')
LVL=$(goal node status | awk -F': ' '/Last committed block/{print $2}')
FV=$((LVL+5))
LV=$((FV+500))

goal clerk send \
  --from "$FROM" \
  --to "$TO" \
  --amount 100000 \
  --fee 1000 \
  --firstvalid "$FV" \
  --lastvalid "$LV" \
  -o demo.utxn

goal clerk sign -i demo.utxn -o demo.stxn
goal clerk safety simulate --txfile demo.stxn --include-simulation | tee simulate.json
```

Expected checks:

1. `verification-status` is `verified`.
2. `outcome-hash` is a 64-hex string.
3. `risk-signals` is deterministic for repeated runs at the same context.

## Case 2: Policy Evaluation

```bash
goal clerk safety evaluate \
  --txfile demo.stxn \
  --policy docs/examples/gtsm-policy.json | tee evaluate.json
```

Expected checks:

1. Response contains `allowed`, `decision`, and `violations`.
2. `verification-status` is `verified` in normal conditions.

## Case 3: Positive Attested Submission

Extract outcome hash:

```bash
HASH=$(jq -r '.outcome-hash' simulate.json)
```

If `jq` is unavailable:

```bash
HASH=$(python3 - <<'PY'
import json
with open('simulate.json', 'r', encoding='utf-8') as f:
    print(json.load(f).get('outcome-hash', ''))
PY
)
```

Submit with attestation:

```bash
goal clerk send \
  --from "$FROM" \
  --to "$TO" \
  --amount 100000 \
  --fee 1000 \
  --firstvalid "$FV" \
  --lastvalid "$LV" \
  --safety-mode 1 \
  --expected-outcome-hash "$HASH"
```

Expected result:

1. Transaction accepted and submitted.

## Case 4: Negative Attestation Mismatch

```bash
BAD=$(printf '0%.0s' {1..64})

goal clerk send \
  --from "$FROM" \
  --to "$TO" \
  --amount 100000 \
  --fee 1000 \
  --firstvalid "$FV" \
  --lastvalid "$LV" \
  --safety-mode 1 \
  --expected-outcome-hash "$BAD"
```

Expected result:

1. Submission fails due to safety outcome hash mismatch.

## Case 5: Strict Mode (Hash + No Risk Signals)

```bash
goal clerk send \
  --from "$FROM" \
  --to "$TO" \
  --amount 100000 \
  --fee 1000 \
  --firstvalid "$FV" \
  --lastvalid "$LV" \
  --safety-mode 2 \
  --expected-outcome-hash "$HASH"
```

Expected result:

1. Accepted only when both hash matches and risk-signal constraints are satisfied.

## Demo Talking Points

Use these points during a walkthrough:

1. GTSM does not require attestation for every transaction; it is policy-driven.
2. Safety simulation and evaluation are pre-sign controls.
3. `safh` and `safm` enforce execution-time guarantees when enabled.
4. A wrong hash is rejected deterministically.

## Troubleshooting

1. If `goal` cannot connect, check your data dir and node status.
2. If evaluation is `unverified`, inspect API/node availability and fallback policy settings.
3. If submission fails on validity rounds, regenerate transaction files with fresh `firstvalid/lastvalid`.
4. If full repository e2e tests fail in this environment, ensure external e2e tool dependencies are installed before running `make testall`.

## Cleanup

```bash
rm -f demo.utxn demo.stxn simulate.json evaluate.json
```

If you used script defaults, script artifacts are written under `/tmp/gtsm-demo-*` and `/tmp/gtsm-group-demo-*`.
