from typing import List, Optional
from fastapi import APIRouter
from fastapi.param_functions import Query
from starlette.exceptions import HTTPException
from . import schemas, db

router = APIRouter(
    prefix="/small_groups",
    tags=["small_groups"],
)


@router.get("/", response_model=List[schemas.SmallGroupOut])
async def list_small_groups(
        semester_id: Optional[str] = Query(None),
        title: Optional[str] = Query(None),
        location: Optional[str] = Query(None)):
    raise HTTPException(status_code=501)


@router.post("/", response_model=schemas.SmallGroupOut)
async def create_small_group(small_group: schemas.SmallGroupCreate):
    raise HTTPException(status_code=501)


@router.get("/{small_group_id}", response_model=schemas.SmallGroupOut)
async def get_small_group(small_group_id: str):
    raise HTTPException(status_code=501)


@router.put("/{small_group_id}", response_model=schemas.SmallGroupOut)
async def update_small_group(small_group_id: str, small_group: schemas.SmallGroupIn):
    raise HTTPException(status_code=501)


@router.delete("/{small_group_id}", response_model=schemas.SmallGroupOut)
async def get_small_group(small_group_id: str):
    raise HTTPException(status_code=501)


@router.post("/{small_group_id}/mentors", tags=["small_groups", "mentors"], response_model=schemas.SmallGroupOut)
async def add_mentors_to_small_group(small_group_id: str, mentor_usernames: List[str] = Query(...)):
    raise HTTPException(status_code=501)


@router.delete("/{small_group_id}/mentors", tags=["small_groups", "mentors"], response_model=schemas.SmallGroupOut)
async def remove_mentors_from_small_group(small_group_id: str, mentor_usernames: Optional[List[str]] = Query(None)):
    raise HTTPException(status_code=501)


@router.post("/{small_group_id}/projects", tags=["small_groups", "projects"], response_model=schemas.SmallGroupOut)
async def add_projects_to_small_group(small_group_id: str, project_ids: List[str] = Query(...)):
    raise HTTPException(status_code=501)


@router.delete("/{small_group_id}/projects", tags=["small_groups", "projects"], response_model=schemas.SmallGroupOut)
async def remove_projects_from_small_group(small_group_id: str, project_ids: Optional[List[str]] = Query(None)):
    raise HTTPException(status_code=501)
