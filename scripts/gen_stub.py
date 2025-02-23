#!/usr/bin/env python3

import os
import sys
import inspect
import importlib


def generate_stub_file(package_name: str, output_path: str):
    """Generates a .pyi stub file for a package.

    The .pyi stub file contains function signatures and docstrings to enable
    IDE autocompletion and type hints.

    Args:
        package_name: Name of the package to process (e.g. 'kand')
        output_path: Target path for the .pyi file (must be writable)
    """
    # Dynamically import the package
    pkg = importlib.import_module(package_name)

    # Get all top-level names that don't start and end with double underscores
    public_names = [
        name for name in dir(pkg) if not (name.startswith("__") and name.endswith("__"))
    ]

    # Generate stub file content
    pyi_lines = []
    pyi_lines.append(f"# Auto-generated stub file for {package_name}")
    pyi_lines.append('"""')
    pyi_lines.append(
        "Type hints and function signatures stub file for IDE autocompletion."
    )
    pyi_lines.append(
        "Auto-generated to avoid manual maintenance. Can be enhanced with more precise type annotations."
    )
    pyi_lines.append('"""')
    pyi_lines.append("")

    # Output signatures and docs for functions/builtins
    for name in public_names:
        attr = getattr(pkg, name)
        # Check if callable (function, builtin, etc)
        if inspect.isfunction(attr) or inspect.isbuiltin(attr):
            # Get function signature (may fail for builtins)
            try:
                sig = str(inspect.signature(attr))
            except ValueError:
                sig = "(...)"

            # Get function docstring
            doc = inspect.getdoc(attr) or ""
            doc_lines = doc.splitlines() if doc else []

            # Write declaration
            pyi_lines.append(f"def {name}{sig}:")

            if doc_lines:
                # Write docstring as proper Python docstring
                pyi_lines.append('    """')
                for line in doc_lines:
                    pyi_lines.append(f"    {line}")
                pyi_lines.append('    """')
            else:
                pyi_lines.append('    """No docstring available."""')

            # Add ellipsis for function body
            pyi_lines.append("    ...")
            pyi_lines.append("")

    # Create output directory if it doesn't exist
    os.makedirs(os.path.dirname(output_path), exist_ok=True)

    # Write the stub file
    with open(output_path, "w", encoding="utf-8") as f:
        f.write("\n".join(pyi_lines))
    print(f"Generated: {output_path}")


if __name__ == "__main__":
    """Example usage:

    To generate a stub file for the kand package:
        python gen_stub.py kand path/to/output.pyi
    """
    if len(sys.argv) < 3:
        print("Usage: python gen_stub.py <package_name> <output_path>")
        print("Example: python gen_stub.py kand python/kand/_kand.pyi")
        sys.exit(1)

    pkg_name = sys.argv[1]
    output_path = sys.argv[2]
    generate_stub_file(pkg_name, output_path)
