from dataclasses import dataclass
from fastapi import Depends
import utils
import os
from typing import Annotated, Dict, List
import polars as pl


@dataclass
class Edge:
    u: str
    v: str


@dataclass
class NetworkShape:
    nodes: List[str]
    edges: List[Edge]


@dataclass
class Replay:
    actions: pl.DataFrame
    agents: pl.DataFrame
    markets: pl.DataFrame
    events: pl.DataFrame
    network: NetworkShape


## Replay Cache
cached_replays: Dict[str, Replay] = {}
last_loaded_time: Dict[str, float] = {}


def get(replay_name: str) -> Replay:
    """
    Load replay from cache if it exists, otherwise load it from disk.
    Can be used as FastAPI Dependency.
    """
    global last_loaded_time  # Declare global here if you are going to modify it

    file_path = f"{utils.root_dir}/output/{replay_name}_tabular.json"
    cur_chg_time = os.path.getmtime(file_path)

    if (
        replay_name not in last_loaded_time
        or cur_chg_time != last_loaded_time[replay_name]
    ):
        cached_replays[replay_name] = _load_replay(replay_name)
        last_loaded_time[replay_name] = cur_chg_time  # Update the entry for replay_name

    return cached_replays[replay_name]


## FastAPI Dependency
ReplayDep = Annotated[Replay, Depends(get)]


def _load_replay(
    name="last",
) -> Replay:
    """
    Load replay from disk.
    Reads tabular output file to load polars dataframes.
    Reads history output file to load network shape.
    """
    # load tabular output file
    (act, ag, mar, ev) = _load_tabular(path=f"output/{name}_tabular.json")

    # load history output file
    data = utils.read_json(f"output/{name}.json")
    # with open(f"{utils.root_dir}output/{name}.json") as json_file:
    #     data = json.load(json_file)

    # join raw_edges with nodes to create readable network
    nodes = data["static_info"]["graph"]["nodes"]
    raw_edges = data["static_info"]["graph"]["edges"]
    network = NetworkShape(
        nodes=nodes, edges=[Edge(nodes[e[0]], nodes[e[1]]) for e in raw_edges]
    )
    mar = mar.select(pl.exclude("pricer"))

    return Replay(actions=act, agents=ag, markets=mar, events=ev, network=network)


def _tabular(blob):
    actions = pl.DataFrame(blob["actions"])
    agents = pl.DataFrame(blob["agents"])
    markets = pl.DataFrame(blob["markets"])
    events = pl.DataFrame(blob["events"])
    return (actions, agents, markets, events)


def _load_tabular(path="output/last_run_tabular.json"):
    return _tabular(utils.read_json(path))
