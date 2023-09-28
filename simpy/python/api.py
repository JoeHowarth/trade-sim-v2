from typing import Dict, List, Optional, Tuple, Union

from fastapi import FastAPI
from fastapi.routing import APIRoute
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
import polars as pl
from dataclasses import dataclass

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


@app.get("/network/shape", response_model=scenarios.NetworkShape)
def network_shape():
    return curr.network


@app.get("/network/{tick}/price")
def price(tick: int) -> Dict[str, float]:
    df = curr.markets.filter(curr.markets["tick"] == tick).select("price", "port")
    return utils.keyed_by(df, index_col="port", extract="price")


@app.get("/network/{tick}/market/{field}")
def market_col(tick: int, field: str) -> Dict[str, float]:
    df = curr.markets.filter(curr.markets["tick"] == tick).select(field, "port")
    return utils.keyed_by(df, index_col="port", extract=field)


@app.get("/network/mapmode")
def list_map_mode() -> List[str]:
    return [
        "price",
        "supply",
        "production",
        "consumption",
    ]


@dataclass
class AgentInfo:
    cargo: Optional[str]
    coins: float
    id: str
    pos: str


@app.get("/agents/{tick}")
def get_agents_pos(tick: int) -> Dict[str, AgentInfo]:
    df = curr.agents.filter(curr.agents["tick"] == tick).select(
        ["cargo", "coins", "id", "pos"]
    )
    return utils.keyed_by(df, index_col="id", drop_index=False)


# @app.get("/network/{tick}/mapmode/{mode}")
# def map_mode(tick: int, mode: str) -> Dict[str, float]:
#     if mode

#     df = curr.markets.filter(curr.markets["tick"] == tick).select(mode, "port")
#     return utils.keyed_by(df, index_col="port", extract=mode)
