test:
    # Use stdin to properly test
    # start and end is none
    echo '{ \
        "id": 0, \
        "uuid": "d67fce70-c0b6-43c5-affc-a21e64567d40", \
        "description": "My description.", \
        "entry": "20220131T083000Z", \
        "modified": "20220131T083000Z", \
        "status": "pending", \
        "urgency": 0.0 \
    }' | cargo run --bin on-add_elapsed
    # start is none
    echo '{ \
        "id": 0, \
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
        "id": 0, \
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
        "id": 0, \
        "uuid": "d67fce70-c0b6-43c5-affc-a21e64567d40", \
        "description": "My description.", \
        "entry": "20220131T083000Z", \
        "modified": "20220131T083000Z", \
        "status": "pending", \
        "urgency": 0.0, \
        "start": "20220131T083000Z", \
        "end": "20220131T090000Z" \
    }' | cargo run --bin on-add_elapsed
