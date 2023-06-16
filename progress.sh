#!/usr/bin/env sh
# Show the progress from a specific episode.
#
# usage:
#   ./progress.sh 01

awk "/## Ep${1}/,/---/" progress-so-far.md
