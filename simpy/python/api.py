import asyncio
from fastapi import FastAPI, WebSocket
from fastapi.routing import APIRoute
from fastapi.middleware.cors import CORSMiddleware
import services


def custom_generate_unique_id(route: APIRoute):
    return f"{route.name}"


app = FastAPI(generate_unique_id_function=custom_generate_unique_id)
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)
app.include_router(services.replays.router)
app.include_router(services.scenarios.router)
app.include_router(services.data.router)


## Playback


## Todo: use use ReplayDep instead of curr
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
                sleep = ms / 1000
            else:
                sleep = 0.1
                if times % 5 == 0:
                    await websocket.send_json({"tick": tick, "ms": ms})
            await asyncio.sleep(sleep)

    await asyncio.gather(read_playback_info(), loop())
