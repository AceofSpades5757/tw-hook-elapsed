# For Rust logging
#export RUST_LOG := "trace,error,warn,debug,info"

help:
    @just --list

build:
    # Linux (cross-compile)
    cargo build --release --bin on-add_noop --target x86_64-unknown-linux-musl

test1:
    echo '{ \
        "uuid": "d67fce70-c0b6-43c5-affc-a21e64567d40", \
        "description": "My description.", \
        "entry": "20220131T083000Z", \
        "modified": "20220131T083000Z", \
        "status": "pending" \
    }' | cargo run --bin on-add_elapsed
test2:
    # Modify test (delete task)
    printf '{ \
        "uuid": "d67fce70-c0b6-43c5-affc-a21e64567d40", \
        "description": "My description.", \
        "entry": "20220131T083000Z", \
        "modified": "20220131T083000Z", \
        "status": "pending" \
    }\n{}' | cargo run --bin on-modify_elapsed

test:
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
