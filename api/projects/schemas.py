from typing import List, Optional
from asyncpg.types import Range
from pydantic import BaseModel, Field
import datetime

from pydantic.networks import HttpUrl


class ProjectBase(BaseModel):
    title: str = Field(example="RCOS API")
    description: str = Field(example="Private API for RCOS infrastructure.")
    languages: List[str] = Field(example=["python"])
    stack: List[str] = Field(example=["fastapi"])
    cover_image_url: Optional[str] = Field(
        None, example="https://via.placeholder.com/640x480")
    homepage_url: Optional[HttpUrl] = Field(None, example="https://rcos.io")
    repository_url: str = Field(example="https://github.com/rcos/rcos-api")
    created_at: datetime.datetime = Field()


class ProjectOut(ProjectBase):
    project_id: int = Field(example=1)


class ProjectIn(ProjectBase):
    pass
