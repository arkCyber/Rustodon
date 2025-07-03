#!/bin/bash

# Rustodon Performance Test Script
# Tests the server under high concurrency (1k users)
# Author: arkSong (arksong2018@gmail.com)

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SERVER_URL="http://localhost:3000"
TEST_DURATION=60  # 1 minute
CONCURRENT_USERS=1000
RAMP_UP_TIME=20    # 20 seconds ramp-up
TEST_ENDPOINTS=(
    "/api/v1/instance"
    "/api/v1/apps"
    "/api/v1/accounts/verify_credentials"
    "/api/v1/statuses"
    "/api/v1/timelines/home"
    "/api/v1/timelines/public"
)

# Performance test results
RESULTS_DIR="performance_results_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$RESULTS_DIR"

echo -e "${BLUE}🚀 Rustodon Performance Test Suite${NC}"
echo -e "${BLUE}================================${NC}"
echo -e "Server URL: ${GREEN}$SERVER_URL${NC}"
echo -e "Test Duration: ${GREEN}${TEST_DURATION}s${NC}"
echo -e "Concurrent Users: ${GREEN}$CONCURRENT_USERS${NC}"
echo -e "Ramp-up Time: ${GREEN}${RAMP_UP_TIME}s${NC}"
echo -e "Results Directory: ${GREEN}$RESULTS_DIR${NC}"
echo ""

# Check if server is running
check_server() {
    echo -e "${YELLOW}🔍 Checking if server is running...${NC}"
    if curl -s "$SERVER_URL/api/v1/instance" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ Server is running${NC}"
        return 0
    else
        echo -e "${RED}❌ Server is not running on $SERVER_URL${NC}"
        echo -e "${YELLOW}Please start the server first:${NC}"
        echo -e "  cargo run -p rustodon-server --release"
        return 1
    fi
}

# Install required tools
install_tools() {
    echo -e "${YELLOW}🔧 Installing required tools...${NC}"

    # Check if wrk is installed
    if ! command -v wrk &> /dev/null; then
        echo -e "${YELLOW}Installing wrk...${NC}"
        if [[ "$OSTYPE" == "darwin"* ]]; then
            brew install wrk
        elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
            sudo apt-get update && sudo apt-get install -y wrk
        else
            echo -e "${RED}Please install wrk manually for your platform${NC}"
            exit 1
        fi
    fi

    # Check if hey is installed
    if ! command -v hey &> /dev/null; then
        echo -e "${YELLOW}Installing hey...${NC}"
        if [[ "$OSTYPE" == "darwin"* ]]; then
            brew install hey
        elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
            wget https://github.com/rakyll/hey/releases/download/v0.1.4/hey_linux_amd64
            chmod +x hey_linux_amd64
            sudo mv hey_linux_amd64 /usr/local/bin/hey
        else
            echo -e "${RED}Please install hey manually for your platform${NC}"
            exit 1
        fi
    fi

    echo -e "${GREEN}✅ All tools installed${NC}"
}

# System resource monitoring
monitor_resources() {
    echo -e "${YELLOW}📊 Starting resource monitoring...${NC}"

    # Monitor CPU, memory, and network
    (
        while true; do
            echo "$(date '+%Y-%m-%d %H:%M:%S') $(ps aux | grep rustodon-server | grep -v grep | awk '{print $3, $4}' | head -1)" >> "$RESULTS_DIR/system_resources.log"
            sleep 5
        done
    ) &
    MONITOR_PID=$!

    echo -e "${GREEN}✅ Resource monitoring started (PID: $MONITOR_PID)${NC}"
}

# Stop resource monitoring
stop_monitoring() {
    if [ ! -z "$MONITOR_PID" ]; then
        echo -e "${YELLOW}🛑 Stopping resource monitoring...${NC}"
        kill $MONITOR_PID 2>/dev/null || true
        echo -e "${GREEN}✅ Resource monitoring stopped${NC}"
    fi
}

# Basic load test with wrk
basic_load_test() {
    echo -e "${BLUE}📈 Running basic load test...${NC}"

    for endpoint in "${TEST_ENDPOINTS[@]}"; do
        echo -e "${YELLOW}Testing endpoint: $endpoint${NC}"

        wrk -t12 -c1000 -d60s -L "$SERVER_URL$endpoint" > "$RESULTS_DIR/wrk_$(echo $endpoint | tr '/' '_').log" 2>&1

        echo -e "${GREEN}✅ Completed load test for $endpoint${NC}"
    done
}

# High concurrency test with hey
high_concurrency_test() {
    echo -e "${BLUE}🔥 Running high concurrency test (10k users)...${NC}"

    # Test each endpoint with high concurrency
    for endpoint in "${TEST_ENDPOINTS[@]}"; do
        echo -e "${YELLOW}Testing $endpoint with $CONCURRENT_USERS concurrent users...${NC}"

        hey -n $CONCURRENT_USERS -c $CONCURRENT_USERS -z ${TEST_DURATION}s \
            -H "Content-Type: application/json" \
            "$SERVER_URL$endpoint" > "$RESULTS_DIR/hey_$(echo $endpoint | tr '/' '_').log" 2>&1

        echo -e "${GREEN}✅ Completed high concurrency test for $endpoint${NC}"
    done
}

