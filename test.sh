#!/bin/sh
# Running Parity Full Test Suite

FEATURES="json-tests,ci-skip-issue"
OPTIONS="--release"
VALIDATE=1

case $1 in
  --no-json)
    FEATURES="ipc"
    shift # past argument=value
    ;;
  --no-release)
    OPTIONS=""
    shift
    ;;
  --no-validate)
    VALIDATE=0
    shift
    ;;
  --no-run)
    OPTIONS="--no-run"
    shift
    ;;
  *)
    # unknown option
    ;;
esac

set -e

if [ "$VALIDATE" -eq "1" ]; then
# Validate --no-default-features build
echo "________Validate build________"
time cargo check --no-default-features
time cargo check --manifest-path util/io/Cargo.toml --no-default-features
time cargo check --manifest-path util/io/Cargo.toml --features "mio"

# Validate chainspecs
echo "________Validate chainspecs________"
time ./scripts/validate_chainspecs.sh
fi

# Running the C++ example
echo "________Running the C++ example________"
cd bpp-clib-examples/cpp && \
  mkdir -p build && \
  cd build && \
  cmake .. && \
  make -j 8 && \
  ./parity-example && \
  cd .. && \
  rm -rf build && \
  cd ../..

# Running tests
echo "________Running Parity Full Test Suite________"
git submodule update --init --recursive
time cargo test  $OPTIONS --features "$FEATURES" --all $1 -- --test-threads 8
