from typing import Optional
from pydantic import BaseModel, Field


class User(BaseModel):
    username: str = Field(example="puckm")
    preferred_name: Optional[str] = Field(None)
    first_name: str = Field(example="Puck")
    last_name: str = Field(example="Man")
    graduation_year: int = Field(example=1828)
    is_rpi: bool = Field(example=True)
    timezone: str = Field(example="America/New_York")
