
import json
import os
import glob
import sys

def find_latest_file(pattern):
    files = glob.glob(pattern)
    if not files:
        return None
    return max(files, key=os.path.getctime)

def load_json(filepath):
    with open(filepath, 'r') as f:
        return json.load(f)

def main():
    results_dir = "k6/results"
    
    # expected patterns from benchmark script: k6_summary_TCP_*.json and k6_summary_QUIC_*.json
    tcp_file = find_latest_file(os.path.join(results_dir, "k6_summary_TCP_*.json"))
    quic_file = find_latest_file(os.path.join(results_dir, "k6_summary_QUIC_*.json"))
    
    if not tcp_file or not quic_file:
        print("Error: Could not find both TCP and QUIC summary files to compare.")
        if tcp_file: print(f"Found TCP: {tcp_file}")
        if quic_file: print(f"Found QUIC: {quic_file}")
        sys.exit(1)
        
    print(f"Comparing:")
    print(f"TCP: {tcp_file}")
    print(f"QUIC: {quic_file}")
    
    tcp_data = load_json(tcp_file)
    quic_data = load_json(quic_file)
    
    # Extract Metrics
    metrics = {
        "RPS (req/s)": {
            "tcp": tcp_data["metrics"]["http_reqs"]["rate"],
            "quic": quic_data["metrics"]["http_reqs"]["rate"]
        },
        "Avg Latency (ms)": {
            "tcp": tcp_data["metrics"]["http_req_duration"]["avg"],
            "quic": quic_data["metrics"]["http_req_duration"]["avg"]
        },
        "P95 Latency (ms)": {
            "tcp": tcp_data["metrics"]["http_req_duration"]["p(95)"],
            "quic": quic_data["metrics"]["http_req_duration"]["p(95)"]
        },
        "P99 Latency (ms)": {
            "tcp": tcp_data["metrics"]["http_req_duration"]["p(99)"],
            "quic": quic_data["metrics"]["http_req_duration"]["p(99)"]
        },
        "Max Latency (ms)": {
            "tcp": tcp_data["metrics"]["http_req_duration"]["max"],
            "quic": quic_data["metrics"]["http_req_duration"]["max"]
        }
    }
    
    # Generate HTML
    html_content = f"""
<!DOCTYPE html>
<html>
<head>
    <title>Kafka Benchmark Comparison: TCP vs QUIC</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        body {{ font-family: sans-serif; margin: 20px; }}
        .chart-container {{ width: 60%; margin: 20px auto; }}
        h1, h2 {{ text-align: center; }}
        .file-info {{ text-align: center; font-size: 0.9em; color: #666; }}
    </style>
</head>
<body>
    <h1>Benchmark Comparison: TCP vs QUIC</h1>
    <div class="file-info">
        <p>TCP Source: {os.path.basename(tcp_file)}</p>
        <p>QUIC Source: {os.path.basename(quic_file)}</p>
    </div>
    
    <div class="chart-container">
        <canvas id="rpsChart"></canvas>
    </div>
    
    <div class="chart-container">
        <canvas id="latencyChart"></canvas>
    </div>

    <script>
        const rpsData = {{
            labels: ['RPS (higher is better)'],
            datasets: [
                {{
                    label: 'TCP',
                    data: [{metrics["RPS (req/s)"]["tcp"]}],
                    backgroundColor: 'rgba(54, 162, 235, 0.6)',
                    borderColor: 'rgba(54, 162, 235, 1)',
                    borderWidth: 1
                }},
                {{
                    label: 'QUIC',
                    data: [{metrics["RPS (req/s)"]["quic"]}],
                    backgroundColor: 'rgba(255, 99, 132, 0.6)',
                    borderColor: 'rgba(255, 99, 132, 1)',
                    borderWidth: 1
                }}
            ]
        }};

        const latencyData = {{
            labels: ['Avg', 'P95', 'P99', 'Max'],
            datasets: [
                {{
                    label: 'TCP',
                    data: [
                        {metrics["Avg Latency (ms)"]["tcp"]},
                        {metrics["P95 Latency (ms)"]["tcp"]},
                        {metrics["P99 Latency (ms)"]["tcp"]},
                        {metrics["Max Latency (ms)"]["tcp"]}
                    ],
                    backgroundColor: 'rgba(54, 162, 235, 0.6)',
                    borderColor: 'rgba(54, 162, 235, 1)',
                    borderWidth: 1
                }},
                {{
                    label: 'QUIC',
                    data: [
                        {metrics["Avg Latency (ms)"]["quic"]},
                        {metrics["P95 Latency (ms)"]["quic"]},
                        {metrics["P99 Latency (ms)"]["quic"]},
                        {metrics["Max Latency (ms)"]["quic"]}
                    ],
                    backgroundColor: 'rgba(255, 99, 132, 0.6)',
                    borderColor: 'rgba(255, 99, 132, 1)',
                    borderWidth: 1
                }}
            ]
        }};

        const commonOptions = {{
            scales: {{
                y: {{
                    beginAtZero: true
                }}
            }}
        }};

        new Chart(document.getElementById('rpsChart'), {{
            type: 'bar',
            data: rpsData,
            options: {{
                ...commonOptions,
                plugins: {{
                    title: {{
                        display: true,
                        text: 'Throughput (Requests Per Second)'
                    }}
                }}
            }}
        }});

        new Chart(document.getElementById('latencyChart'), {{
            type: 'bar',
            data: latencyData,
            options: {{
                ...commonOptions,
                plugins: {{
                    title: {{
                        display: true,
                        text: 'Latency (ms) - Lower is better'
                    }}
                }}
            }}
        }});
    </script>
</body>
</html>
    """
    
    output_html = os.path.join(results_dir, "benchmark_comparison.html")
    with open(output_html, 'w') as f:
        f.write(html_content)
        
    print(f"Generated Comparison Report: {output_html}")

if __name__ == "__main__":
    main()
