import time
import json
import statistics
import sys
import os

# Import the L0 Oracle
sys.path.append(os.path.abspath("test_harness"))
from rsl_lam_v5 import rsl_v5, surviving_structure

def benchmark_rsl_v5(iterations=1000):
    print(f"[Benchmark] Running RSL v5 Interrogation {iterations} times...")
    
    reg_hom = {(2, 2): {"lam_verified": True, "authorized_anchors": []}}
    input_s = {'primes': [2], 'anchors': 10, 'passes': 5}
    
    latencies = []
    for _ in range(iterations):
        start = time.perf_counter()
        rsl_v5(2, 2, reg_hom, input_s)
        end = time.perf_counter()
        latencies.append((end - start) * 1000) # Convert to ms
        
    stats = {
        "mean_ms": statistics.mean(latencies),
        "p50_ms": statistics.median(latencies),
        "p95_ms": sorted(latencies)[int(iterations * 0.95)],
        "p99_ms": sorted(latencies)[int(iterations * 0.99)],
        "ops_per_sec": 1000 / statistics.mean(latencies)
    }
    return stats

def run_benchmarks():
    # 1. RSL v5 Benchmark
    rsl_stats = benchmark_rsl_v5(1000)
    
    # 2. Baseline comparison (from governance_overhead.json)
    # Policy eval (1 rules) Mean: 0.0053 ms
    baseline_mean_ms = 0.0053
    overhead_multiplier = rsl_stats["mean_ms"] / baseline_mean_ms
    
    report = {
        "system": "WORM/Goldilocks",
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "rsl_v5_stats": rsl_stats,
        "baseline_comparison": {
            "baseline_policy_eval_mean_ms": baseline_mean_ms,
            "overhead_multiplier": overhead_multiplier
        },
        "status": "VALIDATED" if overhead_multiplier < 5.0 else "OPTIMIZATION REQUIRED"
    }
    
    with open("projects/worm/WORM_BENCHMARK_RESULTS.json", "w") as f:
        json.dump(report, f, indent=2)
    
    print("\n--- WORM Benchmark Results ---")
    print(json.dumps(report, indent=2))

if __name__ == "__main__":
    run_benchmarks()
