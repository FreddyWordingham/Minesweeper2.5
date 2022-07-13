from fastapi import APIRouter, Depends
from fastapi_login import LoginManager
from fastapi_login.exceptions import InvalidCredentialsException
from fastapi.security import OAuth2PasswordRequestForm

from . import settings


auth_route = APIRouter()
manager = LoginManager(settings.SECRET_KEY, token_url="/auth/token")


@manager.user_loader()
def load_user(username):
    from .database import DB

    return DB["users"].get(username)


@auth_route.post("/token")
def login(data: OAuth2PasswordRequestForm = Depends()):
    username = data.username
    password = data.password

    user = load_user(username)
    if not user:
        raise InvalidCredentialsException
    elif password != user["password"]:
        raise InvalidCredentialsException

    access_token = manager.create_access_token(data=dict(sub=username))
    return {"access_token": access_token, "token_type": "bearer"}
