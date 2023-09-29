import asyncio
from typing import Dict, List, Optional, Tuple, Union

from fastapi import FastAPI, WebSocket, WebSocketDisconnect
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


@dataclass
class InitInfo:
    max_ticks: int


@app.get("/init/{scenario_name}")
def init(scenario_name: str):
    curr  # cause it to reference the global variable
    curr = scenarios.load_scneario(name=scenario_name)
    return InitInfo(
        max_ticks=curr.agents.select("tick").max().to_list()[0],
    )


@app.get("/agents/{tick}")
def get_agents_pos(tick: int) -> Dict[str, AgentInfo]:
    df = curr.agents.filter(curr.agents["tick"] == tick).select(
        ["cargo", "coins", "id", "pos"]
    )
    return utils.keyed_by(df, index_col="id", drop_index=False)


@app.websocket("/wstest")
async def ws_test(websocket: WebSocket):
    try:
        print(
            f"WS: CONNECTING TO {websocket.url}, HDR = {websocket.headers}, PARAMS = {websocket.query_params} ..."
        )
        await websocket.accept()
        print(f"WS: CONNECTED {websocket.url}")

        async for txt in websocket.iter_text():
            await websocket.send_text(txt)
    except WebSocketDisconnect as err:
        print(f"Websocket [{err.code}]: {err.reason}")
        raise


@app.websocket("/ticks/{startTick}")
async def ticks(websocket: WebSocket, startTick: int):
    await websocket.accept()

    tick = startTick
    ms = 2000

    async def read_playback_info():
        nonlocal tick
        nonlocal ms
        async for data in websocket.iter_json():
            print(data)
            if "tick" in data:
                tick = data["tick"]
            if "ms" in data:
                ms = data["ms"]
            await websocket.send_json({"tick": tick, "ms": ms})

    async def loop():
        nonlocal tick
        nonlocal ms
        times = 0
        while True:
            times += 1
            if ms > 0:
                # max_tick = curr.agents.select("tick").max().to_series()[0]
                # print(max_tick)
                # if tick >= max_tick:
                #     tick = max_tick
                tick += 1
                await websocket.send_json({"tick": tick, "ms": ms})
                sleep = ms /1000
            else:
                sleep = 0.1
                if times % 5 == 0:
                    await websocket.send_json({"tick": tick, "ms": ms})
            await asyncio.sleep(sleep)

    await asyncio.gather(read_playback_info(), loop())


# @app.get("/network/{tick}/mapmode/{mode}")
# def map_mode(tick: int, mode: str) -> Dict[str, float]:
#     if mode

#     df = curr.markets.filter(curr.markets["tick"] == tick).select(mode, "port")
#     return utils.keyed_by(df, index_col="port", extract=mode)
