from api.utils import insert_item_query, update_item_query
from typing import Any, List, Dict, Optional
from asyncpg import Connection
from pydantic.fields import Field
from pypika import Query, Table

enr_t = Table("enrollments")


async def fetch_enrollments(conn: Connection, filter: Dict[str, Any]) -> List[Dict]:
    query = Query.from_(enr_t) \
        .select("*") \
        .orderby(enr_t.semester_id) \
        .orderby(enr_t.username)

    # Apply queries
    for key, value in filter.items():
        if value is None:
            continue

        if key == "credits_min":
            query = query.where(enr_t.credits >= value)
        elif key == "credits_max":
            query = query.where(enr_t.credits <= value)
        else:
            query = query.where(enr_t[key] == value)
    return await conn.fetch(str(query))


async def fetch_enrollment(conn: Connection, semester_id: str, username: str) -> Dict:
    query = Query.from_(enr_t) \
        .select("*") \
        .where(enr_t.semester_id == semester_id) \
        .where(enr_t.username == username)
    return await conn.fetchrow(str(query))


async def upsert_enrollment(conn: Connection, semester_id: str, username: str, enrollment_dict: Dict[str, Any]):
    enrollment = await fetch_enrollment(conn, semester_id, username)
    primary_keys = {"semester_id": semester_id, "username": username}
    if enrollment:
        query = update_item_query(
            enr_t, primary_keys, enrollment_dict)
    else:
        query = insert_item_query(enr_t, primary_keys, enrollment_dict)

    return await conn.fetchrow(str(query) + " RETURNING *")
