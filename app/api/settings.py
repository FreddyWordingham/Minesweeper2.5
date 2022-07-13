from fastapi.templating import Jinja2Templates
import os


SECRET_KEY = "**SECRET**"

APP_DIR = "app"
TEMPLATES_DIR = os.path.join(APP_DIR, "templates")
PLUGIN_DIR = os.path.join(TEMPLATES_DIR, "plugins")
STATIC_DIR = os.path.join(APP_DIR, "static")
CAMPAIGNS_DIR = os.path.join(STATIC_DIR, "campaigns")
DATABASE_PATH = os.path.join(STATIC_DIR, "database.json")


templates = Jinja2Templates(directory=TEMPLATES_DIR)
