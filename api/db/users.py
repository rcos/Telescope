from typing import Any, List, Dict
from asyncpg import Connection
from pypika import Query, Table, Field
from pypika.queries import QueryBuilder

users_t = Table("users")
user_acc_t = Table("user_accounts")


async def fetch_users(db: Connection, filter: Dict[str, Any]) -> List[Dict]:
    query = Query.from_(users_t).select("*").orderby(users_t.username)

    for key, value in filter.items():
        if value is None:
            continue
        query = query.where(users_t[key] == value)
    return await db.fetch(str(query))


async def fetch_user(db: Connection, username: str) -> Dict:
    query = Query.from_(users_t) \
        .select("*").where(users_t.username == username)
    return await db.fetchrow(str(query))


async def fetch_user_accounts(db: Connection, username: str) -> List[Dict]:
    query = Query.from_(user_acc_t) \
        .select("*").where(user_acc_t.username == username).orderby(user_acc_t.type)
    return await db.fetch(str(query))
