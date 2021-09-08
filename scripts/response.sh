#!/bin/bash
near call $1 get_data_request "{\"nonce\": \"$2\"}" --accountId $1
near call $1 get_data_response "{\"nonce\": \"$2\"}" --accountId $1