from api.schemas.enrollments import EnrollmentOut
from os import stat
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
async def list_enrollments(semester_id: str,
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
                           db: Connection = Depends(get_db)):
    filter = {
        "project_id": project_id,
        "is_project_lead": is_project_lead,
        "is_coordinator": is_coordinator,
        "credits_min": credits_min,
        "credits_max": credits_max,
    }
    return await fetch_enrollments(db, semester_id, filter)


@router.get("/{username}", response_model=EnrollmentOut, responses={404: {"description": "Not found"}})
async def get_enrollment(username: str, semester_id: Optional[str] = Query(None), db: Connection = Depends(get_db)):
    if semester_id is None:
        raise HTTPException(status_code=501)

    enrollment = await fetch_enrollment(db, semester_id, username)
    if enrollment is None:
        raise HTTPException(status_code=404, detail="Enrollment not found")
    return enrollment
