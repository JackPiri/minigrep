#!/bin/bash
set -eEuo pipefail

# Initialize counters for unit tests
unit_total_passed=1
unit_total_failed=2
unit_total_runtime=3.4

passed_formatted="*Passed*: ${e2e_total_passed}"
failed_formatted="*Failed*: ${e2e_total_failed}"
runtime_formatted="*Runtime*: ${e2e_total_runtime}s"
echo N0-B
echo "E2E_TEST_SUMMARY=*E2E Tests*\n${passed_formatted}, ${failed_formatted}, ${runtime_formatted}" >> $GITHUB_ENV
echo N0-A