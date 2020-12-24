from pypika import Query, Table, Field
from pypika.queries import QueryBuilder

users_t = Table("users")

users_q: QueryBuilder = Query.from_(users_t).select("*").orderby("username")


async def fetch_users(db):
    return await db.fetch(str(users_q))
