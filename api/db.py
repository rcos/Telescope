import os
import asyncpg

pool = None


async def get_pool():
    global pool
    if pool is None:
        pool = await asyncpg.create_pool(
            os.environ["POSTGRES_DSN"], min_size=5, max_size=5)
    return pool


async def get_db():
    pool = await get_pool()
    async with pool.acquire() as connection:
        yield connection
