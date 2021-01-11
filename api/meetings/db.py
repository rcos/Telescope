from typing import Any, List, Dict, Optional
from asyncpg import Connection
from pypika import PostgreSQLQuery as Query, Table

meet_t = Table("meetings")


async def fetch_meetings(conn: Connection, filter: Dict[str, Any]) -> List[Dict]:
    query = Query.from_(meet_t) \
        .select("*") \
        .orderby(meet_t.start_date_time)

    for key, value in filter.items():
        if value is None:
            continue

        if key == "start_date_time_before":
            query = query.where(meet_t.start_date_time < value)
        elif key == "start_date_time_after":
            query = query.where(meet_t.start_date_time > value)
        elif key == "end_date_time_before":
            query = query.where(meet_t.end_date_time < value)
        elif key == "end_date_time_after":
            query = query.where(meet_t.end_date_time > value)
        else:
            query = query.where(value == meet_t[key])

    return await conn.fetch(str(query))
