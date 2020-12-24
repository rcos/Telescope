from typing import List
from asyncpg.connection import Connection
from fastapi import APIRouter
from fastapi.exceptions import HTTPException
from fastapi.param_functions import Depends

from api.schemas.users import UserAccount, UserIn, UserOut
from api.db import get_db
from api.db.users import fetch_user, fetch_user_accounts, fetch_users

router = APIRouter(
    prefix="/users",
    tags=["users"],
)


@router.get("/", response_model=List[UserOut], summary="List all users")
async def list_users(db: Connection = Depends(get_db)):
    return await fetch_users(db)


@router.get("/{username}", response_model=UserOut, summary="Get specific user", response_description="Get a specific user's profile with information that doesn't depend upon a specific semester.")
async def get_user(username: str, db: Connection = Depends(get_db)):
    user = await fetch_user(db, username)
    if user is None:
        raise HTTPException(status_code=404, detail="User not found")
    return user


@router.get("/{username}/accounts", response_model=List[UserAccount], summary="Get specific user's accounts", response_description="Get a user's connected social media and Git platform accounts.")
async def get_user(username: str, db: Connection = Depends(get_db)):
    return await fetch_user_accounts(db, username)
