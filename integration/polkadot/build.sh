#!/bin/bash
# SPDX-License-Identifier: Apache-2.0

set -e

dup_contracts=$(grep -r '^contract .* {' | grep -v node_modules | awk '{ print $2 }' | sort | uniq -d)
if [[ $dup_contracts ]]; then
	echo "Found contract with duplicate names: ${dup_contracts}"
	/bin/false
else
	parallel solang compile -v -g --wasm-opt Z --target polkadot ::: *.sol test/*.sol tornado/contracts/*.sol
	solang compile -v --target polkadot --release release_version.sol
fi

