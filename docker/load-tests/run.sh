#!/bin/bash

# Load Testing Script for LanguageTool API
# Usage: ./run.sh [baseline|target|stress|all]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RESULTS_DIR="$SCRIPT_DIR/results"
API_URL="${API_URL:-https://languagetool-autocorrect.fly.dev}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if k6 is installed
check_k6() {
    if ! command -v k6 &> /dev/null; then
        echo -e "${RED}Error: k6 is not installed${NC}"
        echo "Install with: brew install k6"
        exit 1
    fi
    echo -e "${GREEN}✓ k6 found: $(k6 version)${NC}"
}

# Create results directory
setup() {
    mkdir -p "$RESULTS_DIR"
    echo -e "${BLUE}Results will be saved to: $RESULTS_DIR${NC}"
    echo -e "${BLUE}Target API: $API_URL${NC}"
    echo ""
}

# Run a specific test
run_test() {
    local test_name=$1
    local test_file="$SCRIPT_DIR/${test_name}.js"

    if [ ! -f "$test_file" ]; then
        echo -e "${RED}Error: Test file not found: $test_file${NC}"
        exit 1
    fi

    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${YELLOW}Running: $test_name${NC}"
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""

    k6 run \
        --env API_URL="$API_URL" \
        --out json="$RESULTS_DIR/${test_name}_$(date +%Y%m%d_%H%M%S).json" \
        "$test_file"

    echo ""
    echo -e "${GREEN}✓ $test_name completed${NC}"
    echo ""
}

# Print usage
usage() {
    echo "Usage: $0 [command]"
    echo ""
    echo "Commands:"
    echo "  baseline   Run baseline test (10 users, 5 min)"
    echo "  target     Run target test (100 users, 10 min)"
    echo "  stress     Run stress test (up to 500 users)"
    echo "  all        Run all tests sequentially"
    echo "  check      Verify k6 installation and API connectivity"
    echo ""
    echo "Environment variables:"
    echo "  API_URL    Override the default API URL"
    echo "             Default: https://languagetool-autocorrect.fly.dev"
    echo ""
    echo "Examples:"
    echo "  $0 baseline"
    echo "  API_URL=http://localhost:8010 $0 baseline"
}

# Check API connectivity
check_api() {
    echo -e "${BLUE}Checking API connectivity...${NC}"

    local response
    response=$(curl -s -o /dev/null -w "%{http_code}" "$API_URL/v2/languages" --max-time 10)

    if [ "$response" == "200" ]; then
        echo -e "${GREEN}✓ API is reachable at $API_URL${NC}"
    else
        echo -e "${RED}✗ API not reachable (HTTP $response)${NC}"
        echo "Make sure the server is running and accessible"
        exit 1
    fi
}

# Main
main() {
    check_k6
    setup

    case "${1:-}" in
        baseline)
            check_api
            run_test "baseline"
            ;;
        target)
            check_api
            run_test "target"
            ;;
        stress)
            check_api
            run_test "stress"
            ;;
        all)
            check_api
            echo -e "${BLUE}Running all tests sequentially...${NC}"
            echo ""
            run_test "baseline"
            sleep 30  # Cooldown between tests
            run_test "target"
            sleep 30
            run_test "stress"
            echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
            echo -e "${GREEN}All tests completed!${NC}"
            echo -e "${GREEN}Results saved in: $RESULTS_DIR${NC}"
            echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
            ;;
        check)
            check_api
            echo -e "${GREEN}Everything looks good!${NC}"
            ;;
        *)
            usage
            exit 1
            ;;
    esac
}

main "$@"
