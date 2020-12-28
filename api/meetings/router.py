from api.utils import filter_dict
from typing import List, Optional
from asyncpg.connection import Connection
from fastapi import APIRouter
from fastapi.exceptions import HTTPException
from fastapi.param_functions import Depends, Query
from api.db import get_db
from . import schemas, db

router = APIRouter(
    prefix="/meetings",
    tags=["meetings"],
)


@router.get("/", response_model=List[schemas.MeetingOut])
async def list_meetings(
        semester_id: Optional[str] = Query(None),
        meeting_type: Optional[str] = Query(None),
        host_username: Optional[str] = Query(None),
        is_public: Optional[bool] = Query(None),
        location: Optional[str] = Query(None),
        conn: Connection = Depends(get_db)):
    return await db.fetch_meetings(conn, filter_dict(locals(), ["semester_id", "meeting_type", "host_username", "is_public", "location"]))


@router.get("/{meeting_id}")
async def get_meeting(meeting_id: str, conn: Connection = Depends(get_db)):
    raise HTTPException(status_code=501)


@router.put("/{meeting_id}")
async def update_meeting(meeting_id: str, conn: Connection = Depends(get_db)):
    raise HTTPException(status_code=501)


@router.delete("/{meeting_id}")
async def delete_meeting(meeting_id: str, conn: Connection = Depends(get_db)):
    raise HTTPException(status_code=501)


@router.get("/{meeting_id}/attendances")
async def get_meeting_attendances(meeting_id: str, conn: Connection = Depends(get_db)):
    raise HTTPException(status_code=501)


@router.post("/{meeting_id}/attendances")
async def create_meeting_attendance(meeting_id: str, username: str = Query(..., example="manp")):
    raise HTTPException(status_code=501)
