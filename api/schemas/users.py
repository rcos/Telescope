from typing import Optional
from pydantic import BaseModel, Field


class UserBase(BaseModel):
    preferred_name: Optional[str] = Field(None)
    first_name: str = Field(example="Puck")
    last_name: str = Field(example="Man")
    graduation_year: int = Field(example=1828)
    timezone: str = Field(example="America/New_York")
    is_rpi: bool = Field(example=True)
    is_faculty: bool = Field(example=False)


class UserOut(UserBase):
    username: str = Field(example="manp")


class UserIn(UserBase):
    pass


class UserAccount(BaseModel):
    username: str = Field(example="manp")
    type: str = Field(example="github")
    account_id: str = Field(example="7462762139")
