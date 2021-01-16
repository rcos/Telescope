import requests
import jwt
import os

API_URL = "http://198.211.105.73:3000"
JWT_SECRET = os.environ["POSTGREST_JWT_SECRET"]

# Create JSON Web Token to authenticate Postgrest
encoded_jwt = jwt.encode({"role": "api_user"}, JWT_SECRET,
                         algorithm="HS256").decode("utf-8")

s = requests.Session()
s.headers["Authorization"] = "Bearer " + encoded_jwt

# Get RPI student members
students = s.get(
    API_URL + "/users?is_rpi=eq.true&select=username,first_name,last_name").json()
for student in students:
    print(student)
