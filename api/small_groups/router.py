from api.utils import filter_dict
from typing import List, Optional
from asyncpg.connection import Connection
from fastapi import APIRouter
from fastapi.param_functions import Depends, Query
from starlette.exceptions import HTTPException
from api.db import get_db
from . import schemas, db

router = APIRouter(
    prefix="/small_groups",
    tags=["small_groups"],
)


@router.get("/", response_model=List[schemas.SmallGroupOut])
async def list_small_groups(
        semester_id: Optional[str] = Query(None),
        title: Optional[str] = Query(None),
        location: Optional[str] = Query(None),
        conn: Connection = Depends(get_db)):
    return await db.fetch_small_groups(conn, filter_dict(locals(), ["semester_id", "title", "location"]))


@router.post("/", response_model=schemas.SmallGroupOut)
async def create_small_group(small_group: schemas.SmallGroupCreate):
    raise HTTPException(status_code=501)


@router.get("/{small_group_id}", response_model=schemas.SmallGroupOut)
async def get_small_group(small_group_id: int, conn: Connection = Depends(get_db)):
    small_group = await db.fetch_small_group(conn, small_group_id)
    if small_group is None:
        raise HTTPException(status_code=404, detail="Small group not found")
    return small_group


@router.put("/{small_group_id}", response_model=schemas.SmallGroupOut)
async def update_small_group(small_group_id: int, small_group: schemas.SmallGroupIn):
    raise HTTPException(status_code=501)


@router.delete("/{small_group_id}", response_model=schemas.SmallGroupOut)
async def delete_small_group(small_group_id: int, conn: Connection = Depends(get_db)):
    deleted_small_group = await db.delete_small_group(conn, small_group_id)
    if deleted_small_group is None:
        raise HTTPException(status_code=404, detail="Small group not found")
    return deleted_small_group


@router.post("/{small_group_id}/mentors", tags=["small_groups", "mentors"], response_model=schemas.SmallGroupOut)
async def add_mentors_to_small_group(
        small_group_id: int,
        mentor_usernames: List[str] = Query(...),
        conn: Connection = Depends(get_db)):
    return await db.add_small_group_mentors(conn, small_group_id, mentor_usernames)


@router.delete("/{small_group_id}/mentors", tags=["small_groups", "mentors"], response_model=schemas.SmallGroupOut)
async def remove_mentors_from_small_group(small_group_id: int, mentor_usernames: Optional[List[str]] = Query(None)):
    raise HTTPException(status_code=501)


@router.post("/{small_group_id}/projects", tags=["small_groups", "projects"], response_model=schemas.SmallGroupOut)
async def add_projects_to_small_group(small_group_id: int, project_ids: List[str] = Query(...)):
    raise HTTPException(status_code=501)


@router.delete("/{small_group_id}/projects", tags=["small_groups", "projects"], response_model=schemas.SmallGroupOut)
async def remove_projects_from_small_group(small_group_id: int, project_ids: Optional[List[str]] = Query(None)):
    raise HTTPException(status_code=501)
