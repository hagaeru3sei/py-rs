import sys
import os
try:
    import numpy as np
except ImportError as e:
    np = None
    print(e)

def say(message="world"):
    print(f'numpy: {np}')
    max_key = max(dict(os.environ).keys())
    l = len(max_key)
    for k, v in os.environ.items():
        print(f'{k:{l}}: {v}')
    return f'Hello {message}'

#if __name__ == "__main__":
#    say()