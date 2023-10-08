import asyncio
from fastapi import FastAPI, WebSocket
from fastapi.routing import APIRoute
from fastapi.middleware.cors import CORSMiddleware
import replay_cache
from replay_cache import ReplayDep


def custom_generate_unique_id(route: APIRoute):
    return f"{route.tags[0]}{route.name}"


from services.replays import router as replays_router
from services.scenarios import router as scenarios_router
from services.data import router as data_router

# app = FastAPI()
app = FastAPI(generate_unique_id_function=custom_generate_unique_id)

app.include_router(replays_router)
app.include_router(scenarios_router)
app.include_router(data_router)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


## Playback


@app.websocket("/replay/{replay_name}/ticks/{startTick}")
async def ticks(websocket: WebSocket, startTick: int, replay: ReplayDep):
    print("start")
    await websocket.accept()
    print("accepted")

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
                tick += 1

                max_tick = replay.agents.select("tick").max().to_series()[0]
                print(max_tick)
                print(
                    replay_cache.get(replay.name)
                    .agents.select("tick")
                    .max()
                    .to_series()[0]
                )
                if tick >= max_tick:
                    tick = max_tick

                await websocket.send_json({"tick": tick, "ms": ms})
                sleep = ms / 1000
            else:
                sleep = 0.1
                if times % 5 == 0:
                    await websocket.send_json({"tick": tick, "ms": ms})
            await asyncio.sleep(sleep)

    await asyncio.gather(read_playback_info(), loop())
