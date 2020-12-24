from typing import Optional
from fastapi import APIRouter, HTTPException
from fastapi.param_functions import Query

router = APIRouter(
    prefix="/enrollments",
    tags=["enrollments"],
)


@router.get("/")
async def list_enrollments(semester_id: str):
    raise HTTPException(status_code=501)


@router.get("/{username}")
async def get_enrollment(username: str, semester_id: Optional[str] = Query(None)):
    raise HTTPException(status_code=501)
