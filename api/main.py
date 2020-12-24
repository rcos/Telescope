from api.db import get_pool
from fastapi import FastAPI
from .routers import users, enrollments, projects, semesters, small_groups, meetings

app = FastAPI()


@app.on_event("startup")
async def connect_db():
    await get_pool()


@app.on_event("shutdown")
async def connect_db():
    pool = await get_pool()
    await pool.close()


@app.get("/")
async def root():
    return {"message": "Hello World"}

app.include_router(semesters.router)
app.include_router(users.router)
app.include_router(enrollments.router)
app.include_router(projects.router)
app.include_router(small_groups.router)
app.include_router(meetings.router)
