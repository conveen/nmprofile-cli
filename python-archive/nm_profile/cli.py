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

from argparse import Action, ArgumentParser, Namespace
from os import environ as ENV
import typing


class EnvVariableDefaultAction(Action):
    """argparse action to get arguments from environment variables.
    Used with ArgumentParser.add_argument and the action kwarg like the following:

    parser.add_argument("-s", "--source",
                        action=EnvVariableDefaultAction,
                        help="Help message",
                        metavar="ENV_VARIABLE_NAME",
                        dest="variable_name")

    where the metavar kwarg specifies the environment variable to read from.
    By default, the action will read from the specified environment variable.
    If the environment variable isn't present, it will use the default if provided,
    otherwise read from the command line arguments.
    """

    def __init__(self, *args, **kwargs) -> None:
        metavar: typing.Optional[str] = kwargs.get("metavar")

        if not metavar:
            raise ValueError("must specify environment variable name as metavar")
        if metavar in ENV and ENV[metavar]:
            kwargs["default"] = ENV[metavar]
            kwargs["required"] = False
        super().__init__(*args, **kwargs)

    def __call__(
        self,
        _: ArgumentParser,
        namespace: Namespace,
        values: typing.Any,
        __: typing.Optional[str] = None,
    ) -> None:
        setattr(namespace, self.dest, values)
