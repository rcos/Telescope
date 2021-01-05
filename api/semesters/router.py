from api.utils import fetch_item, upsert_item
from os import stat
from fastapi.exceptions import HTTPException
from typing import List
from asyncpg.connection import Connection
from fastapi import APIRouter
from fastapi.param_functions import Query, Depends
from starlette import responses
from api.db import get_db
from . import schemas, db

router = APIRouter(
    prefix="/semesters",
    tags=["semesters"],
)


@router.get("/", response_model=List[schemas.SemesterOut])
async def list_semesters(conn: Connection = Depends(get_db)):
    return await db.fetch_semesters(conn)


@router.get("/{semester_id}", responses={404: {"description": "Not found"}})
async def get_semester(semester_id: str, conn: Connection = Depends(get_db)):
    semester = await fetch_item(conn, "semesters", {"semester_id": semester_id})
    if semester is None:
        raise HTTPException(status_code=404, detail="Semester not found")
    return semester


@router.put("/{semester_id}")
async def create_or_update_semester(semester_id: str, semester: schemas.SemesterIn, conn: Connection = Depends(get_db)):
    updated_semester_dict = await upsert_item(conn, "semesters", {"semester_id": semester_id}, semester.dict(exclude_unset=True))
    return updated_semester_dict
