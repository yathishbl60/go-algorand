#!/usr/bin/env bash

set -euo pipefail

AMOUNT1=100000
AMOUNT2=200000
FEE=1000
POLICY_FILE=""
WORKDIR="/tmp/gtsm-group-demo-$(date +%s)"

usage() {
  cat <<'EOF'
Usage: scripts/gtsm_demo_group_e2e.sh [options]

Options:
  --amount1 <microalgos>   First transfer amount (default: 100000)
  --amount2 <microalgos>   Second transfer amount (default: 200000)
  --fee <microalgos>       Fee per txn (default: 1000)
  --policy <path>          Policy JSON path (default: auto-generated permissive policy)
  --workdir <path>         Output directory for artifacts
  -h, --help               Show this help
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --amount1)
      AMOUNT1="$2"
      shift 2
      ;;
    --amount2)
      AMOUNT2="$2"
      shift 2
      ;;
    --fee)
      FEE="$2"
      shift 2
      ;;
    --policy)
      POLICY_FILE="$2"
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
echo "[gtsm-group-demo] workdir: $WORKDIR"

goal node status >/dev/null

ACCOUNT_LINES=$(goal account list | awk '/microAlgos/{print $(NF-2), $(NF-1)}' | sort -k2nr)
FROM=$(echo "$ACCOUNT_LINES" | awk 'NR==1{print $1}')
TO1=$(echo "$ACCOUNT_LINES" | awk 'NR==2{print $1}')
TO2=$(echo "$ACCOUNT_LINES" | awk 'NR==3{print $1}')

if [[ -z "${FROM:-}" || -z "${TO1:-}" || -z "${TO2:-}" ]]; then
  echo "Unable to determine three distinct funded accounts from account list" >&2
  exit 1
fi

if [[ "$FROM" == "$TO1" || "$FROM" == "$TO2" || "$TO1" == "$TO2" ]]; then
  echo "Account selection resolved to duplicates; need three distinct funded accounts" >&2
  exit 1
fi

LAST_ROUND=$(goal node status | awk -F': ' '/Last committed block/{print $2}')
if [[ -z "${LAST_ROUND:-}" ]]; then
  echo "Unable to parse last committed block" >&2
  exit 1
fi

FIRST_VALID=$LAST_ROUND
LAST_VALID=$((FIRST_VALID + 500))

UTXN1="$WORKDIR/tx1.utxn"
UTXN2="$WORKDIR/tx2.utxn"
COMBINED="$WORKDIR/combined.utxn"
GROUPED="$WORKDIR/grouped.utxn"
SIGNED="$WORKDIR/grouped.stxn"
SIM_OUT="$WORKDIR/group_simulate.json"
EVAL_OUT="$WORKDIR/group_evaluate.json"
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

echo "[gtsm-group-demo] building unsigned transactions"
goal clerk send \
  --from "$FROM" \
  --to "$TO1" \
  --amount "$AMOUNT1" \
  --fee "$FEE" \
  --firstvalid "$FIRST_VALID" \
  --lastvalid "$LAST_VALID" \
  -o "$UTXN1"

goal clerk send \
  --from "$FROM" \
  --to "$TO2" \
  --amount "$AMOUNT2" \
  --fee "$FEE" \
  --firstvalid "$FIRST_VALID" \
  --lastvalid "$LAST_VALID" \
  -o "$UTXN2"

cat "$UTXN1" "$UTXN2" > "$COMBINED"

echo "[gtsm-group-demo] grouping transactions"
goal clerk group -i "$COMBINED" -o "$GROUPED"

echo "[gtsm-group-demo] signing grouped transactions"
goal clerk sign -i "$GROUPED" -o "$SIGNED"

echo "[gtsm-group-demo] simulate safety on grouped transaction"
goal clerk safety simulate --txfile "$SIGNED" --include-simulation > "$SIM_OUT"
cat "$SIM_OUT"

echo "[gtsm-group-demo] evaluate policy on grouped transaction"
goal clerk safety evaluate --txfile "$SIGNED" --policy "$POLICY_FILE" > "$EVAL_OUT"
cat "$EVAL_OUT"

echo "[gtsm-group-demo] submitting grouped transaction"
goal clerk rawsend -f "$SIGNED"

cat <<EOF

[gtsm-group-demo] complete
Artifacts:
  tx1 unsigned:  $UTXN1
  tx2 unsigned:  $UTXN2
  combined:      $COMBINED
  grouped:       $GROUPED
  signed group:  $SIGNED
  simulate json: $SIM_OUT
  evaluate json: $EVAL_OUT

EOF
