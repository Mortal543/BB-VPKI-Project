#!/usr/bin/env bash
set -euo pipefail

MD="docs/run_instructions.md"
OUT="docs/run_instructions.pdf"

if ! command -v pandoc >/dev/null 2>&1; then
  echo "pandoc not found. Please install pandoc (brew install pandoc) or see docs/run_instructions.md for instructions." >&2
  exit 2
fi

echo "Generating PDF from ${MD} → ${OUT}"

pandoc "${MD}" -o "${OUT}" --pdf-engine=xelatex || {
  echo "pandoc failed with xelatex engine; trying default engine..." >&2
  pandoc "${MD}" -o "${OUT}"
}

echo "Done: ${OUT}"
