from api.utils import insert_item_query, update_item_query
from typing import Any, List, Dict, Optional
from asyncpg import Connection
from pypika import Query, Table

users_t = Table("users")
user_acc_t = Table("user_accounts")


async def fetch_users(conn: Connection, filter: Dict[str, Any]) -> List[Dict]:
    query = Query.from_(users_t) \
        .select("*") \
        .orderby(users_t.username)

    for key, value in filter.items():
        if value is None:
            continue
        query = query.where(users_t[key] == value)
    return await conn.fetch(str(query))


async def fetch_user_accounts(conn: Connection, username: str) -> List[Dict]:
    query = Query.from_(user_acc_t) \
        .select("*").where(user_acc_t.username == username).orderby(user_acc_t.type)
    return await conn.fetch(str(query))
