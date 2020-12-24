from typing import Any, List, Dict, Optional
from asyncpg import Connection
from pydantic.fields import Field
from pypika import Query, Table

enr_t = Table("enrollments")


async def fetch_enrollments(db: Connection, semester_id: str, filter: Dict[str, Any]) -> List[Dict]:
    query = Query.from_(enr_t) \
        .select("*") \
        .where(enr_t.semester_id == semester_id) \
        .orderby(enr_t.semester_id) \
        .orderby(enr_t.username)

    # Apply queries
    for key, value in filter.items():
        if value is not None:
            query = query.where(enr_t[key] == value)

    return await db.fetch(str(query))


async def fetch_enrollment(db: Connection, semester_id: str, username: str) -> Dict:
    query = Query.from_(enr_t) \
        .select("*").where(enr_t.semester_id == semester_id).where(enr_t.username == username)
    return await db.fetchrow(str(query))
