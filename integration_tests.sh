#!/bin/bash

test() {
    echo "Testing: $TITLE"

    RESPONSE=$(curl -sS -w "%{response_code}" -X "$METHOD" \
        -H "Content-Type: application/json" ${BODY:+-d "$BODY"} \
        "http://localhost:8080/v1$ENDPOINT${QUERY:+?$QUERY}" 2>&1)
    CONTENT="${RESPONSE:: -3}"
    STATUS_CODE="${RESPONSE: -3}"

    if [ "$STATUS_CODE" -ne "$EXPECTED_STATUS_CODE" ]; then
        echo "Test Failed: expected HTTP status $EXPECTED_STATUS_CODE but got $STATUS_CODE with content '$CONTENT'"
        exit 1
    elif [ -n "$EXPECTED_CONTENT" ] && [ "$CONTENT" != "$EXPECTED_CONTENT" ]; then
        echo "Test Failed: expected response '$EXPECTED_CONTENT' but got '$CONTENT'"
        exit 1
    else
        echo "Test Success!"
        echo
    fi

    unset BODY
    unset QUERY
}

TITLE='test_compare_valid'
METHOD='GET'
ENDPOINT='/compare'
QUERY='start_date=2025-01-01&end_date=2025-01-03'
EXPECTED_STATUS_CODE=200
EXPECTED_CONTENT='[{"day": "2025-01-01", "average_price": 101.05}, {"day": "2025-01-02", "average_price": 102.45}]'
test

TITLE='test_compare_invalid_start_date'
METHOD='GET'
ENDPOINT='/compare'
QUERY='start_date=invalid&end_date=2023-01-31'
EXPECTED_STATUS_CODE=400
EXPECTED_CONTENT='start_date is invalid'
test

TITLE='test_compare_invalid_end_date'
METHOD='GET'
ENDPOINT='/compare'
QUERY='start_date=2023-01-01&end_date=invalid'
EXPECTED_STATUS_CODE=400
EXPECTED_CONTENT='end_date is invalid'
test

TITLE='test_compare_start_after_end'
METHOD='GET'
ENDPOINT='/compare'
QUERY='start_date=2023-02-01&end_date=2023-01-31'
EXPECTED_STATUS_CODE=400
EXPECTED_CONTENT='start_date is after end_date'
test
