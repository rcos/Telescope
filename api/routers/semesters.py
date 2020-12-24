from typing import Optional
from fastapi import APIRouter
from fastapi.param_functions import Query

router = APIRouter(
    prefix="/semesters",
    tags=["semesters"],
)


@router.get("/")
async def list_semesters():
    return []


@router.get("/{semester_id}")
async def get_semester(semester_id: str):
    return {
        "semester_id": semester_id,
    }
