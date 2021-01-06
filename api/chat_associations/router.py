from api.utils import delete_item, fetch_item, filter_dict, upsert_item
from asyncpg.connection import Connection
from typing import List, Optional
from fastapi import APIRouter, HTTPException
from fastapi.param_functions import Depends, Query
from api.db import get_db
from . import db, schemas

router = APIRouter(
    prefix="/chat_associations",
    tags=["chat"],
)


@router.get("/", response_model=List[schemas.ChatAssociationOut], summary="List all semester enrollments")
async def list_enrollments(
        source_type: Optional[str] = Query(None, example="discord_role"),
        target_type: Optional[str] = Query(None, example="project"),
        source_id: Optional[str] = Query(None),
        conn: Connection = Depends(get_db)):

    return await db.fetch_chat_associations(conn, filter_dict(locals(), ["source_type", "target_type", "source_id"]))


# @router.get("/{semester_id}/{username}", response_model=schemas.ChatAssociationOut, responses={404: {"description": "Not found"}})
# async def get_enrollment(semester_id: str, username: str, conn: Connection = Depends(get_db)):
#     enrollment = await fetch_item(conn, "enrollments", {"semester_id": semester_id, "username": username})
#     if enrollment is None:
#         raise HTTPException(
#             status_code=404, detail="ChatAssociation not found")
#     return enrollment


# @router.put("/{semester_id}/{username}", response_model=schemas.ChatAssociationOut, responses={404: {"description": "Not found"}})
# async def create_or_update_enrollment(semester_id: str, username: str, enrollment: schemas.ChatAssociationIn, conn: Connection = Depends(get_db)):
#     updated_enrollment_dict = await upsert_item(conn, "enrollments", {"semester_id": semester_id, "username": username}, enrollment.dict(exclude_unset=True))
#     return updated_enrollment_dict


# @router.delete("/{semester_id}/{username}", response_model=schemas.ChatAssociationOut, responses={404: {"description": "Not found"}})
# async def delete_enrollment(semester_id: str, username: str, conn: Connection = Depends(get_db)):
#     deleted_enrollment = await delete_item(conn, "enrollments", {"semester_id": semester_id, "username": username})
#     if deleted_enrollment is None:
#         raise HTTPException(
#             status_code=404, detail="ChatAssociation not found")
#     return deleted_enrollment
