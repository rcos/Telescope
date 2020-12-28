from api.utils import filter_dict
from asyncpg.connection import Connection
from typing import List, Optional
from fastapi import APIRouter, HTTPException
from fastapi.param_functions import Depends, Query
from api.db import get_db
from . import schemas, db

router = APIRouter(
    prefix="/projects",
    tags=["projects"],
)


@router.get("/", response_model=List[schemas.ProjectOut])
async def list_projects(
        semester_id: Optional[str] = Query(None),
        title: Optional[str] = Query(None),
        languages: Optional[List[str]] = Query(None),
        stack: Optional[List[str]] = Query(None),
        repository_url: Optional[str] = Query(None),
        conn: Connection = Depends(get_db)):
    if semester_id is not None:
        raise HTTPException(status_code=501)

    return await db.fetch_projects(conn, filter_dict(locals(), ["title", "languages", "stack", "repository_url"]))


@router.post("/", response_model=schemas.ProjectOut)
async def create_project(project: schemas.ProjectIn):
    raise HTTPException(status_code=501)


@router.get("/{project_id}", response_model=schemas.ProjectOut, responses={404: {"description": "Not found"}})
async def get_project(project_id: str, conn: Connection = Depends(get_db)):
    project = await db.fetch_project(conn, project_id)
    if project is None:
        raise HTTPException(status_code=404, detail="Project not found")
    return project


@router.put("/{project_id}", response_model=schemas.ProjectOut, responses={404: {"description": "Not found"}})
async def update_project(project_id: str, project: schemas.ProjectIn, conn: Connection = Depends(get_db)):
    raise HTTPException(status_code=501)


@router.delete("/{project_id}", response_model=Optional[schemas.ProjectOut], responses={404: {"description": "Not found"}})
async def delete_project(project_id: str):
    raise HTTPException(status_code=501)
