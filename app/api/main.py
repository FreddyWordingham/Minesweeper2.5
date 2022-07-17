from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse
from fastapi.staticfiles import StaticFiles

from . import settings
from .auth import auth_route, manager


app = FastAPI()
app.mount("/static", StaticFiles(directory="app/static"), name="static")
app.include_router(auth_route, prefix="/auth")


@app.get("/", response_class=HTMLResponse)
async def homepage(request: Request):
    return settings.templates.TemplateResponse("pages/game.html", {"request": request})


@app.get("/login", response_class=HTMLResponse)
async def login(request: Request):
    return settings.templates.TemplateResponse("pages/login.html", {"request": request})
