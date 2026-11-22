#!/usr/bin/env bash
set -euo pipefail

if [[ -e AGENT_HANDOFF.md ]]; then
  echo "AGENT_HANDOFF.md already exists; leaving it untouched."
  exit 0
fi

cp docs/agent/AGENT_HANDOFF.template.md AGENT_HANDOFF.md
echo "Created local AGENT_HANDOFF.md (Git-ignored)."
