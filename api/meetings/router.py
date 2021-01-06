from typing import List, Optional

from api.db import get_db
from api.security import get_api_key, requires_api_key
from api.utils import filter_dict
from asyncpg.connection import Connection
from fastapi import APIRouter
from fastapi.exceptions import HTTPException
from fastapi.param_functions import Depends, Query

from . import db, schemas

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
        api_key=Depends(get_api_key),
        conn: Connection = Depends(get_db)):
    search = filter_dict(locals(), [
                         "semester_id", "meeting_type", "host_username", "is_public", "location"])

    # Only authenticated requests can fetch private meetings
    if api_key is None:
        search["is_public"] = True

    return await db.fetch_meetings(conn, search)


@router.post("/", response_model=schemas.MeetingOut,
             dependencies=[Depends(requires_api_key)])
async def create_meeting(meeting: schemas.MeetingIn, conn: Connection = Depends(get_db)):
    return await db.insert_meeting(conn, meeting.dict(exclude_unset=True))


@router.get("/{meeting_id}", response_model=schemas.MeetingOut, responses={404: {"description": "Not found"}})
async def get_meeting(meeting_id: str, api_key=Depends(get_api_key), conn: Connection = Depends(get_db)):
    meeting = await db.fetch_meeting(conn, meeting_id)
    if meeting is None:
        raise HTTPException(status_code=404, detail="Meeting not found")
    if api_key is None and not meeting["is_public"]:
        raise HTTPException(
            status_code=403, detail="You must use an API KEY to access private meetings.")
    return meeting


@router.put("/{meeting_id}", response_model=schemas.MeetingOut, responses={404: {"description": "Not found"}},
            dependencies=[Depends(requires_api_key)])
async def update_meeting(meeting_id: str, meeting: schemas.MeetingIn, conn: Connection = Depends(get_db)):
    updated_meeting = await db.update_meeting(conn, meeting_id, meeting.dict(exclude_unset=True))
    if updated_meeting is None:
        raise HTTPException(status_code=404, detail="Meeting not found")
    return updated_meeting


@router.delete("/{meeting_id}", response_model=schemas.MeetingOut, responses={404: {"description": "Not found"}},
               dependencies=[Depends(requires_api_key)])
async def delete_meeting(meeting_id: str, conn: Connection = Depends(get_db)):
    deleted_meeting = await db.delete_meeting(conn, meeting_id)
    if deleted_meeting is None:
        raise HTTPException(status_code=404, detail="Meeting not found")
    return deleted_meeting


@router.get("/{meeting_id}/attendances", responses={404: {"description": "Not found"}},
            dependencies=[Depends(requires_api_key)])
async def get_meeting_attendances(meeting_id: str, conn: Connection = Depends(get_db)):
    raise HTTPException(status_code=501)


@router.post("/{meeting_id}/attendances", responses={404: {"description": "Not found"}},
             dependencies=[Depends(requires_api_key)])
async def create_meeting_attendance(meeting_id: str, username: str = Query(..., example="manp")):
    raise HTTPException(status_code=501)


@router.delete("/{meeting_id}/attendances/{username}", responses={404: {"description": "Not found"}},
               dependencies=[Depends(requires_api_key)])
async def delete_meeting_attendance(meeting_id: str, username: str = Query(..., example="manp")):
    raise HTTPException(status_code=501)
