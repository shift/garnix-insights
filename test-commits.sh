#!/usr/bin/env bash
set -e

# Test script for garnix-fetcher with specific commits
# Usage: ./test-commits.sh <JWT_TOKEN>

if [ $# -eq 0 ]; then
    echo "Usage: $0 <JWT_TOKEN>"
    echo "Tests garnix-fetcher with commits: 47fb520b8301a5783311987fa36c0ab38159b458 and bc2f2d2c4ad98b921e7fa64ddb690cf414e06791"
    exit 1
fi

JWT_TOKEN="$1"
COMMIT1="47fb520b8301a5783311987fa36c0ab38159b458"
COMMIT2="bc2f2d2c4ad98b921e7fa64ddb690cf414e06791"

echo "🧪 Testing garnix-fetcher with provided commits..."
echo

echo "📦 Building project first..."
nix build .#default
echo "✅ Build complete"
echo

echo "🔍 Testing commit 1: $COMMIT1"
echo "----------------------------------------"
./result/bin/garnix-fetcher "$JWT_TOKEN" "$COMMIT1"
echo
echo "🔍 Testing commit 1 with JSON output:"
echo "----------------------------------------"
./result/bin/garnix-fetcher "$JWT_TOKEN" "$COMMIT1" --json-output | head -20
echo "... (truncated)"
echo

echo "🔍 Testing commit 2: $COMMIT2"
echo "----------------------------------------"
./result/bin/garnix-fetcher "$JWT_TOKEN" "$COMMIT2"
echo

echo "🌐 Testing API server mode..."
echo "----------------------------------------"
echo "Starting server in background..."
./result/bin/garnix-fetcher --server &
SERVER_PID=$!
sleep 3

echo "Testing API endpoint with commit 1..."
curl -X POST http://127.0.0.1:8080/build-status \
  -H "Content-Type: application/json" \
  -d "{\"jwt_token\": \"$JWT_TOKEN\", \"commit_id\": \"$COMMIT1\"}" \
  -s | jq '.summary' || echo "No jq available, raw response above"

echo
echo "Stopping server..."
kill $SERVER_PID 2>/dev/null

echo "✅ All tests completed!"
