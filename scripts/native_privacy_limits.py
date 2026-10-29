# Copyright (C) 2026 Algorand Foundation Ltd.
# SPDX-License-Identifier: AGPL-3.0-or-later

"""Check authored native privacy file/function physical line limits; no tests run."""

import ast
import re
from pathlib import Path

TOKEN = re.compile(r'//[^\n]*|/\*[\s\S]*?\*/|"(?:\\.|[^"\\])*"|\x27(?:\\.|[^\x27\\])\x27|`[^`]*`')
FUNCTION = re.compile(r'(?m)^[ \t]*(?:pub(?:\([^)]*\))?[ \t]+)?(?:const[ \t]+)?(?:unsafe[ \t]+)?(?:extern[ \t]+"[^"\n]+"[ \t]+)?fn[ \t]+\w+|^func[ \t]+')


def sources():
    roots = (Path("internal/assetprivacy"), Path("docs/native-asset-privacy"))
    files = [p for root in roots for p in root.rglob("*")
             if p.suffix in (".go", ".rs", ".md") and "target" not in p.parts]
    for root in ("ledger", "node", "config", "data/transactions"):
        files.extend(Path(root).rglob("assetprivacy*.go"))
    files.extend(Path("scripts").glob("native_privacy*.py"))
    return sorted(set(files))


def mask(text):
    return TOKEN.sub(lambda match: "".join("\n" if c == "\n" else " " for c in match[0]), text)


def functions(text):
    masked = mask(text)
    for match in FUNCTION.finditer(text):
        opening = masked.find("{", match.end())
        if opening < 0:
            continue
        depth, at = 1, opening + 1
        while at < len(masked) and depth:
            depth += (masked[at] == "{") - (masked[at] == "}")
            at += 1
        start = masked.count("\n", 0, match.start()) + 1
        yield start, masked.count("\n", 0, at) + 1


def spans(path, text):
    if path.suffix == ".py":
        return [(n.lineno, n.end_lineno) for n in ast.walk(ast.parse(text))
                if isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef))]
    if path.suffix in (".go", ".rs"):
        return functions(text)
    return ()


def audit(path):
    text = path.read_text()
    exempt = path.parent == Path("docs/native-asset-privacy") and path.name in ("README.md", "IMPLEMENTATION.md")
    errors = []
    if len(text.splitlines()) > 100 and not exempt:
        errors.append(f"{path}: file exceeds 100 physical lines")
    for start, end in spans(path, text):
        if end - start + 1 > 20:
            errors.append(f"{path}:{start}: function has {end - start + 1} physical lines")
    return errors


def main():
    files = sources()
    errors = [error for path in files for error in audit(path)]
    for error in errors:
        print(error)
    print(f"Checked {len(files)} authored files; {len(errors)} line-limit violations")
    raise SystemExit(bool(errors))


if __name__ == "__main__":
    main()
