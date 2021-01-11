from enum import Enum
from typing import Any, Dict, List, Optional, Tuple, Union
from asyncpg.connection import Connection

from pypika import PostgreSQLQuery as Query
from pypika.queries import QueryBuilder, Table
from pypika.terms import Array


def separate_col_op(pk: str):
    parts = pk.split('__', 1)
    if len(parts) == 1:
        return (pk, None)
    else:
        return parts


def filter_dict(locals: Dict[str, Any], keys: List[str]):
    return dict((key, locals[key])
                for key in keys)


def apply_where(query: QueryBuilder, table: Table, keys: Dict[str, Any], ignore_none: bool = False):
    for pk, value in keys.items():
        if value is None and ignore_none:
            continue

        col, op = separate_col_op(pk)
        if op == 'gte':
            query = query.where(table[col] >= value)
        elif op == 'lte':
            query = query.where(table[col] <= value)
        elif op == 'gt':
            query = query.where(table[col] > value)
        elif op == 'lt':
            query = query.where(table[col] < value)
        elif op == 'in':
            query = query.where(table[col].isin(value))
        else:
            query = query.where(table[pk] == value)
    return query


def update_item_query(table: Table, primary_keys: Dict[str, Any], update_dict: Dict[str, Any]) -> QueryBuilder:
    query = Query.update(table)

    # Prevent accidental total update
    if len(primary_keys) == 0:
        raise Exception("Tried to creat update query with no primary keys.")

    # Find by primary keys
    query = apply_where(query, table, primary_keys)

    # Update values present in update_dict
    for col, value in update_dict.items():
        if isinstance(value, list):
            value = Array(*value)
        query = query.set(table[col], value)

    return query


def insert_item_query(table: Table, primary_keys: Dict[str, Any], item_dict: Dict[str, Any]):
    query = Query.into(table).columns(
        *primary_keys.keys(), *item_dict.keys()).insert(*primary_keys.values(), *item_dict.values())
    return query


async def execute_and_return(conn: Connection, query: Query):
    return await conn.fetchrow(str(query) + " RETURNING *")


async def list_items(conn: Connection, table: Union[Table, str], search_keys: Dict[str, any] = dict(), order_by: List[str] = []):
    if isinstance(table, str):
        table = Table(table)
    query = Query.from_(table) \
        .select("*")

    for col, order in order_by:
        query = query.orderby(col, order=order)

    query = apply_where(query, table, search_keys, ignore_none=True)
    print(query)
    return await conn.fetch(str(query))


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


async def update_item(conn: Connection, table_name: str, primary_keys: Dict[str, Any], item_dict: Dict[str, Any]):
    table = Table(table_name)
    query = update_item_query(table, primary_keys, item_dict)
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
