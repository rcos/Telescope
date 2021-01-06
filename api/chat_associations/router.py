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


@router.get("/", response_model=List[schemas.ChatAssociationOut])
async def list_enrollments(
        source_type: Optional[schemas.Source] = Query(None, example="project"),
        target_type: Optional[schemas.Target] = Query(
            None, example="discord_role"),
        source_id: Optional[str] = Query(None),
        target_id: Optional[str] = Query(None),
        conn: Connection = Depends(get_db)):

    return await db.fetch_chat_associations(conn, filter_dict(locals(), ["source_type", "target_type", "source_id", "target_id"]))


@router.get("/{source_type}/{source_id}", response_model=schemas.ChatAssociationOut, responses={404: {"description": "Not found"}})
async def get_chat_association(source_type: schemas.Source, source_id: str, target_type: schemas.Target = Query(...), conn: Connection = Depends(get_db)):
    chat_association = await fetch_item(conn, "chat_associations", filter_dict(locals(), ["source_type", "source_id", "target_type"]))
    if chat_association is None:
        raise HTTPException(
            status_code=404, detail="Chat association not found")
    return chat_association


@router.put("/{source_type}/{source_id}", response_model=schemas.ChatAssociationOut, responses={404: {"description": "Not found"}})
async def create_or_update_chat_association(source_type: schemas.Source, source_id: str, chat_association: schemas.ChatAssociationIn, target_type: schemas.Target = Query(..., example="discord_role"), conn: Connection = Depends(get_db)):
    updated_chat_association_dict = await upsert_item(conn, "chat_associations", filter_dict(locals(), ["source_type", "source_id", "target_type"]), chat_association.dict(exclude_unset=True))
    return updated_chat_association_dict


@router.delete("/{source_type}/{source_id}", response_model=schemas.ChatAssociationOut, responses={404: {"description": "Not found"}})
async def delete_chat_association(source_type: schemas.Source, source_id: str, target_type: schemas.Target = Query(..., example="discord_role"), conn: Connection = Depends(get_db)):
    deleted_chat_association = await delete_item(conn, "chat_associations", filter_dict(locals(), ["source_type", "source_id", "target_type"]))
    if deleted_chat_association is None:
        raise HTTPException(
            status_code=404, detail="Chat association not found")
    return deleted_chat_association
