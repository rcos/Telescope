from api.utils import filter_dict
from typing import Any, List, Dict, Optional
from asyncpg import Connection
from pypika import PostgreSQLQuery as Query, Table

sg_t = Table("small_groups")
sg_mentors_t = Table("small_group_mentors")
sg_projects_t = Table("small_group_projects")


async def fetch_small_groups(conn: Connection, filter: Dict[str, Any]) -> List[Dict]:
    query = Query.from_(sg_t) \
        .select(sg_t.star,
                sg_projects_t.project_id,
                sg_mentors_t.username.as_("mentor_username")) \
        .left_join(sg_mentors_t) \
        .on(sg_mentors_t.small_group_id == sg_t.small_group_id) \
        .left_join(sg_projects_t) \
        .on(sg_projects_t.small_group_id == sg_t.small_group_id) \
        .orderby(sg_t.semester_id)

    # Apply filters
    for key, value in filter.items():
        if value is None:
            continue
        query = query.where(sg_t[key] == value)

    results = await conn.fetch(str(query))

    # We join small group mentors and projects and then aggregate for each small group
    small_groups = dict()
    for result in results:
        if result["small_group_id"] not in small_groups:
            small_groups[result["small_group_id"]] = {
                **filter_dict(result, ["small_group_id", "semester_id", "title", "location"]),
                "mentor_usernames": set(),
                "project_ids": set()
            }

        if result["project_id"]:
            small_groups[result["small_group_id"]
                         ]["project_ids"].add(result["project_id"])
        if result["mentor_username"]:
            small_groups[result["small_group_id"]]["mentor_usernames"].add(
                result["mentor_username"])
    return list(small_groups.values())


async def fetch_small_group(conn: Connection, small_group_id: int) -> Optional[Dict]:
    query = Query.from_(sg_t) \
        .select(sg_t.star,
                sg_projects_t.project_id,
                sg_mentors_t.username.as_("mentor_username")) \
        .left_join(sg_mentors_t) \
        .on(sg_mentors_t.small_group_id == sg_t.small_group_id) \
        .left_join(sg_projects_t) \
        .on(sg_projects_t.small_group_id == sg_t.small_group_id) \
        .where(sg_t.small_group_id == small_group_id)
    results = await conn.fetch(str(query))

    if len(results) == 0:
        return None

    small_group = {
        **filter_dict(results[0], ["small_group_id", "semester_id", "title", "location"]),
        "mentor_usernames": set(),
        "project_ids": set()
    }

    for result in results:
        if result["project_id"]:
            small_group["project_ids"].add(result["project_id"])
        if result["mentor_username"]:
            small_group["mentor_usernames"].add(
                result["mentor_username"])

    return small_group


async def delete_small_group(conn: Connection, small_group_id: int) -> Optional[Dict]:
    query = Query \
        .from_(sg_t) \
        .where(sg_t.small_group_id == small_group_id) \
        .delete()
    return await conn.fetchrow(str(query) + " RETURNING *")


#######################
# SMALL GROUP MENTORS #
#######################

async def fetch_small_group_mentors(conn: Connection, small_group_id: int) -> List[str]:
    query = Query \
        .from_(sg_mentors_t) \
        .select(sg_mentors_t.username) \
        .where(sg_mentors_t.small_group_id == small_group_id)

    return list(map(lambda record: record["username"], await conn.fetch(str(query))))


async def add_small_group_mentors(conn: Connection, small_group_id: int, mentor_usernames: List[str]) -> List[str]:
    small_group = await fetch_small_group(conn, small_group_id)
    if small_group is None:
        return None

    query = Query.into(sg_mentors_t)

    # Only add new mentors to avoid conflicts
    adding = False  # Tracks whether we are adding at least one mentor
    for mu in mentor_usernames:
        if mu not in small_group["mentor_usernames"]:
            query = query.insert((small_group_id, mu))
            adding = True

    # Only execute INSERT if there is at least one new mentor, otherwise SQL would be malformed
    if adding:
        await conn.fetchrow(str(query) + " RETURNING *")

    # Just refetch the small group and have it figure out the new list of mentors
    return await fetch_small_group(conn, small_group_id)
