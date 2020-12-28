from typing import List, Optional
from asyncpg.types import Range
from pydantic import BaseModel, Field
import datetime

from pydantic.networks import HttpUrl


class MeetingBase(BaseModel):
    semester_id: str = Field(example="202101")
    meeting_type: str = Field(example="large_group")
    host_username: Optional[str] = Field(None, example="manp")
    is_public: bool = Field(True, example=True)
    start_date_time: datetime.datetime = Field()
    end_date_time: datetime.datetime = Field()
    title: Optional[str] = Field(example="Day 15")
    agenda: Optional[str] = Field()
    recording_url: Optional[HttpUrl] = Field()
    presentation_url: Optional[HttpUrl] = Field()
    location: Optional[str] = Field()


class MeetingOut(MeetingBase):
    meeting_id: int = Field(example=1)


class MeetingIn(MeetingBase):
    pass
