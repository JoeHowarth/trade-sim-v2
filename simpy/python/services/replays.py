import os
from fastapi import APIRouter
import polars as pl
from utils import read_json
from dataclasses import dataclass
from typing import List
import json
from pydantic import BaseModel
import utils
import polars as pl
import replay_cache

router = APIRouter(prefix="/replay", tags=["replay"])

## Types


class ReplayInfo(BaseModel):
    name: str
    ticks: int


## Routes


@router.get("/")
def all():
    return list_replays()


@router.get("/info/{name}")
def get_into(name: str) -> ReplayInfo:
    replay = replay_cache.get(name)
    return ReplayInfo(name=name, ticks=replay.actions.select("tick").max())


## Utils


def list_replays():
    dir_path = f"{utils.root_dir}/output"
    return [
        item[:-13] for item in os.listdir(dir_path) if item.endswith("_tabular.json")
    ]
