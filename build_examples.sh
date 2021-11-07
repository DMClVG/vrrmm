#!/bin/bash

mkdir -p examples/bin

python compiler.py -i examples/fibonacci.code -o examples/bin/fibonacci.bin
python compiler.py -i examples/bubble_sort.code -o examples/bin/bubble_sort.bin
python compiler.py -i examples/hello_world.code -o examples/bin/hello_world.bin
python compiler.py -i examples/print_string.code -o examples/bin/print_string.bin
python compiler.py -i examples/print_hex.code -o examples/bin/print_hex.bin