#!/usr/bin/env python3
# Generate minimal bytecode for testing

import marshal
import struct
import sys

def create_simple_pyc(code_obj, filename):
    python_major = sys.version_info[0]
    python_minor = sys.version_info[1]
    
    magic = {
        (3, 12): b'\x55\x0d\x0d\x0a',
    }.get((python_major, python_minor), b'\x42\x0d\x0d\x0a')
    
    flags = b'\x00\x00\x00\x00'
    timestamp = struct.pack('<I', 0)
    size = struct.pack('<I', 0)
    
    with open(filename, 'wb') as f:
        f.write(magic)
        f.write(flags)
        f.write(timestamp)
        f.write(size)
        marshal.dump(code_obj, f)

# Create simple code objects
code_hello = compile('print("Hello")', 'hello.py', 'exec')
create_simple_pyc(code_hello, 'pyc/hello.pyc')

code_add = compile('a = 1 + 2\nprint(a)', 'add.py', 'exec')
create_simple_pyc(code_add, 'pyc/add.pyc')

code_loop = compile('i = 0\nwhile i < 3:\n    print(i)\n    i = i + 1', 'loop.py', 'exec')
create_simple_pyc(code_loop, 'pyc/loop.pyc')

code_list = compile('lst = [1,2,3]\nprint(lst)', 'list.py', 'exec')
create_simple_pyc(code_list, 'pyc/list.pyc')

print("Created test .pyc files")