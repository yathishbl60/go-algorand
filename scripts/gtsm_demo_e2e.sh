#!/usr/bin/env bash

set -euo pipefail

AMOUNT=100000
FEE=1000
SAFETY_MODE=1
POLICY_FILE=""
NOTE="gtsm-demo-note"
WORKDIR="/tmp/gtsm-demo-$(date +%s)"

usage() {
  cat <<'EOF'
Usage: scripts/gtsm_demo_e2e.sh [options]

Options:
  --amount <microalgos>     Payment amount (default: 100000)
  --fee <microalgos>        Tx fee (default: 1000)
  --safety-mode <0|1|2>     Attestation mode for positive submit (default: 1)
  --policy <path>           Policy JSON path (default: auto-generated permissive policy)
  --note <text>             Deterministic note value (default: gtsm-demo-note)
  --workdir <path>          Output directory for demo artifacts
  -h, --help                Show this help
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --amount)
      AMOUNT="$2"
      shift 2
      ;;
    --fee)
      FEE="$2"
      shift 2
      ;;
    --safety-mode)
      SAFETY_MODE="$2"
      shift 2
      ;;
    --policy)
      POLICY_FILE="$2"
      shift 2
      ;;
    --note)
      NOTE="$2"
      shift 2
      ;;
    --workdir)
      WORKDIR="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage
      exit 1
      ;;
  esac
done

if ! command -v goal >/dev/null 2>&1; then
  echo "goal command not found in PATH" >&2
  exit 1
fi

mkdir -p "$WORKDIR"
echo "[gtsm-demo] workdir: $WORKDIR"

echo "[gtsm-demo] validating node connectivity"
goal node status >/dev/null

ACCOUNT_LINES=$(goal account list | awk '/microAlgos/{print $(NF-2), $(NF-1)}' | sort -k2nr)
FROM=$(echo "$ACCOUNT_LINES" | awk 'NR==1{print $1}')
TO=$(echo "$ACCOUNT_LINES" | awk 'NR==2{print $1}')

if [[ -z "${FROM:-}" || -z "${TO:-}" ]]; then
  echo "Unable to determine distinct source/receiver accounts from funded account list" >&2
  exit 1
fi

if [[ "$FROM" == "$TO" ]]; then
  echo "Source and destination resolved to the same account; need at least two funded accounts" >&2
  exit 1
fi

LAST_ROUND=$(goal node status | awk -F': ' '/Last committed block/{print $2}')
if [[ -z "${LAST_ROUND:-}" ]]; then
  echo "Unable to parse last committed block" >&2
  exit 1
fi

FIRST_VALID=$LAST_ROUND
LAST_VALID=$((FIRST_VALID + 500))

UTXN="$WORKDIR/demo.utxn"
STXN="$WORKDIR/demo.stxn"
SIM_OUT="$WORKDIR/simulate.json"
EVAL_OUT="$WORKDIR/evaluate.json"
DEMO_POLICY="$WORKDIR/policy.json"

if [[ -z "$POLICY_FILE" ]]; then
  cat > "$DEMO_POLICY" <<EOF
{
  "enforcement-mode": "soft",
  "allow-rekey": false,
  "blocked-receivers": [],
  "allow-app-global-deletes": false,
  "allow-bypass-if-unavailable": true,
  "required-risk-signals-absent": [
    "rekeyChange",
    "closeRemainderToUsed",
    "assetCloseToUsed",
    "appGlobalStateDelete"
  ]
}
EOF
  POLICY_FILE="$DEMO_POLICY"
fi

if [[ ! -f "$POLICY_FILE" ]]; then
  echo "Policy file not found: $POLICY_FILE" >&2
  exit 1
fi

echo "[gtsm-demo] building unsigned transaction"
goal clerk send \
  --from "$FROM" \
  --to "$TO" \
  --amount "$AMOUNT" \
  --fee "$FEE" \
  --note "$NOTE" \
  --firstvalid "$FIRST_VALID" \
  --lastvalid "$LAST_VALID" \
  -o "$UTXN"

echo "[gtsm-demo] signing transaction"
goal clerk sign -i "$UTXN" -o "$STXN"

echo "[gtsm-demo] simulate safety"
goal clerk safety simulate --txfile "$STXN" --include-simulation > "$SIM_OUT"
cat "$SIM_OUT"

echo "[gtsm-demo] evaluate safety policy"
goal clerk safety evaluate --txfile "$STXN" --policy "$POLICY_FILE" > "$EVAL_OUT"
cat "$EVAL_OUT"

extract_hash() {
  local file="$1"
  if command -v jq >/dev/null 2>&1; then
    jq -r '."outcome-hash"' "$file"
    return
  fi

  python3 - <<'PY' "$file"
import json
import sys
path = sys.argv[1]
with open(path, 'r', encoding='utf-8') as f:
    data = json.load(f)
value = data.get('outcome-hash', '')
print(value if isinstance(value, str) else '')
PY
}

HASH=$(extract_hash "$SIM_OUT")
if [[ -z "$HASH" || ${#HASH} -ne 64 ]]; then
  echo "Invalid or missing outcome-hash in $SIM_OUT" >&2
  exit 1
fi

echo "[gtsm-demo] extracted outcome-hash: $HASH"

echo "[gtsm-demo] positive baseline submit (rawsend simulated signed transaction)"
goal clerk rawsend -f "$STXN"

BAD_HASH=$(printf '0%.0s' {1..64})

echo "[gtsm-demo] negative attested submit (expected failure)"
set +e
NEG_OUTPUT=$(goal clerk send \
  --from "$FROM" \
  --to "$TO" \
  --amount "$AMOUNT" \
  --fee "$FEE" \
  --note "$NOTE" \
  --firstvalid "$FIRST_VALID" \
  --lastvalid "$LAST_VALID" \
  --safety-mode 1 \
  --expected-outcome-hash "$BAD_HASH" 2>&1)
NEG_RC=$?
set -e

if [[ $NEG_RC -eq 0 ]]; then
  echo "Negative test unexpectedly succeeded" >&2
  exit 1
fi

echo "$NEG_OUTPUT"
echo "[gtsm-demo] negative test failed as expected"

cat <<EOF

[gtsm-demo] complete
Artifacts:
  simulate: $SIM_OUT
  evaluate: $EVAL_OUT
  unsigned: $UTXN
  signed:   $STXN

EOF
