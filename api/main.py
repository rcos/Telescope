from fastapi import FastAPI
from fastapi.exceptions import HTTPException
from fastapi.middleware.cors import CORSMiddleware
from fastapi.param_functions import Depends
from fastapi.routing import APIRouter

from api import VERSION
from api.db import get_pool
from api.security import get_api_key

from . import enrollments, meetings, projects, semesters, small_groups, users

app = FastAPI(title="RCOS API", version=VERSION,
              description="Repository available at [rcos/rcos-api](https://github.com/rcos/rcos-api)")

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

# Protect most routers entirely with an API key, but allow some to manage their own protected routes
api.include_router(semesters.router,
                   dependencies=[Depends(get_api_key)])
api.include_router(users.router,
                   dependencies=[Depends(get_api_key)])
api.include_router(enrollments.router,
                   dependencies=[Depends(get_api_key)])
api.include_router(projects.router)
api.include_router(small_groups.router,
                   dependencies=[Depends(get_api_key)])
api.include_router(meetings.router)

app.include_router(api)
