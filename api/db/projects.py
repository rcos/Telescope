from typing import List, Dict
from asyncpg import Connection
from pypika import Query, Table

proj_t = Table("projects")


async def fetch_projects(db: Connection) -> List[Dict]:
    query = Query.from_(proj_t).select("*").orderby(proj_t.created_at)
    return await db.fetch(str(query))


async def fetch_project(db: Connection, project_id: str) -> Dict:
    query = Query.from_(proj_t) \
        .select("*").where(proj_t.project_id == project_id)
    return await db.fetchrow(str(query))
