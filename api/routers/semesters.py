from os import stat
from fastapi.exceptions import HTTPException
from api.schemas.semesters import SemesterOut
from api.db import get_db
from typing import List, Optional
from asyncpg.connection import Connection
from fastapi import APIRouter
from fastapi.param_functions import Query, Depends
from api.db.semesters import fetch_semesters, fetch_semester

router = APIRouter(
    prefix="/semesters",
    tags=["semesters"],
)


@router.get("/", response_model=List[SemesterOut])
async def list_semesters(db: Connection = Depends(get_db)):
    return await fetch_semesters(db)


@router.get("/{semester_id}", responses={404: {"description": "Not found"}})
async def get_semester(semester_id: str, db: Connection = Depends(get_db)):
    semester = await fetch_semester(db, semester_id)
    if semester is None:
        raise HTTPException(status_code=404, detail="Semester not found")
    return semester
