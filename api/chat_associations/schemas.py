from typing import Optional
from pydantic import BaseModel, Field
import datetime


class ChatAssociationBase(BaseModel):
    source_type: str = Field()
    target_type: str = Field()
    source_id: str = Field()
    target_id: str = Field()


class ChatAssociationOut(ChatAssociationBase):
    pass


class ChatAssociationIn(ChatAssociationBase):
    pass
