from typing import List, Dict
from asyncpg import Connection
from pypika import Query, Table

enr_t = Table("enrollments")


async def fetch_enrollments(db: Connection, semester_id: str) -> List[Dict]:
    query = Query.from_(enr_t) \
        .select("*").orderby(enr_t.semester_id).orderby(enr_t.username)
    return await db.fetch(str(query))


async def fetch_enrollment(db: Connection, semester_id: str, username: str) -> Dict:
    query = Query.from_(enr_t) \
        .select("*").where(enr_t.semester_id == semester_id).where(enr_t.username == username)
    return await db.fetchrow(str(query))
