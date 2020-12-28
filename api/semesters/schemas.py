from typing import Any, Optional, Tuple
from asyncpg.types import Range
from pydantic import BaseModel, Field
import datetime


class SemesterBase(BaseModel):
    title: str = Field(example="Spring 2021")
    start_date: datetime.date = Field(example="2021-01-19")
    end_date: datetime.date = Field(example="2021-05-30")


class SemesterOut(SemesterBase):
    semester_id: str = Field(example="202101")


class SemesterIn(SemesterBase):
    pass
