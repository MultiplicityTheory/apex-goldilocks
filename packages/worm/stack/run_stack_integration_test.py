import subprocess
import json

def run_integration_test():
    print("--- Initiating Sedona Spine Stack Integration Test ---")
    
    # 1. Run Goldilocks-Atlas Bridge Test
    print("[1/2] Testing Goldilocks-Atlas Bridge...")
    try:
        # We need to compile the C bridge first
        subprocess.run(["gcc", "projects/worm/stack/atlas/goldilocks_atlas_bridge.c", "-o", "goldilocks_atlas_bridge"], check=True)
        result = subprocess.run(["./goldilocks_atlas_bridge"], capture_output=True, text=True, check=True)
        print(result.stdout)
    except subprocess.CalledProcessError as e:
        print(f"Bridge Test Failed: {e}")
        return

    # 2. Run Hologram vGPU Operator Test
    print("[2/2] Testing Hologram vGPU Operator Adapter...")
    try:
        result = subprocess.run(["python3", "projects/worm/stack/hologram/hologram_adapter.py"], capture_output=True, text=True, check=True)
        print(result.stdout)
    except subprocess.CalledProcessError as e:
        print(f"Hologram Adapter Test Failed: {e}")
        return
        
    print("\n--- Stack Integration Test: SUCCESS ---")

if __name__ == "__main__":
    run_integration_test()
