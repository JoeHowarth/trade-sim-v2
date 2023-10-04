import json
import os
from typing import Any, Dict, List, Optional, Tuple
from fastapi import APIRouter
from pydantic import BaseModel
from scenarios.builder import save_scenario
from services.replays import Scenario, load_tabular
import polars as pl
from .. import utils
from .. import cli

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
def all():
    return list_scenarios()


@router.get("/{name}")
def get(name: str) -> Dict[str, Any]:
    return Scenario(utils.read_json(f"/input/{name}.json"))


@router.post("/{name}")
def post(name: str, scenario: Scenario):
    utils.write_json(f"/input/{name}.json", scenario)


## Utils


def list_scenarios() -> List[str]:
    dir_path = f"{utils.root_dir}/input"
    return [item[:-5] for item in os.listdir(dir_path) if item.endswith(".json")]


def run_scenario(name: str = "last", input: Scenario | None = None):
    if input is not None:
        save_scenario(input, name)

    cli.run_with_args(
        input_path=f"input/{name}.json",
        output_tabular_path=f"output/{name}_tabular.json",
        output_path=f"output/{name}.json",
    )
