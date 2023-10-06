import json
import os
from typing import Any, Dict, List, Optional, Tuple
from fastapi import APIRouter
from pydantic import BaseModel
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


@router.post("/")
def run_scenario(name: str = "last", input: Scenario | None = None) -> bool:
    if input is not None:
        save_scenario(input, name)

    print("run_scenario", name)
    cli.run_with_args(
        input_path=f"input/{name}.json",
        output_tabular_path=f"output/{name}_tabular.json",
        output_path=f"output/{name}.json",
    )

    return True


## Utils


def list_scenarios() -> List[str]:
    dir_path = f"{utils.root_dir}/input"
    print("list dir", list(os.listdir(dir_path)))
    return [item[:-5] for item in os.listdir(dir_path) if item.endswith(".json")]
