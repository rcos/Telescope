from typing import Optional
from fastapi import APIRouter, HTTPException
from fastapi.param_functions import Query

router = APIRouter(
    prefix="/projects",
    tags=["projects"],
)


@router.get("/")
async def list_projects(semester_id: Optional[str] = Query(None)):
    raise HTTPException(status_code=501)


@router.get("/{project_id}")
async def get_project(project_id: str, semester_id: Optional[str] = Query(None)):
    raise HTTPException(status_code=501)
