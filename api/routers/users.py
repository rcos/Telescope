from typing import List
from api.schemas.users import User
from api.db import get_db
import api.db.users
from fastapi import APIRouter
from fastapi.exceptions import HTTPException
from fastapi.param_functions import Depends

router = APIRouter(
    prefix="/users",
    tags=["users"],
)


@router.get("/", response_model=List[User])
async def list_users(db=Depends(get_db)):
    users = await api.db.users.fetch_users(db)
    return users


@ router.get("/{username}")
async def get_user(username: str):
    raise HTTPException(status_code=501)
