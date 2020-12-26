from api.utils import filter_dict
from api.schemas.enrollments import EnrollmentIn, EnrollmentOut
from api.db import get_db
from asyncpg.connection import Connection
from api.db.enrollments import fetch_enrollment, fetch_enrollments
from typing import List, Optional
from fastapi import APIRouter, HTTPException
from fastapi.param_functions import Depends, Query

router = APIRouter(
    prefix="/enrollments",
    tags=["enrollments"],
)


@router.get("/", response_model=List[EnrollmentOut], summary="List all semester enrollments")
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
                           db: Connection = Depends(get_db)):
    """
    List all enrollments meeting the specified filters. Each filter is added as an AND operation, so setting `semester_id='202101'`, `is_project_lead=true`, and `project_id=1` will return the project leads for project 1 in the Spring 2021 semester.

    Find all enrolled students taking RCOS for credit with `credits_min=1` and find all students taking RCOS for experience with `credits_max=0, is_for_pay=false`.
    """

    return await fetch_enrollments(db, filter_dict(locals(), ["semester_id", "username", "project_id", "is_project_lead", "is_coordinator", "credits_min", "credits_max", "is_for_pay"]))


@router.get("/{semester_id}/{username}", response_model=EnrollmentOut, responses={404: {"description": "Not found"}})
async def get_enrollment(semester_id: str, username: str, db: Connection = Depends(get_db)):
    if semester_id is None:
        raise HTTPException(status_code=501)

    enrollment = await fetch_enrollment(db, semester_id, username)
    if enrollment is None:
        raise HTTPException(status_code=404, detail="Enrollment not found")
    return enrollment


@router.put("/{semester_id}/{username}", response_model=EnrollmentOut, responses={404: {"description": "Not found"}})
async def update_enrollment(semester_id: str, username: str, updated_enrollment: EnrollmentIn, db: Connection = Depends(get_db)):
    raise HTTPException(status_code=501)


@router.delete("/{semester_id}/{username}", response_model=EnrollmentOut, responses={404: {"description": "Not found"}})
async def delete_enrollment(semester_id: str, username: str, db: Connection = Depends(get_db)):
    raise HTTPException(status_code=501)
