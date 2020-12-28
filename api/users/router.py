from asyncpg.exceptions import ForeignKeyViolationError
from api.utils import filter_dict
from typing import List, Optional
from asyncpg.connection import Connection
from fastapi import APIRouter
from fastapi.exceptions import HTTPException
from fastapi.param_functions import Depends, Query

from api.db import get_db
from . import db, schemas

router = APIRouter(
    prefix="/users",
    tags=["users"],
)


@router.get("/", response_model=List[schemas.UserOut], summary="List all users")
async def list_users(
        is_rpi: Optional[bool] = Query(None, example=True),
        is_faculty: Optional[bool] = Query(None, example=False),
        timezone: Optional[str] = Query(None, example=None),
        conn: Connection = Depends(get_db)):
    return await db.fetch_users(conn, filter_dict(locals(), ["is_rpi", "is_faculty", "timezone"]))


@router.get("/{username}", response_model=schemas.UserOut, summary="Get specific user", response_description="Get a specific user's profile with information that doesn't depend upon a specific semester.", responses={404: {"description": "Not found"}})
async def get_user(username: str, conn: Connection = Depends(get_db)):
    user = await db.fetch_user(conn, username)
    if user is None:
        raise HTTPException(status_code=404, detail="User not found")
    return user


@router.put("/{username}", response_model=schemas.UserOut, summary="Create or update a user")
async def create_or_update_user(username: str, user: schemas.UserIn, conn: Connection = Depends(get_db)):
    updated_user_dict = await db.upsert_user(conn, username, user.dict(exclude_unset=True))
    return updated_user_dict


@router.delete("/{username}", response_model=schemas.UserOut, summary="Delete a specific user", responses={404: {"description": "Not found"}})
async def delete_user(username: str, conn: Connection = Depends(get_db)):
    # TODO: Cascade deletions... probably should be done on DB end
    deleted_user = await db.delete_user(conn, username)
    if deleted_user is None:
        raise HTTPException(status_code=404, detail="User not found")
    return deleted_user


@router.get("/{username}/accounts", response_model=List[schemas.UserAccount], summary="Get specific user's accounts", response_description="Get a user's connected social media and Git platform accounts.", responses={404: {"description": "Not found"}})
async def get_user(username: str, conn: Connection = Depends(get_db)):
    user = await db.fetch_user(conn, username)
    if user is None:
        raise HTTPException(status_code=404, detail="User not found")
    return await db.fetch_user_accounts(conn, username)
