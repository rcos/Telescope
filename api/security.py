import os
from fastapi.exceptions import HTTPException
from fastapi.param_functions import Security
from fastapi.security.api_key import APIKeyHeader

API_KEY = os.environ["API_KEY"]
API_KEY_NAME = "api_key"
api_key_header = APIKeyHeader(name=API_KEY_NAME, auto_error=False)


async def requires_api_key(key: str = Security(api_key_header)):
    if key == API_KEY:
        return key
    raise HTTPException(
        status_code=403, detail=f"Missing or invalid {API_KEY_NAME} header")


async def get_api_key(key: str = Security(api_key_header)):
    if key == API_KEY:
        return key
    return None
