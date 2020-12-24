from fastapi import FastAPI
from .routers import users, enrollments, projects, semesters, small_groups, meetings

app = FastAPI()


@app.get("/")
async def root():
    return {"message": "Hello World"}

app.include_router(semesters.router)
app.include_router(users.router)
app.include_router(enrollments.router)
app.include_router(projects.router)
app.include_router(small_groups.router)
app.include_router(meetings.router)
