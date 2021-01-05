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


async def fetch_user(conn: Connection, username: str) -> Optional[Dict]:
    query = Query.from_(users_t) \
        .select("*") \
        .where(users_t.username == username)
    return await conn.fetchrow(str(query))


async def upsert_user(conn: Connection, username: str, user_dict: Dict[str, Any]) -> Dict:
    # Does user exist?
    user = await fetch_user(conn, username)
    primary_keys = {"username": username}
    if user:
        query = update_item_query(users_t, primary_keys, user_dict)
    else:
        query = insert_item_query(users_t, primary_keys, user_dict)

    return await conn.fetchrow(str(query) + " RETURNING *")


async def delete_user(conn: Connection, username: str) -> Optional[Dict]:
    query = Query.from_(users_t).where(users_t.username == username).delete()
    return await conn.fetchrow(str(query) + " RETURNING *")


async def fetch_user_accounts(conn: Connection, username: str) -> List[Dict]:
    query = Query.from_(user_acc_t) \
        .select("*").where(user_acc_t.username == username).orderby(user_acc_t.type)
    return await conn.fetch(str(query))
