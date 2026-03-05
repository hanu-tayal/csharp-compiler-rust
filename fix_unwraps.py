#!/usr/bin/env python3
"""Fix all consume().unwrap() calls to use expect() instead."""

import os
import re

def fix_unwraps_in_file(filepath):
    """Replace consume().unwrap() with expect() in a file."""
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Pattern to match consume(...).unwrap()
    pattern = r'self\.parser\.consume\((.*?)\)\.unwrap\(\)'
    replacement = r'self.parser.expect(\1)'
    
    new_content = re.sub(pattern, replacement, content)
    
    # Also fix direct self.consume (in mod.rs)
    pattern2 = r'self\.consume\((.*?)\)\.unwrap\(\)'
    replacement2 = r'self.expect(\1)'
    
    new_content = re.sub(pattern2, replacement2, new_content)
    
    if new_content != content:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(new_content)
        print(f"Fixed {filepath}")
        return True
    return False

def main():
    parser_dir = r"C:\Users\hanut\rust-csharp-compiler\src\parser"
    
    fixed_count = 0
    for filename in os.listdir(parser_dir):
        if filename.endswith('.rs'):
            filepath = os.path.join(parser_dir, filename)
            if fix_unwraps_in_file(filepath):
                fixed_count += 1
    
    print(f"\nFixed {fixed_count} files")

if __name__ == "__main__":
    main()