#!/bin/bash

set -eux

# Code inspired by https://github.com/blas-lapack-rs/lapack-sys/blob/master/bin/generate.sh
bindgen --allowlist-function='^.*_$' --use-core wrapper.h \
  | sed -e 's/::std::os::raw:://g' \
  | sed -e '/__darwin_size_t/d' \
  > pfapack.rs

rustfmt pfapack.rs
