

class MainConfigs:
    def __init__(self) -> None:
        self.warns: list[str] = []
        self.deaf = None

    @property
    def deaf(self):
        try:
            return
        except AttributeError:
            return
