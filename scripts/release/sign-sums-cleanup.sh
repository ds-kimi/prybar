#!/bin/sh
# Removes the decoded update signing key; safe when nothing exists.
rm -rf "${RUNNER_TEMP:-/tmp}/prybar-update-sign"
