from typing import List, Tuple, Union

from fastapi import FastAPI
from fastapi.routing import APIRoute
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel

import utils
import scenarios


def custom_generate_unique_id(route: APIRoute):
    print(route)
    return f"{route.name}"


app = FastAPI(generate_unique_id_function=custom_generate_unique_id)
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

curr = scenarios.load_scneario()


# @app.get("/scenarios/{name}")
# def last_scenario(name: str):
#     return a.write_json(row_oriented=True)


@app.get("/network/shape", response_model=scenarios.NetworkShape)
def network_shape():
    return curr.network
