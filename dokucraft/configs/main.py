import os
import logging
from typing import TypeVar

TV = TypeVar('TV')


class MainConfigs:
    def __init__(self) -> None:
        self.warns: list[str] = []
        self.defaults = None

    @property
    def defaults(self):
        try:
            return self._defaults.copy()
        except AttributeError as ae:
            return ae

    @defaults.setter
    def defaults(self, value):
        self._defaults = value
