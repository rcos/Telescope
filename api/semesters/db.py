from typing import List, Dict
from asyncpg import Connection
from pypika import Query, Table, Field
from pypika.queries import QueryBuilder

sems_t = Table("semesters")


async def fetch_semesters(conn: Connection) -> List[Dict]:
    query = Query.from_(sems_t).select("*").orderby(sems_t.semester_id)
    return await conn.fetch(str(query))


async def fetch_semester(conn: Connection, semester_id: str) -> Dict:
    query = Query.from_(sems_t) \
        .select("*").where(sems_t.semester_id == semester_id)
    return await conn.fetchrow(str(query))
