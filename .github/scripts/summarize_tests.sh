#!/bin/bash
set -eEuo pipefail

# Initialize counters for unit tests
unit_total_passed=1
unit_total_failed=2
unit_total_ignored=0
unit_total_runtime=3.4

passed_formatted="*Passed*: ${unit_total_passed}"
failed_formatted="*Failed*: ${unit_total_failed}"
ignored_formatted="*Ignored*: ${unit_total_ignored}"
runtime_formatted="*Runtime*: ${unit_total_runtime}s"
echo "UNIT_TEST_SUMMARY=*Unit Tests*\n${passed_formatted}, ${failed_formatted}, ${ignored_formatted}, ${runtime_formatted}" >> $GITHUB_ENV
echo "INTEGRATION_TEST_SUMMARY=*Integration Tests*\n${passed_formatted}, ${failed_formatted}, ${ignored_formatted}, ${runtime_formatted}" >> $GITHUB_ENV
functions_count=70
functions_percent=71.11
lines_count=72
lines_percent=73.33
regions_count=74
regions_percent=75.55
instantiations_count=76
instantiations_percent=77.77
coverage_summary="*Test Coverage Summary (${lines_percent}%)*\n*Functions:* ${functions_count} (${functions_percent}%), *Lines:* ${lines_count} (${lines_percent}%), *Regions:* ${regions_count} (${regions_percent}%), *Instantiations:* ${instantiations_count} (${instantiations_percent}%)"
echo "COVERAGE_TEST_SUMMARY=${coverage_summary}" >> $GITHUB_ENV
echo "LINE_COVERAGE_PERCENT=${lines_percent}" >> $GITHUB_ENV
echo "E2E_TEST_SUMMARY=*E2E Tests*\n${passed_formatted}, ${failed_formatted}, ${runtime_formatted}" >> $GITHUB_ENV
