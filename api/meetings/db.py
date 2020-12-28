from api.db import ARRAY_ANY
from typing import Any, List, Dict, Optional
from asyncpg import Connection
from pypika import Query, Table
from pypika.terms import Parameter

meet_t = Table("meetings")


async def fetch_meetings(conn: Connection, filter: Dict[str, Any]) -> List[Dict]:
    query = Query.from_(meet_t) \
        .select("*") \
        .orderby(meet_t.start_date_time)

    for key, value in filter.items():
        if value is None:
            continue
        query = query.where(value == meet_t[key])

    return await conn.fetch(str(query))


async def insert_meeting(conn: Connection, meeting: Dict[str, Any]) -> Dict:
    query = Query \
        .into(meet_t) \
        .columns(*meeting.keys()) \
        .insert(*meeting.values())
    print(str(query))
    return await conn.fetchrow(str(query) + " RETURNING *")


async def fetch_project(conn: Connection, project_id: str) -> Optional[Dict]:
    query = Query.from_(proj_t) \
        .select("*") \
        .where(proj_t.project_id == project_id)
    return await conn.fetchrow(str(query))


async def delete_project(conn: Connection, project_id: str) -> Optional[Dict]:
    query = Query.from_(proj_t) \
        .where(proj_t.project_id == project_id) \
        .delete()
    return await conn.fetchrow(str(query) + " RETURNING *")
