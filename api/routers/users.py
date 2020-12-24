from api.db import get_db
from fastapi import APIRouter
from fastapi.exceptions import HTTPException
from fastapi.param_functions import Depends

router = APIRouter(
    prefix="/users",
    tags=["users"],
)


@router.get("/")
async def list_users(db=Depends(get_db)):
    users = await db.fetch("SELECT 1 AS num")
    return users


@ router.get("/{username}")
async def get_user(username: str):
    raise HTTPException(status_code=501)
