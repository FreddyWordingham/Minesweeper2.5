import atexit
from pathlib import Path
import json


from . import settings


DB = json.loads(Path(settings.DATABASE_PATH).read_text())


def exit_handler():
    Path(settings.DATABASE_PATH).write_text(json.dumps(DB, indent=4))


atexit.register(exit_handler)
