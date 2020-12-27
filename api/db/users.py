from typing import Any, List, Dict, Optional
from asyncpg import Connection
from pypika import Query, Table, Field
from pypika.queries import QueryBuilder
from pypika.terms import Values

users_t = Table("users")
user_acc_t = Table("user_accounts")


async def fetch_users(db: Connection, filter: Dict[str, Any]) -> List[Dict]:
    query = Query.from_(users_t) \
        .select("*") \
        .orderby(users_t.username)

    for key, value in filter.items():
        if value is None:
            continue
        query = query.where(users_t[key] == value)
    return await db.fetch(str(query))


async def fetch_user(db: Connection, username: str) -> Optional[Dict]:
    query = Query.from_(users_t) \
        .select("*") \
        .where(users_t.username == username)
    return await db.fetchrow(str(query))


async def upsert_user(db: Connection, username: str, user_dict: Dict[str, Any]) -> Dict:
    # Does user exist?
    user = await fetch_user(db, username)
    if user:
        query = Query.update(users_t).where(users_t.username == username)
        for col, value in user_dict.items():
            query = query.set(users_t[col], value)
    else:
        query = Query.into(users_t).columns(
            "username", *user_dict.keys()).insert(username, *user_dict.values())

    return await db.fetchrow(str(query) + " RETURNING *")


async def delete_user(db: Connection, username: str) -> Optional[Dict]:
    query = Query.from_(users_t).where(users_t.username == username).delete()
    return await db.fetchrow(str(query) + " RETURNING *")


async def fetch_user_accounts(db: Connection, username: str) -> List[Dict]:
    query = Query.from_(user_acc_t) \
        .select("*").where(user_acc_t.username == username).orderby(user_acc_t.type)
    return await db.fetch(str(query))
