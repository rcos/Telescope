from typing import List, Optional

from pypika.enums import Order

from api.db import get_db
from api.security import requires_api_key
from api.utils import delete_item, fetch_item, filter_dict, insert_item, list_items, update_item
from asyncpg.connection import Connection
from fastapi import APIRouter, HTTPException
from fastapi.param_functions import Depends, Query

from . import db, schemas

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

    return await list_items(
        conn,
        "projects",
        filter_dict(locals(), ["title", "languages",
                               "stack", "repository_url"]),
        order_by=[("created_at", Order.desc)])


@router.post("/", response_model=schemas.ProjectOut,
             dependencies=[Depends(requires_api_key)])
async def create_project(project: schemas.ProjectIn,
                         conn: Connection = Depends(get_db)):
    return await insert_item(conn, "projects", {}, project.dict(exclude_unset=True))


@router.get("/{project_id}", response_model=schemas.ProjectOut, responses={404: {"description": "Not found"}})
async def get_project(project_id: str, conn: Connection = Depends(get_db)):
    project = await fetch_item(conn, "projects", {"project_id": project_id})
    if project is None:
        raise HTTPException(status_code=404, detail="Project not found")
    return project


@router.put("/{project_id}", response_model=schemas.ProjectOut, responses={404: {"description": "Not found"}},
            dependencies=[Depends(requires_api_key)])
async def update_project(project_id: str, project: schemas.ProjectIn, conn: Connection = Depends(get_db)):
    updated_project = await update_item(conn, "projects", {"project_id": project_id}, project.dict(exclude_unset=True))
    if updated_project is None:
        raise HTTPException(status_code=404, detail="Project not found")
    return updated_project


@router.delete("/{project_id}", response_model=Optional[schemas.ProjectOut], responses={404: {"description": "Not found"}},
               dependencies=[Depends(requires_api_key)])
async def delete_project(project_id: str, conn: Connection = Depends(get_db)):
    deleted_project = await delete_item(conn, "projects", {"project_id": project_id})
    if deleted_project is None:
        raise HTTPException(status_code=404, detail="Project not found")
    return deleted_project
