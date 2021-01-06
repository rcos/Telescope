from api import small_groups
from typing import Optional
from pydantic import BaseModel, Field
import datetime

from enum import Enum


class Source(str, Enum):
    project = "project"
    small_group = "small_group"


class Target(str, Enum):
    discord_server = "discord_server"
    discord_role = "discord_role"
    discord_category = "discord_category"
    discord_text_channel = "discord_text_channel"
    discord_text_voice_channel = "discord_text_voice_channel"


class ChatAssociationBase(BaseModel):
    target_id: str = Field()


class ChatAssociationOut(ChatAssociationBase):
    source_type: Source = Field()
    target_type: Target = Field()
    source_id: str = Field()


class ChatAssociationIn(ChatAssociationBase):
    pass
