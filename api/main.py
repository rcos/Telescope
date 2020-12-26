from fastapi.routing import APIRouter
from api.db import get_pool
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from .routers import users, enrollments, projects, semesters, small_groups, meetings

app = FastAPI()

# Allow requests from all origins
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.on_event("startup")
async def connect_db():
    await get_pool()


@app.on_event("shutdown")
async def connect_db():
    pool = await get_pool()
    await pool.close()

api = APIRouter(prefix="/api/v1")

api.include_router(semesters.router)
api.include_router(users.router)
api.include_router(enrollments.router)
api.include_router(projects.router)
api.include_router(small_groups.router)
api.include_router(meetings.router)

app.include_router(api)
