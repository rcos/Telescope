from os import stat
from fastapi.exceptions import HTTPException
from typing import List
from asyncpg.connection import Connection
from fastapi import APIRouter
from fastapi.param_functions import Query, Depends
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
    semester = await db.fetch_semester(conn, semester_id)
    if semester is None:
        raise HTTPException(status_code=404, detail="Semester not found")
    return semester
