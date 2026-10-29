# Copyright (C) 2026 Algorand Foundation Ltd.
# SPDX-License-Identifier: AGPL-3.0-or-later

"""Keep appended privacy witnesses after historical flattened array fields.

The pinned msgp generator flattens embedded structs in declaration order. Only
array decoding is normalized here; canonical map encoders/decoders are untouched.
Fail on a changed generator shape rather than silently changing historical slots.
"""

import re
import sys
from pathlib import Path

COUNTS = {"SignedTxn": 7, "SignedTxnWithAD": 15, "SignedTxnInBlock": 17}
FIELD = re.compile(r"(?m)^\t\tif \w+ > 0 \{\n[\s\S]*?^\t\t\}\n")


def require(condition, message):
    if not condition:
        raise ValueError(message)


def array_branch(source, name):
    signature = f"func (z *{name}) UnmarshalMsgWithState("
    require(source.count(signature) == 1, f"missing/duplicate {name} decoder")
    function = source.index(signature)
    start = source.index("\tif _, ok := err.(msgp.TypeError); ok {\n", function)
    end = source.index("\n\t} else {", start)
    require("\nfunc " not in source[function:end], f"missing {name} array branch")
    return start, end


def array_fields(branch, count):
    fields = [m for m in FIELD.finditer(branch) if '"struct-from-array",' in m[0]]
    require(len(fields) == count, "changed generated array field count")
    first, last = fields[0].start(), fields[-1].end()
    require("".join(m[0] for m in fields) == branch[first:last], "noncontiguous array fields")
    privacy = [m for m in fields if "NativeAssetPrivacyProof" in m[0]]
    require(len(privacy) == 1, "missing/duplicate privacy array field")
    return fields, privacy[0], first, last


def normalize(source, name, count):
    start, end = array_branch(source, name)
    branch = source[start:end]
    fields, privacy, first, last = array_fields(branch, count)
    ordered = [m[0] for m in fields if m is not privacy] + [privacy[0]]
    replacement = branch[:first] + "".join(ordered) + branch[last:]
    return source[:start] + replacement + source[end:]


def main():
    require(len(sys.argv) == 2, "usage: native_privacy_array_codec.py generated-file")
    path = Path(sys.argv[1])
    if path.parts[-3:] != ("data", "transactions", "msgp_gen.go"):
        return
    source = path.read_text()
    for name, count in COUNTS.items():
        source = normalize(source, name, count)
    path.write_text(source)


if __name__ == "__main__":
    main()
