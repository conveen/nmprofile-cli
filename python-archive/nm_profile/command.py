## Copyright (c) 2021 conveen
##
## Permission is hereby granted, free of charge, to any person obtaining a copy
## of this software and associated documentation files (the "Software"), to deal
## in the Software without restriction, including without limitation the rights
## to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
## copies of the Software, and to permit persons to whom the Software is
## furnished to do so, subject to the following conditions:
##
## The above copyright notice and this permission notice shall be included in all
## copies or substantial portions of the Software.
##
## THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
## IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
## FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
## AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
## LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
## OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
## SOFTWARE.

import subprocess
import typing


__author__ = "conveen"


def run_command(command: typing.List[str], capture_output: bool = True, exit_codes: typing.List[int] = [0]) -> bytes:
    """Run command via Bash shell and return output to stdout."""
    result = subprocess.run(["/usr/bin/bash", "-c"] + command, capture_output=capture_output)
    if result.returncode not in exit_codes:
        stderr = result.stderr.strip().decode("utf8")
        raise RuntimeError(f"Command '{' '.join(command)}' exited with code {result.returncode} ({stderr})")
    return (result.stdout or "").strip()
