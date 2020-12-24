from fastapi import APIRouter
from fastapi.exceptions import HTTPException

router = APIRouter(
    prefix="/users",
    tags=["users"],
)


@router.get("/")
async def list_users():
    raise HTTPException(status_code=501)


@router.get("/{username}")
async def get_user(username: str):
    raise HTTPException(status_code=501)
