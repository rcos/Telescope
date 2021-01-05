from typing import Any, Dict, List, Tuple

from pypika.queries import Query, QueryBuilder, Table


def filter_dict(locals: Dict[str, Any], keys: List[str]):
    return dict((key, locals[key])
                for key in keys)


def update_item_query(table: Table, primary_keys: Dict[str, Any], update_dict: Dict[str, Any]) -> QueryBuilder:
    query = Query.update(table)

    # Prevent accidental total update
    if len(primary_keys) == 0:
        raise Exception("Tried to creat update query with no primary keys.")

    # Find by primary keys
    for pk, value in primary_keys.items():
        query = query.where(table[pk] == value)

    # Update values present in update_dict
    for col, value in update_dict.items():
        query = query.set(table[col], value)

    return query


def insert_item_query(table: Table, primary_keys: Dict[str, Any], item_dict: Dict[str, Any]):
    query = Query.into(table).columns(
        *primary_keys.keys(), *item_dict.keys()).insert(*primary_keys.values(), *item_dict.values())
    return query
