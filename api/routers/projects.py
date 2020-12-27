from api.utils import filter_dict
from api.db import get_db
from asyncpg.connection import Connection
import api.db.projects as project_db
from api.schemas.projects import ProjectOut
from typing import List, Optional
from fastapi import APIRouter, HTTPException
from fastapi.param_functions import Depends, Query

router = APIRouter(
    prefix="/projects",
    tags=["projects"],
)


@router.get("/", response_model=List[ProjectOut])
async def list_projects(
        semester_id: Optional[str] = Query(None),
        title: Optional[str] = Query(None),
        languages: Optional[List[str]] = Query(None),
        stack: Optional[List[str]] = Query(None),
        repository_url: Optional[str] = Query(None),
        db: Connection = Depends(get_db)):
    if semester_id is not None:
        raise HTTPException(status_code=501)

    return await project_db.fetch_projects(db, filter_dict(locals(), ["title", "languages", "stack", "repository_url"]))


@router.get("/{project_id}", response_model=ProjectOut, responses={404: {"description": "Not found"}})
async def get_project(project_id: str, db: Connection = Depends(get_db)):
    project = await project_db.fetch_project(db, project_id)
    if project is None:
        raise HTTPException(status_code=404, detail="Project not found")
    return project
