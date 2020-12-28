from typing import Any, List, Dict, Optional
from asyncpg import Connection
from pypika import Query, Table

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


async def fetch_meeting(conn: Connection, meeting_id: int) -> Optional[Dict]:
    query = Query.from_(meet_t) \
        .select("*") \
        .where(meet_t.meeting_id == meeting_id)
    return await conn.fetchrow(str(query))


async def update_meeting(conn: Connection, meeting_id: int, meeting_dict: Dict[str, Any]) -> Optional[Dict]:
    query = Query \
        .update(meet_t) \
        .where(meet_t.meeting_id == meeting_id)
    for col, value in meeting_dict.items():
        query = query.set(meet_t[col], value)

    return await conn.fetchrow(str(query) + " RETURNING *")


async def delete_meeting(conn: Connection, meeting_id: int) -> Optional[Dict]:
    query = Query.from_(meet_t) \
        .where(meet_t.meeting_id == meeting_id) \
        .delete()
    return await conn.fetchrow(str(query) + " RETURNING *")
