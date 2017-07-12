#!/bin/sh
# Werk - a pure Rust opus library
#
# Copyright (c) 2001-2011 the opus developers, and
# Copyright (c) 2017 est31 <MTest31@outlook.com>
# and contributors, All rights reserved.
# Licensed under the BSD 3 clause license.
# Please see the COPYING file attached to
# this source distribution for details.

cur_dir=`dirname "$0"`
cd "$cur_dir/werk_test"

tests=`ls src/bin | sort | cut -d. -f1 | grep test`

for test in $tests; do
	cargo run --release --bin $test
done
