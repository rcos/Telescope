from typing import Optional
from pydantic import BaseModel, Field
import datetime


class EnrollmentBase(BaseModel):
    project_id: Optional[int] = Field(None)
    is_project_lead: bool = Field(example=False)
    credits: int = Field(example=4)
    is_for_pay: bool = Field(example=False)
    mid_year_grade: Optional[float] = Field(None)
    final_grade: Optional[float] = Field(None)
    enrolled_at: datetime.datetime = Field()


class EnrollmentOut(EnrollmentBase):
    semester_id: str = Field(example="202101")
    username: str = Field(example="manp")


class EnrollmentIn(EnrollmentBase):
    pass
