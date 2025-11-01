import subprocess
import os 
import sys

def test_memory_leaks():
    # Quick hack to check if nanobind is complaining about leaks.
    dir_path = os.path.dirname(os.path.realpath(__file__))
    out = subprocess.run([sys.executable, dir_path + "/memory_leak_subprocess.py"], capture_output=True)
    assert len(out.stderr) == 0