# Stress test
stress_test() {
    echo -e "${BLUE}💥 Running stress test...${NC}"

    # Gradually increase load
    for users in 100 500 1000 5000 10000; do
        echo -e "${YELLOW}Stress testing with $users concurrent users...${NC}"

        hey -n $((users * 10)) -c $users -z 30s \
            -H "Content-Type: application/json" \
            "$SERVER_URL/api/v1/instance" > "$RESULTS_DIR/stress_${users}_users.log" 2>&1

        echo -e "${GREEN}✅ Completed stress test with $users users${NC}"
        sleep 10  # Cool down period
    done
}

# Database performance test
database_performance_test() {
    echo -e "${BLUE}🗄️ Running database performance test...${NC}"

    # Test database-heavy endpoints
    local db_endpoints=(
        "/api/v1/statuses"
        "/api/v1/timelines/home"
        "/api/v1/timelines/public"
    )

    for endpoint in "${db_endpoints[@]}"; do
        echo -e "${YELLOW}Testing database performance for $endpoint...${NC}"

        hey -n 1000 -c 100 -z 60s \
            -H "Content-Type: application/json" \
            "$SERVER_URL$endpoint" > "$RESULTS_DIR/db_perf_$(echo $endpoint | tr '/' '_').log" 2>&1

        echo -e "${GREEN}✅ Completed database performance test for $endpoint${NC}"
    done
}

# Memory leak test
memory_leak_test() {
    echo -e "${BLUE}🧠 Running memory leak test...${NC}"

    # Long-running test to check for memory leaks
    echo -e "${YELLOW}Running 10-minute memory leak test...${NC}"

    hey -n 10000 -c 100 -z 600s \
        -H "Content-Type: application/json" \
        "$SERVER_URL/api/v1/instance" > "$RESULTS_DIR/memory_leak_test.log" 2>&1

    echo -e "${GREEN}✅ Completed memory leak test${NC}"
}

# Generate performance report
generate_report() {
    echo -e "${BLUE}📋 Generating performance report...${NC}"

    cat > "$RESULTS_DIR/performance_report.md" << EOF
# Rustodon Performance Test Report

**Test Date:** $(date)
**Server URL:** $SERVER_URL
**Test Duration:** ${TEST_DURATION}s
**Concurrent Users:** $CONCURRENT_USERS
**Ramp-up Time:** ${RAMP_UP_TIME}s

## Test Summary

### System Resources
- CPU Usage: $(tail -n 20 "$RESULTS_DIR/system_resources.log" | awk '{sum+=$2} END {print sum/NR "%%"}')
- Memory Usage: $(tail -n 20 "$RESULTS_DIR/system_resources.log" | awk '{sum+=$3} END {print sum/NR "%%"}')

### Load Test Results

EOF

    # Add wrk results
    for log_file in "$RESULTS_DIR"/wrk_*.log; do
        if [ -f "$log_file" ]; then
            endpoint=$(basename "$log_file" .log | sed 's/wrk_//' | tr '_' '/')
            echo "#### $endpoint" >> "$RESULTS_DIR/performance_report.md"
            echo '```' >> "$RESULTS_DIR/performance_report.md"
            cat "$log_file" >> "$RESULTS_DIR/performance_report.md"
            echo '```' >> "$RESULTS_DIR/performance_report.md"
            echo "" >> "$RESULTS_DIR/performance_report.md"
        fi
    done

    # Add hey results
    for log_file in "$RESULTS_DIR"/hey_*.log; do
        if [ -f "$log_file" ]; then
            endpoint=$(basename "$log_file" .log | sed 's/hey_//' | tr '_' '/')
            echo "#### $endpoint (High Concurrency)" >> "$RESULTS_DIR/performance_report.md"
            echo '```' >> "$RESULTS_DIR/performance_report.md"
            cat "$log_file" >> "$RESULTS_DIR/performance_report.md"
            echo '```' >> "$RESULTS_DIR/performance_report.md"
            echo "" >> "$RESULTS_DIR/performance_report.md"
        fi
    done

    echo -e "${GREEN}✅ Performance report generated: $RESULTS_DIR/performance_report.md${NC}"
}

# Cleanup function
cleanup() {
    echo -e "${YELLOW}🧹 Cleaning up...${NC}"
    stop_monitoring
    echo -e "${GREEN}✅ Cleanup completed${NC}"
}

# Main test execution
main() {
    echo -e "${BLUE}🚀 Starting Rustodon Performance Test Suite${NC}"
    echo ""

    # Set up cleanup on exit
    trap cleanup EXIT

    # Check prerequisites
    check_server || exit 1
    install_tools

    # Create results directory
    mkdir -p "$RESULTS_DIR"

    # Start resource monitoring
    monitor_resources

    # Run tests
    echo -e "${BLUE}📊 Running performance tests...${NC}"
    echo ""

    basic_load_test
    echo ""

    high_concurrency_test
    echo ""

    stress_test
    echo ""

    database_performance_test
    echo ""

    memory_leak_test
    echo ""

    # Generate report
    generate_report

    echo -e "${GREEN}🎉 Performance testing completed!${NC}"
    echo -e "${GREEN}📁 Results saved in: $RESULTS_DIR${NC}"
    echo -e "${GREEN}📋 Report: $RESULTS_DIR/performance_report.md${NC}"
}

# Run main function
main "$@"
