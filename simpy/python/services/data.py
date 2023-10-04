from typing import Dict, List, Optional

from fastapi import APIRouter
from dataclasses import dataclass
from replay_cache import ReplayDep
from services import replays

import utils


router = APIRouter(prefix="/data/{replay_name}", tags=["data"])


@router.get("/network/shape", response_model=replays.NetworkShape)
def network_shape(replay: ReplayDep):
    return replay.network


@router.get("/network/{tick}/market/{field}")
def market_col(replay: ReplayDep, tick: int, field: str) -> Dict[str, float]:
    df = replay.markets.filter(replay.markets["tick"] == tick).select(field, "port")
    return utils.keyed_by(df, index_col="port", extract=field)


@router.get("/network/mapmode")
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


## Agent Data Views


@router.get("/agents/{tick}")
def get_agents_pos(replay: ReplayDep, tick: int) -> Dict[str, AgentInfo]:
    df = replay.agents.filter(replay.agents["tick"] == tick).select(
        ["cargo", "coins", "id", "pos"]
    )
    return utils.keyed_by(df, index_col="id", drop_index=False)
