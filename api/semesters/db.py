from api.utils import execute_and_return, insert_item_query, update_item_query
from typing import Any, List, Dict
from asyncpg import Connection
from pypika import Query, Table, Field
from pypika.queries import QueryBuilder

sems_t = Table("semesters")


async def fetch_semesters(conn: Connection) -> List[Dict]:
    query = Query.from_(sems_t).select("*").orderby(sems_t.semester_id)
    return await conn.fetch(str(query))
