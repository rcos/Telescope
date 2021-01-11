from typing import List, Optional

from api.db import get_db
from api.utils import (delete_item, fetch_item, filter_dict, list_items,
                       upsert_item)
from asyncpg.connection import Connection
from asyncpg.exceptions import ForeignKeyViolationError
from fastapi import APIRouter
from fastapi.exceptions import HTTPException
from fastapi.param_functions import Depends, Query
from pypika.enums import Order

from . import schemas

router = APIRouter(
    prefix="/users",
    tags=["users"],
)


@router.get("/", response_model=List[schemas.UserOut], summary="List all users")
async def list_users(
        is_rpi: Optional[bool] = Query(None, example=True),
        is_faculty: Optional[bool] = Query(None, example=False),
        graduation_year: Optional[int] = Query(None, example=2022),
        timezone: Optional[str] = Query(None, example=None),
        conn: Connection = Depends(get_db)):
    return await list_items(conn,
                            "users",
                            filter_dict(
                                locals(), ["is_rpi", "is_faculty", "graduation_year", "timezone"]),
                            order_by=[("username", Order.asc)])


@router.get("/{username}", response_model=schemas.UserOut, summary="Get specific user", response_description="Get a specific user's profile with information that doesn't depend upon a specific semester.", responses={404: {"description": "Not found"}})
async def get_user(username: str, conn: Connection = Depends(get_db)):
    user = await fetch_item(conn, "users", {"username": username})
    if user is None:
        raise HTTPException(status_code=404, detail="User not found")
    return user


@router.put("/{username}", response_model=schemas.UserOut, summary="Create or update a user")
async def create_or_update_user(username: str, user: schemas.UserIn, conn: Connection = Depends(get_db)):
    updated_user_dict = await upsert_item(conn, "users", {"username": username}, user.dict(exclude_unset=True))
    return updated_user_dict


@router.delete("/{username}", response_model=schemas.UserOut, summary="Delete a specific user", responses={404: {"description": "Not found"}})
async def delete_user(username: str, conn: Connection = Depends(get_db)):
    # TODO: Cascade deletions... probably should be done on DB end
    deleted_user = await delete_item(conn, "users", {"username": username})
    if deleted_user is None:
        raise HTTPException(status_code=404, detail="User not found")
    return deleted_user


@router.get("/{username}/accounts", response_model=List[schemas.UserAccountOut], summary="Get specific user's accounts", response_description="List of user's connected social media and Git platform accounts.", responses={404: {"description": "Not found"}})
async def list_user_accounts(username: str, conn: Connection = Depends(get_db)):
    user = await fetch_item(conn, "users", {"username": username})
    if user is None:
        raise HTTPException(status_code=404, detail="User not found")
    return await list_items(conn, "user_accounts", {"username": username})


@router.get("/{username}/accounts/{type}", response_model=schemas.UserAccountOut, summary="Get specific user's platform account", responses={404: {"description": "Not found"}})
async def get_user_account(username: str, type: str, conn: Connection = Depends(get_db)):
    user_account = await fetch_item(conn, "user_accounts", {"username": username, "type": type})
    if user_account is None:
        raise HTTPException(status_code=404, detail="User account not found")
    return user_account


@router.put("/{username}/accounts/{type}", response_model=schemas.UserAccountOut, summary="Create or update a user account")
async def create_or_update_user_account(username: str, type: str, user_account: schemas.UserAccountIn, conn: Connection = Depends(get_db)):
    updated_user_account_dict = await upsert_item(conn, "user_accounts", {"username": username, "type": type}, user_account.dict(exclude_unset=True))
    return updated_user_account_dict


@router.delete("/{username}/accounts/{type}", response_model=schemas.UserAccountOut, summary="Delete a specific user account", responses={404: {"description": "Not found"}})
async def delete_user_account(username: str, type: str, conn: Connection = Depends(get_db)):
    # TODO: Cascade deletions... probably should be done on DB end
    deleted_user_account = await delete_item(conn, "user_accounts", {"username": username, "type": type})
    if deleted_user_account is None:
        raise HTTPException(status_code=404, detail="User account not found")
    return deleted_user_account
