from typing import List, Optional
from pydantic import BaseModel, Field


class SmallGroupBase(BaseModel):
    title: str = Field(example="Small Group 1")
    location: Optional[str] = Field(None, example="Sage 3303")


class SmallGroupCreate(SmallGroupBase):
    semester_id: str = Field(example="202101")


class SmallGroupOut(SmallGroupCreate):
    small_group_id: int = Field(example=1)
    mentor_usernames: List[str] = Field([], example=["manp", "matraf"])
    project_ids: List[int] = Field([], example=[1, 2])


class SmallGroupIn(SmallGroupBase):
    pass
