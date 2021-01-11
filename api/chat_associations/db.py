from api.utils import execute_and_return, insert_item_query, update_item_query
from typing import Any, List, Dict, Optional
from asyncpg import Connection
from pydantic.fields import Field
from pypika import PostgreSQLQuery as Query, Table

chat_t = Table("chat_associations")


async def fetch_chat_associations(conn: Connection, filter: Dict[str, Any]) -> List[Dict]:
    query = Query.from_(chat_t) \
        .select("*")

    # Apply queries
    for key, value in filter.items():
        if value is None:
            continue

        query = query.where(chat_t[key] == value)
    return await conn.fetch(str(query))
