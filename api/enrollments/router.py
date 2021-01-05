from api.utils import delete_item, fetch_item, filter_dict, upsert_item
from asyncpg.connection import Connection
from typing import List, Optional
from fastapi import APIRouter, HTTPException
from fastapi.param_functions import Depends, Query
from api.db import get_db
from . import db, schemas

router = APIRouter(
    prefix="/enrollments",
    tags=["enrollments"],
)


@router.get("/", response_model=List[schemas.EnrollmentOut], summary="List all semester enrollments")
async def list_enrollments(semester_id: Optional[str] = Query(None, example="202101"),
                           username: Optional[str] = Query(None, example=None),
                           project_id: Optional[int] = Query(
                               None, example=None),
                           is_project_lead: Optional[bool] = Query(
                               None, example=None),
                           is_coordinator: Optional[bool] = Query(
                               None, example=None),
                           credits_min: Optional[int] = Query(
                               None, example=None),
                           credits_max: Optional[int] = Query(
                               None, example=None),
                           is_for_pay: Optional[bool] = Query(
                               None, example=None),
                           conn: Connection = Depends(get_db)):
    """
    List all enrollments meeting the specified filters. Each filter is added as an AND operation, so setting `semester_id='202101'`, `is_project_lead=true`, and `project_id=1` will return the project leads for project 1 in the Spring 2021 semester.

    Find all enrolled students taking RCOS for credit with `credits_min=1` and find all students taking RCOS for experience with `credits_max=0, is_for_pay=false`.
    """

    return await db.fetch_enrollments(conn, filter_dict(locals(), ["semester_id", "username", "project_id", "is_project_lead", "is_coordinator", "credits_min", "credits_max", "is_for_pay"]))


@router.get("/{semester_id}/{username}", response_model=schemas.EnrollmentOut, responses={404: {"description": "Not found"}})
async def get_enrollment(semester_id: str, username: str, conn: Connection = Depends(get_db)):
    if semester_id is None:
        raise HTTPException(status_code=501)

    enrollment = await fetch_item(conn, "enrollments", {"semester_id": semester_id, "username": username})
    if enrollment is None:
        raise HTTPException(status_code=404, detail="Enrollment not found")
    return enrollment


@router.put("/{semester_id}/{username}", response_model=schemas.EnrollmentOut, responses={404: {"description": "Not found"}})
async def create_or_update_enrollment(semester_id: str, username: str, enrollment: schemas.EnrollmentIn, conn: Connection = Depends(get_db)):
    updated_enrollment_dict = await upsert_item(conn, "enrollments", {"semester_id": semester_id, "username": username}, enrollment.dict(exclude_unset=True))
    return updated_enrollment_dict


@router.delete("/{semester_id}/{username}", response_model=schemas.EnrollmentOut, responses={404: {"description": "Not found"}})
async def delete_enrollment(semester_id: str, username: str, conn: Connection = Depends(get_db)):
    deleted_enrollment = await delete_item(conn, "enrollments", {"semester_id": semester_id, "username": username})
    if deleted_enrollment is None:
        raise HTTPException(status_code=404, detail="Enrollment not found")
    return deleted_enrollment
