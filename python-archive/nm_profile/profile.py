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

import importlib
import logging
from pathlib import Path
import subprocess
import sys
from time import sleep
import typing


__author__ = "conveen"
logger = logging.getLogger(__name__)


class BaseProfile:
    """Base network profile class.

    All profile classes *must* extend this class and implement the up and down methods.
    When joining a network via a profile the up method is called, and when disconnecting the down method is called.
    The NAME class variable is the name of the profile used for connecting via the CLI,
    and DESCRIPTION should be a friendly, one-line explanation of the profile (i.e., "Connect to corporate OpenVPN").
    Profiles are loaded dynamically at runtime, and will be validated as child classes of this class.
    """
    __slots__ = ()

    NAME: str = ""
    DESCRIPTION: str = ""

    def run_command(
        self,
        command: typing.List[str],
        exit_codes: typing.List[int] = [0],
        **kwargs,
    ) -> bytes:
        """Run command via Bash shell and return output to stdout."""
        result = subprocess.run(["/usr/bin/bash", "-c"] + command, **kwargs)

        if result.returncode not in exit_codes:
            stderr = result.stderr.strip().decode("utf8")
            raise RuntimeError(f"Command '{' '.join(command)}' exited with code {result.returncode} ({stderr})")

        return (result.stdout or "").strip()

    def up(self) -> None:
        """Connect to this network profile."""
        raise NotImplementedError()

    def down(self) -> None:
        """Disconnect from this network profile."""
        raise NotImplementedError()


def load_profiles(profile_directory_path: Path) -> typing.Dict[str, typing.Type[typing.BaseProfile]]:
    """Load profile class mapping from directory of Python modules."""
    profile_directory_str = str(profile_directory_path)
    if not profile_directory_path.is_dir():
        raise FileNotFoundError(profile_directory_str)

    # If profile directory not in sys.path, invalidate the import caches and add it
    # See: https://docs.python.org/3/library/importlib.html#importlib.import_module
    if profile_directory_str not in sys.path:
        importlib.invalidate_caches()
        sys.path.insert(0, profile_directory_str)

    profiles: typing.Dict[str, typing.Type[typing.BaseProfile]] = {}
    for file_path in profile_directory_path.iterdir():
        if file_path.is_file() and file_path.suffix == ".py":
            try:
                module = importlib.import_module(f"{file_path.stem}")
                if not getattr(module, "Profile"):
                    logger.warning("Module {} does not have a profile class defined".format(file_path.stem))
                    continue
                profile_cls = module.Profile
                profile_name = profile_cls.NAME
