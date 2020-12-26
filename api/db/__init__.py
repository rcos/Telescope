import os
import asyncpg
from pypika import CustomFunction

pool = None


async def get_pool():
    global pool
    if pool is None:
        pool = await asyncpg.create_pool(
            os.environ["POSTGRES_DSN"], min_size=4, max_size=4)
    return pool


async def get_db() -> asyncpg.Connection:
    pool = await get_pool()
    async with pool.acquire() as connection:
        yield connection

ARRAY_ANY = CustomFunction('ANY', ['column'])
