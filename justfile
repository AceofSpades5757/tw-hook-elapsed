# For Rust logging
#export RUST_LOG := "trace,error,warn,debug,info"
# For Rust backtracing
#export RUST_BACKTRACE := 1

BUILD := "./build"

help:
    @just --list

build:
    # Linux (cross-compile)
    cargo build --release --bin on-add_noop --target x86_64-unknown-linux-musl

install:
    # Linux (cross-compile)
    #cargo install --root {{BUILD}} --target x86_64-unknown-linux-musl
    # Create binaries in {{BUILD}}/bin
    cargo install --root {{BUILD}} --path .
    # Copy binaries to hooks directory
    cp {{BUILD}}/bin/* ~/.task/hooks/

clean:
    cargo clean
    rm -rf {{BUILD}}

test: test-cargo-test test-on-add test-on-modify


test-cargo-test:
    cargo test


test-on-modify:
    # Not Implemented
    # Needs CLI arguments added, or support for stdin-only


test-on-add:
    # Blackbox Tests

    # echo '{ "uuid": "d67fce70-c0b6-43c5-affc-a21e64567d40", "description": "My description.", "entry": "20220131T083000Z", "modified": "20220131T083000Z", "status": "pending", "urgency": 0.0 }' | cargo run --bin on-add_noop
    # NOOP
    echo '{ \
        "uuid": "d67fce70-c0b6-43c5-affc-a21e64567d40", \
        "description": "My description.", \
        "entry": "20220131T083000Z", \
        "modified": "20220131T083000Z", \
        "status": "pending", \
        "urgency": 0.0 \
    }' | cargo run --bin on-add_noop
    # Use stdin to properly test
    # start and end is none
    echo '{ \
        "uuid": "d67fce70-c0b6-43c5-affc-a21e64567d40", \
        "description": "My description.", \
        "entry": "20220131T083000Z", \
        "modified": "20220131T083000Z", \
        "status": "pending", \
        "urgency": 0.0 \
    }' | cargo run --bin on-add_elapsed
    # start is none
    echo '{ \
        "uuid": "d67fce70-c0b6-43c5-affc-a21e64567d40", \
        "description": "My description.", \
        "entry": "20220131T083000Z", \
        "modified": "20220131T083000Z", \
        "status": "pending", \
        "urgency": 0.0, \
        "start": "20220131T083000Z" \
    }' | cargo run --bin on-add_elapsed
    # end is none
    echo '{ \
        "uuid": "d67fce70-c0b6-43c5-affc-a21e64567d40", \
        "description": "My description.", \
        "entry": "20220131T083000Z", \
        "modified": "20220131T083000Z", \
        "status": "pending", \
        "urgency": 0.0, \
        "end": "20220131T083000Z" \
    }' | cargo run --bin on-add_elapsed
    # start and end is present
    echo '{ \
        "uuid": "d67fce70-c0b6-43c5-affc-a21e64567d40", \
        "description": "My description.", \
        "entry": "20220131T083000Z", \
        "modified": "20220131T083000Z", \
        "status": "pending", \
        "urgency": 0.0, \
        "start": "20220131T083000Z", \
        "end": "20220131T090000Z" \
    }' | cargo run --bin on-add_elapsed
