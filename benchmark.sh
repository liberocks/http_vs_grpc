#!/bin/bash
set -e

# Check if the server is running
curl -s -o /dev/null -X POST \
  -H "Content-Type: application/json" \
  -d '{
    "id": 1,
    "title": "Example Book",
    "author": "John Doe",
    "description": "A wonderful book",
    "published_at": 2022
  }' \
  http://127.0.0.1:3000 || (echo "Server is not running" && exit 1)

# Run the benchmark
PORT=3000 autocannon -c 500 -d 10 -p 10 -w 10 -m POST \
  -H "Content-Type:application/json" \
  -b '{"id": 1, "title": "Example Book", "author": "John Doe", "description": "A wonderful book", "published_at": 2022 }' \
  http://127.0.0.1:3000/
