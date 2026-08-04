#!/usr/bin/env bash
# Compile tikz figures to standalone SVGs for the reveal.js deck.
#   pdflatex (crop to standalone) -> PDF -> pdftocairo -svg (keeps vectors, no Ghostscript needed)
#
# Usage:  ./build-figures.sh [fig1 fig2 ...]
#   With no args, rebuilds the default list below.
#   Figure names are basenames (no .tikz) found in $FIGDIR.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"          # repo root (…/e_hypergraphs_paper)
FIGDIR="$ROOT/slides/Lab_Lunch/figures"
ASSETS="$ROOT/slides/MSFP_2026/reveal.js/assets/figures"
mkdir -p "$ASSETS"

default_figs=(
  e-graph-generic-example
  e-graph-example-a-no-label
  e-graph-example-b-add e-graph-example-b-merged
  e-graph-example-c-add e-graph-example-c-merged
  e-graph-example-d-merged e-graph-example-e-merged
  e-graphs-binding-example-plain e-graphs-binding-example-indices
)
figs=("${@:-}")
[ -z "${figs[*]}" ] && figs=("${default_figs[@]}")

BUILD="$(mktemp -d)"
trap 'rm -rf "$BUILD"' EXIT
cat > "$BUILD/preamble.tex" <<'EOF'
\documentclass[border=3pt]{standalone}
\usepackage{tikz}
\usepackage{tikzit}
\usepackage{amsmath}
\usepackage{amssymb}
\usepackage{stmaryrd}
\usepackage{tikz-network}
\usepackage{mathtools}
\input{hypergraph.tikzdefs}
\input{sample.tikzstyles}
\input{hypergraph.tikzstyles}
\begin{document}
\input{FIGURE}
\end{document}
EOF

cd "$BUILD"
for f in "${figs[@]}"; do
  sed "s|FIGURE|$FIGDIR/$f.tikz|" preamble.tex > "$f.tex"
  TEXINPUTS="$ROOT:" pdflatex -interaction=nonstopmode -halt-on-error "$f.tex" >"$f.log" 2>&1
  pdftocairo -svg "$f.pdf" "$ASSETS/$f.svg"
  echo "  built $f.svg"
done
echo "Done -> $ASSETS"
