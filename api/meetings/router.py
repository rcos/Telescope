from typing import Optional
from fastapi import APIRouter
from fastapi.exceptions import HTTPException
from fastapi.param_functions import Query

router = APIRouter(
    prefix="/meetings",
    tags=["meetings"],
)


@router.get("/")
async def list_meetings(semester_id: Optional[str] = Query(None)):
    raise HTTPException(status_code=501)


@router.get("/{meeting_id}")
async def get_meeting(meeting_id: str):
    raise HTTPException(status_code=501)


@router.put("/{meeting_id}")
async def update_meeting(meeting_id: str):
    raise HTTPException(status_code=501)


@router.delete("/{meeting_id}")
async def delete_meeting(meeting_id: str):
    raise HTTPException(status_code=501)


@router.get("/{meeting_id}/attendances")
async def get_meeting_attendances(meeting_id: str):
    raise HTTPException(status_code=501)


@router.post("/{meeting_id}/attendances")
async def create_meeting_attendance(meeting_id: str, username: str = Query(..., example="manp")):
    raise HTTPException(status_code=501)
