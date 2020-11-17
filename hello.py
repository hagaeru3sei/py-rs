#!/usr/bin/env python3
import sys
import os
import pprint
import typing
import re
import itertools
import time
from concurrent.futures import ThreadPoolExecutor
print("VERSION INFO:", sys.version_info)
print("MODULES:", sys.modules.keys())
print("PYTHONPATH:", os.environ.get('PYTHONPATH'))
print("PATH:", os.environ.get('PATH'))
print("")
try:
    import joblib
except ImportError as e:
    print("")
    print("ImportError!", "(joblib)", "-"*128)
    print(e)

try:
    import numpy as np
    print(f'NUMPY: {np}')
except ImportError as e:
    np = None
    print("")
    print("ImportError!", "(Numpy)", "-"*128)
    print(e)

def say(message="world"):
    max_key = max(dict(os.environ).keys())
    l = len(max_key)
    for k, v in os.environ.items():
        print(f'{k:{l}}: {v}')
    
    if np:
        a = np.random.rand(10000, 10000)
        s = time.time()
        _ = np.linalg.inv(a)
        print("exec time:", time.time() - s)

    return f'Hello {message}'

#if __name__ == "__main__":
#    say()