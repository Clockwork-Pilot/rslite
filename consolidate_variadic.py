#!/usr/bin/env python3
"""
Consolidate C variadic functions to printf_c_variadic.rs

This script:
1. Finds variadic functions (with "mut args: ..." pattern)
2. Extracts them and appends to printf_c_variadic.rs
3. Removes them from the source file
4. Adds re-exports in the source module
"""

import re
import sys
from pathlib import Path

def find_function_boundaries(lines, func_start_line):
    """Find the start and end lines of a function given the start line."""
    # Count braces to find the end of the function
    brace_count = 0
    in_function = False

    for i in range(func_start_line, len(lines)):
        line = lines[i]

        # Count opening and closing braces
        for char in line:
            if char == '{':
                in_function = True
                brace_count += 1
            elif char == '}':
                brace_count -= 1
                if in_function and brace_count == 0:
                    return func_start_line, i

    return func_start_line, len(lines) - 1

def extract_function(lines, func_start_line):
    """Extract a complete function definition from content."""
    # Find attributes like #[no_mangle] that precede the function
    attr_start = func_start_line
    for i in range(func_start_line - 1, -1, -1):
        line = lines[i].strip()
        if line.startswith('#[') or line == '':
            attr_start = i
        else:
            attr_start = i + 1
            break

    func_start, func_end = find_function_boundaries(lines, func_start_line)
    func_end_line = func_end + 1  # Include the closing brace

    # Extract lines
    func_lines = lines[attr_start:func_end_line]

    return ''.join(func_lines), attr_start, func_end_line

def find_variadic_functions(file_path):
    """Find all variadic functions in a file."""
    with open(file_path, 'r') as f:
        lines = f.readlines()

    variadic_functions = []
    already_extracted = set()

    # Look for "mut args: ..." pattern in function signatures
    for i, line in enumerate(lines):
        if 'mut args: ...' in line:
            # Find the start of the function (look backwards for 'fn ')
            for j in range(i, -1, -1):
                if ' fn ' in lines[j] or 'unsafe extern "C" fn' in lines[j]:
                    # Skip if already extracted
                    if j in already_extracted:
                        break

                    func_start = j
                    func_code, attr_start, func_end = extract_function(lines, func_start)
                    already_extracted.add(j)

                    # Extract function name
                    func_match = re.search(r'fn\s+(\w+)\s*\(', func_code)
                    if func_match:
                        func_name = func_match.group(1)
                        variadic_functions.append({
                            'name': func_name,
                            'code': func_code,
                            'start_line': attr_start,
                            'end_line': func_end,
                            'line_range': (attr_start, func_end)
                        })
                    break

    return variadic_functions

def append_to_printf_c_variadic(funcs, variadic_target_file):
    """Append variadic functions to printf_c_variadic.rs"""
    with open(variadic_target_file, 'a') as f:
        f.write('\n\n')
        for func in funcs:
            f.write(func['code'])
            f.write('\n\n')

def remove_from_source(source_file, funcs):
    """Remove variadic functions from source file."""
    with open(source_file, 'r') as f:
        lines = f.readlines()

    # Sort by line number in descending order to remove from bottom up
    sorted_funcs = sorted(funcs, key=lambda x: x['end_line'], reverse=True)

    for func in sorted_funcs:
        start = func['line_range'][0]
        end = func['line_range'][1]
        # Remove lines from start to end (inclusive)
        del lines[start:end]

    with open(source_file, 'w') as f:
        f.writelines(lines)

def add_re_exports(source_file, funcs, module_name):
    """Add re-export statements to source file."""
    # All variadic functions are consolidated in crate::src::printf_c_variadic
    var_path = 'crate::src::printf_c_variadic'

    re_exports = '\n// Re-export variadic functions from printf_c_variadic module\n'
    for func in funcs:
        re_exports += f'pub use {var_path}::{func["name"]};\n'

    # Append re-exports to the end of the file
    with open(source_file, 'a') as f:
        f.write(re_exports)

def process_file(source_file, variadic_target_file, module_path):
    """Process a single source file to consolidate variadic functions."""
    print(f"\nProcessing {source_file}...")

    funcs = find_variadic_functions(source_file)

    if not funcs:
        print(f"  No variadic functions found.")
        return

    print(f"  Found {len(funcs)} variadic functions:")
    for func in funcs:
        print(f"    - {func['name']}")

    # Append to printf_c_variadic.rs
    print(f"  Appending to {variadic_target_file}...")
    append_to_printf_c_variadic(funcs, variadic_target_file)

    # Remove from source file
    print(f"  Removing from {source_file}...")
    remove_from_source(source_file, funcs)

    # Add re-exports
    print(f"  Adding re-exports to {source_file}...")
    add_re_exports(source_file, funcs, module_path)

    print(f"  Done!")

def main():
    workspace = Path('/workspace')
    variadic_target = workspace / 'src' / 'printf_c_variadic.rs'

    # List of files to process in order
    files_to_process = [
        (workspace / 'src' / 'src' / 'main.rs', 'src::main'),
        (workspace / 'src' / 'src' / 'build.rs', 'src::build'),
        (workspace / 'src' / 'src' / 'json.rs', 'src::json'),
        (workspace / 'src' / 'src' / 'vtab.rs', 'src::vtab'),
        (workspace / 'src' / 'src' / 'date.rs', 'src::date'),
        (workspace / 'src' / 'src' / 'func.rs', 'src::func'),
        (workspace / 'src' / 'src' / 'btree.rs', 'src::btree'),
        (workspace / 'src' / 'ext' / 'session' / 'sqlite3session.rs', 'ext::session::sqlite3session'),
        (workspace / 'src' / 'ext' / 'rtree' / 'rtree.rs', 'ext::rtree::rtree'),
        (workspace / 'src' / 'ext' / 'fts3' / 'fts3.rs', 'ext::fts3::fts3'),
        (workspace / 'src' / 'fts5.rs', 'fts5'),
    ]

    for source_file, module_name in files_to_process:
        if source_file.exists():
            process_file(str(source_file), str(variadic_target), module_name)
        else:
            print(f"\nSkipping {source_file} - file not found")

if __name__ == '__main__':
    main()
