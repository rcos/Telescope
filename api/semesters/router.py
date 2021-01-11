import datetime
from typing import List

from api.db import get_db
from api.utils import (delete_item, fetch_item, filter_dict, list_items,
                       upsert_item)
from asyncpg.connection import Connection
from fastapi import APIRouter
from fastapi.exceptions import HTTPException
from fastapi.param_functions import Depends, Query
from pypika import Order

from . import schemas

router = APIRouter(
    prefix="/semesters",
    tags=["semesters"],
)


@router.get("/", response_model=List[schemas.SemesterOut])
async def list_semesters(
        start_date__gte: datetime.date = Query(None),
        start_date__lte: datetime.date = Query(None),
        end_date__gte: datetime.date = Query(None),
        end_date__lte: datetime.date = Query(None),
        conn: Connection = Depends(get_db)):
    return await list_items(
        conn,
        "semesters",
        filter_dict(locals(), [
                    "start_date__gte", "start_date__lte", "end_date__gte", "end_date__lte"]),
        order_by=[("semester_id", Order.asc)])


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


@router.delete("/{semester_id}", response_model=schemas.SemesterOut, summary="Delete a specific semester", responses={404: {"description": "Not found"}})
async def delete_semester(semester_id: str, conn: Connection = Depends(get_db)):
    # TODO: Cascade deletions... probably should be done on DB end
    deleted_semester = await delete_item(conn, "semesters", {"semester_id": semester_id})
    if deleted_semester is None:
        raise HTTPException(status_code=404, detail="Semester not found")
    return deleted_semester
