from typing import Any, Dict, List, Tuple, Union
from asyncpg.connection import Connection

from pypika.queries import Query, QueryBuilder, Table


def filter_dict(locals: Dict[str, Any], keys: List[str]):
    return dict((key, locals[key])
                for key in keys)


def apply_where(query: QueryBuilder, table: Table, keys: Dict[str, Any]):
    for pk, value in keys.items():
        query = query.where(table[pk] == value)
    return query


def update_item_query(table: Table, primary_keys: Dict[str, Any], update_dict: Dict[str, Any]) -> QueryBuilder:
    query = Query.update(table)

    # Prevent accidental total update
    if len(primary_keys) == 0:
        raise Exception("Tried to creat update query with no primary keys.")

    # Find by primary keys
    query = apply_where(query, table, primary_keys.items())

    # Update values present in update_dict
    for col, value in update_dict.items():
        query = query.set(table[col], value)

    return query


def insert_item_query(table: Table, primary_keys: Dict[str, Any], item_dict: Dict[str, Any]):
    query = Query.into(table).columns(
        *primary_keys.keys(), *item_dict.keys()).insert(*primary_keys.values(), *item_dict.values())
    return query


async def execute_and_return(conn: Connection, query: Query):
    return await conn.fetchrow(str(query) + " RETURNING *")


async def fetch_item(conn: Connection, table: Union[Table, str], primary_keys: Dict[str, Any]):
    # If table is table name, get table from it
    if isinstance(table, str):
        table = Table(table)

    query = Query.from_(table) \
        .select("*").limit(1)

    query = apply_where(query, table, primary_keys)

    return await conn.fetchrow(str(query))


async def insert_item(conn: Connection, table_name: str, primary_keys: Dict[str, Any], item_dict: Dict[str, Any]):
    table = Table(table_name)
    query = insert_item_query(table, primary_keys, item_dict)
    return await execute_and_return(conn, query)


async def upsert_item(conn: Connection, table_name: str, primary_keys: Dict[str, Any], item_dict: Dict[str, Any]):
    table = Table(table_name)
    item = await fetch_item(conn, table, primary_keys)

    if item:
        query = update_item_query(
            table, primary_keys, item_dict)
    else:
        query = insert_item_query(table, primary_keys, item_dict)

    return await execute_and_return(conn, query)


async def delete_item(conn: Connection, table_name: str, primary_keys: Dict[str, Any]):
    table = Table(table_name)
    query = Query.from_(table).delete()
    query = apply_where(query, table, primary_keys)

    return await execute_and_return(conn, query)
