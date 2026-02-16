
import sys
import csv
import json
import os

def parse_size(size_str):
    """Converts memory size string (e.g., '500MB', '1.2GB') to Megabytes (float)."""
    size_str = size_str.strip()
    if not size_str:
        return 0.0
    
    units = {
        'B': 1 / (1024 * 1024),
        'KB': 1 / 1024,
        'MB': 1,
        'GB': 1024,
        'TB': 1024 * 1024,
        'KiB': 1 / 1024, # treating as KB for simplicity or precise if needed
        'MiB': 1,
        'GiB': 1024
    }
    
    number = ""
    unit = ""
    
    # Simple parser
    for i, char in enumerate(size_str):
        if char.isdigit() or char == '.':
            number += char
        else:
            unit = size_str[i:].strip()
            break
            
    try:
        val = float(number)
    except ValueError:
        return 0.0
        
    multiplier = units.get(unit.upper(), 1) # Default to MB if unknown, or handle error
    # Heuristic for unknown units or bytes
    if not unit:
        return val # Assume MB? Or Bytes? Podman usually gives units.
        
    return val * multiplier

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 plot_stats.py <csv_file>")
        sys.exit(1)

    csv_file = sys.argv[1]
    if not os.path.exists(csv_file):
        print(f"Error: File {csv_file} not found.")
        sys.exit(1)

    data = {}
    timestamps = []
    
    # Containers to ignore (noise)
    ignore_list = ["infra-kafka-ui", "infra-redis", "infra-mssql"]

    print(f"Parsing {csv_file}...")
    
    with open(csv_file, 'r') as f:
        reader = csv.reader(f)
        header = next(reader, None) # Skip header
        
        for row in reader:
            if len(row) < 4:
                continue
                
            ts, name, cpu_str, mem_str = row[0], row[1], row[2], row[3]
            
            # Filter noise
            if any(ign in name for ign in ignore_list):
                continue
                
            # Parse CPU
            try:
                cpu_val = float(cpu_str.replace('%', ''))
            except ValueError:
                cpu_val = 0.0
                
            # Parse Mem
            mem_val = parse_size(mem_str)
            
            if name not in data:
                data[name] = {'cpu': [], 'mem': []}
            
            # Align timestamps (simple approach: just append, assuming mostly synchronous collection)
            # Better approach: store (ts, val) tuples
            data[name]['cpu'].append({'x': int(ts) * 1000, 'y': cpu_val})
            data[name]['mem'].append({'x': int(ts) * 1000, 'y': mem_val})


    # Generate HTML
    html_content = f"""
<!DOCTYPE html>
<html>
<head>
    <title>Kafka Benchmark Resource Usage</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/chartjs-adapter-date-fns"></script>
    <style>
        body {{ font-family: sans-serif; margin: 20px; }}
        .chart-container {{ width: 80%; margin: 20px auto; }}
        h1, h2 {{ text-align: center; }}
    </style>
</head>
<body>
    <h1>Benchmark Resource Usage</h1>
    <h3>Source: {os.path.basename(csv_file)}</h3>
    
    <div class="chart-container">
        <h2>CPU Usage (%)</h2>
        <canvas id="cpuChart"></canvas>
    </div>
    
    <div class="chart-container">
        <h2>Memory Usage (MB)</h2>
        <canvas id="memChart"></canvas>
    </div>

    <script>
        const data = {json.dumps(data)};
        
        const colors = [
            '#FF6384', '#36A2EB', '#FFCE56', '#4BC0C0', '#9966FF', '#FF9F40',
            '#C9CBCF', '#7BC225', '#A63F8B', '#2C8EBE'
        ];
        
        const cpuDatasets = Object.keys(data).map((name, index) => ({{
            label: name,
            data: data[name].cpu,
            borderColor: colors[index % colors.length],
            fill: false,
            tension: 0.1
        }}));
        
        const memDatasets = Object.keys(data).map((name, index) => ({{
            label: name,
            data: data[name].mem,
            borderColor: colors[index % colors.length],
            fill: false,
            tension: 0.1
        }}));

        const commonOptions = {{
            scales: {{
                x: {{
                    type: 'time',
                    time: {{
                        unit: 'second'
                    }},
                    title: {{
                        display: true,
                        text: 'Time'
                    }}
                }}
            }},
            plugins: {{
                legend: {{
                    position: 'right'
                }}
            }}
        }};

        new Chart(document.getElementById('cpuChart'), {{
            type: 'line',
            data: {{ datasets: cpuDatasets }},
            options: {{
                ...commonOptions,
                plugins: {{
                    ...commonOptions.plugins,
                    title: {{
                        display: true,
                        text: 'CPU Usage'
                    }}
                }},
                scales: {{
                    ...commonOptions.scales,
                    y: {{
                        beginAtZero: true,
                        title: {{
                            display: true,
                            text: 'CPU %'
                        }}
                    }}
                }}
            }}
        }});

        new Chart(document.getElementById('memChart'), {{
            type: 'line',
            data: {{ datasets: memDatasets }},
            options: {{
                ...commonOptions,
                plugins: {{
                    ...commonOptions.plugins,
                    title: {{
                        display: true,
                        text: 'Memory Usage'
                    }}
                }},
                scales: {{
                    ...commonOptions.scales,
                    y: {{
                        beginAtZero: true,
                        title: {{
                            display: true,
                            text: 'Memory (MB)'
                        }}
                    }}
                }}
            }}
        }});
    </script>
</body>
</html>
    """
    
    output_html = csv_file.replace('.csv', '.html')
    with open(output_html, 'w') as f:
        f.write(html_content)
        
    print(f"Generated HTML report: {output_html}")

if __name__ == "__main__":
    main()
