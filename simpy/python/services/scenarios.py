import asyncio
import json
import os
from typing import Any, Dict, List, Optional, Tuple
from fastapi import APIRouter, BackgroundTasks
from pydantic import BaseModel
import replay_cache
from scenarios.builder import save_scenario
import polars as pl
import utils
import cli

router = APIRouter(prefix="/scenario", tags=["scenario"])

## Types


class Scenario(BaseModel):
    description: Optional[str]
    agents: list[dict]
    ports: list[dict]
    edges: list[list[str]]
    opts: dict = {}


## Routes


@router.get("/")
def all() -> List[str]:
    return list_scenarios()


@router.get("/{name}")
def get(name: str) -> Scenario:
    _json = utils.read_json(f"/input/{name}.json")
    return Scenario(
        description=None,
        agents=_json["agents"],
        ports=_json["ports"],
        edges=_json["edges"],
        opts=_json["opts"],
    )


@router.post("/{name}")
def post(name: str, scenario: Scenario):
    utils.write_json(f"/input/{name.strip()}.json", scenario.model_dump(mode="json"))


@router.put("/sync")
async def run_scenario_sync(name: str = "last", input: Scenario | None = None) -> bool:
    if input is not None:
        save_scenario(input, name)

    print("run_scenario", name)
    cli.run_channel_sync(name)

    return True


@router.post("/")
async def run_scenario_async(name: str = "last", input: Scenario | None = None) -> bool:
    if input is not None:
        save_scenario(input, name)

    print("run_scenario", name)
    asyncio.create_task(cli.run_channel(name))

    async def print_tick():
        while True:
            replay = replay_cache.get(name)
            print(
                "tick from task: ", replay.markets.select("tick").max().to_series()[0]
            )
            await asyncio.sleep(0.5)

    asyncio.create_task(print_tick())

    return True


## Utils


def list_scenarios() -> List[str]:
    dir_path = f"{utils.root_dir}/input"
    print("list dir", list(os.listdir(dir_path)))
    return [item[:-5] for item in os.listdir(dir_path) if item.endswith(".json")]
