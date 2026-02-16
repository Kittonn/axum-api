#!/bin/bash
set -e

ENV_FILE="crates/axum-api/.env"
K6_SCRIPT="k6/load-test.js"
RESULTS_DIR="k6/results"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

mkdir -p "$RESULTS_DIR"

# Function to update .env for TCP
use_tcp_kafka() {
    echo "Switching to TCP Kafka brokers..."
    # Uncomment TCP, comment Envoy
    sed -i '' 's/^# KAFKA_BROKERS=kafka1:29092,kafka2:29092,kafka3:29092/KAFKA_BROKERS=kafka1:29092,kafka2:29092,kafka3:29092/' "$ENV_FILE"
    sed -i '' 's/^KAFKA_BROKERS=envoy-client:9091/# KAFKA_BROKERS=envoy-client:9091/' "$ENV_FILE"
}

# Function to update .env for Envoy (QUIC)
use_envoy_kafka() {
    echo "Switching to Envoy (QUIC) Kafka brokers..."
    # Uncomment Envoy, comment TCP
    sed -i '' 's/^KAFKA_BROKERS=kafka1:29092,kafka2:29092,kafka3:29092/# KAFKA_BROKERS=kafka1:29092,kafka2:29092,kafka3:29092/' "$ENV_FILE"
    sed -i '' 's/^# KAFKA_BROKERS=envoy-client:9091/KAFKA_BROKERS=envoy-client:9091/' "$ENV_FILE"
}

run_benchmark() {
    local MODE=$1
    echo "========================================"
    echo "Starting Benchmark: $MODE"
    echo "========================================"

    echo "Restarting axum-api..."
    podman compose restart axum-api
    
    # Wait for service to be ready (simple sleep or health check could be added)
    echo "Waiting for service to stabilize..."
    sleep 10

    STATS_FILE="$RESULTS_DIR/stats_${MODE}_${TIMESTAMP}.csv"
    echo "Recording resource usage to $STATS_FILE..."
    
    # Header for CSV
    echo "Timestamp,Container,CPU,Mem" > "$STATS_FILE"

    # We want a continuous loop of stats during the test
    (
        while true; do
            # Get stats in CSV-like format: Name,CPU%,MemUsage
            # We strip the '%' from CPU and 'MB'/'GB' from Mem in Python, or try to clean it here.
            # Podman stats format is a bit tricky for raw numbers, so we keep it simple and parse in Python.
            # Using specific format to make parsing easier.
            timestamp=$(date +%s)
            podman stats --no-stream --format "{{.Name}},{{.CPUPerc}},{{.MemUsage}}" | while read line; do
                echo "$timestamp,$line" >> "$STATS_FILE"
            done
            sleep 2
        done
    ) &
    PID_LOOP=$!

    echo "Running k6 load test..."
    k6 run "$K6_SCRIPT" --summary-export "$RESULTS_DIR/k6_summary_${MODE}_${TIMESTAMP}.json"

    # Kill stats loop
    kill $PID_LOOP
    
    # Generate Plot
    if [ -f "scripts/plot_stats.py" ]; then
        echo "Generating HTML report..."
        python3 scripts/plot_stats.py "$STATS_FILE"
    fi

    echo "Benchmark $MODE completed."
}

# --- Main Execution ---

echo "Cleaning up existing containers to avoid conflicts..."
podman compose down

echo "Starting all infrastructure..."
podman compose up -d

echo "Waiting for services to start..."
# Function to wait for a container to be running
wait_for_container() {
    local container_name=$1
    echo "Waiting for $container_name..."
    until podman ps --format "{{.Names}}" | grep -q "$container_name"; do
        sleep 2
    done
    echo "$container_name is up."
}

# Wait for essential services
wait_for_container "kafka1"
wait_for_container "kafka2"
wait_for_container "kafka3"
wait_for_container "envoy-server"
wait_for_container "envoy-client"

echo "Waiting 30s for Kafka cluster to stabilize..."
sleep 30

# 1. Test TCP
use_tcp_kafka
run_benchmark "TCP"

# 2. Test Envoy/QUIC
use_envoy_kafka
run_benchmark "QUIC"

# Generate Comparison Report
if [ -f "scripts/plot_comparison.py" ]; then
    echo "Generating Comparison Report..."
    python3 scripts/plot_comparison.py
fi

echo "All benchmarks completed. Results in $RESULTS_DIR"
