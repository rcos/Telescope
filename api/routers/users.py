from asyncpg.exceptions import ForeignKeyViolationError
from api.utils import filter_dict
from typing import List, Optional
from asyncpg.connection import Connection
from fastapi import APIRouter
from fastapi.exceptions import HTTPException
from fastapi.param_functions import Depends, Query

from api.schemas.users import UserAccount, UserIn, UserOut
from api.db import get_db
from api.db.users import fetch_user, fetch_user_accounts, fetch_users, upsert_user, delete_user as delete_user_from_db

router = APIRouter(
    prefix="/users",
    tags=["users"],
)


@router.get("/", response_model=List[UserOut], summary="List all users")
async def list_users(
        is_rpi: Optional[bool] = Query(None, example=True),
        is_faculty: Optional[bool] = Query(None, example=False),
        timezone: Optional[str] = Query(None, example=None),
        db: Connection = Depends(get_db)):
    return await fetch_users(db, filter_dict(locals(), ["is_rpi", "is_faculty", "timezone"]))


@router.get("/{username}", response_model=UserOut, summary="Get specific user", response_description="Get a specific user's profile with information that doesn't depend upon a specific semester.", responses={404: {"description": "Not found"}})
async def get_user(username: str, db: Connection = Depends(get_db)):
    user = await fetch_user(db, username)
    if user is None:
        raise HTTPException(status_code=404, detail="User not found")
    return user


@router.put("/{username}", response_model=UserOut, summary="Create or update a user")
async def create_or_update_user(username: str, user: UserIn, db: Connection = Depends(get_db)):
    updated_user_dict = await upsert_user(db, username, user.dict(exclude_unset=True))
    return updated_user_dict


@router.delete("/{username}", response_model=UserOut, summary="Delete a specific user", responses={404: {"description": "Not found"}})
async def delete_user(username: str, db: Connection = Depends(get_db)):
    # TODO: Cascade deletions... probably should be done on DB end
    deleted_user = await delete_user_from_db(db, username)
    if deleted_user is None:
        raise HTTPException(status_code=404, detail="User not found")
    return deleted_user


@router.get("/{username}/accounts", response_model=List[UserAccount], summary="Get specific user's accounts", response_description="Get a user's connected social media and Git platform accounts.", responses={404: {"description": "Not found"}})
async def get_user(username: str, db: Connection = Depends(get_db)):
    user = await fetch_user(db, username)
    if user is None:
        raise HTTPException(status_code=404, detail="User not found")
    return await fetch_user_accounts(db, username)
