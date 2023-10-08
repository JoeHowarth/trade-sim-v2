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
    name: str
    actions: pl.DataFrame
    agents: pl.DataFrame
    markets: pl.DataFrame
    events: pl.DataFrame
    network: NetworkShape


## Replay Cache
cached_replays: Dict[str, Replay] = {}  # replay_name -> Replay
last_loaded_time: Dict[str, float] = {}  # last time the replay was loaded from disk
dirty: Dict[str, bool] = {}  # has the replay been modified in-memory since last load?


def get(replay_name: str) -> Replay:
    """
    Load replay from cache if it exists, otherwise load it from disk.
    Can be used as FastAPI Dependency.
    """
    global last_loaded_time

    file_path = f"{utils.root_dir}output/{replay_name}_tabular.json"

    if not os.path.exists(file_path) or dirty.get(replay_name, False):
        print("replay not found in cache or dirty, reading from cache")
        return cached_replays.get(replay_name, None)

    ## If the replay exists, load it's last modified time
    cur_chg_time = os.path.getmtime(file_path)

    ## If the replay is not in the cache, or it's been modified since last load, load it
    if (
        replay_name not in last_loaded_time
        or cur_chg_time != last_loaded_time[replay_name]
    ):
        print("replay not in cache or modified since last load, loading from disk")
        cached_replays[replay_name] = load_replay_from_output(replay_name)
        last_loaded_time[replay_name] = cur_chg_time  # Update the entry for replay_name
        dirty[replay_name] = False

    return cached_replays[replay_name]


## FastAPI Dependency
ReplayDep = Annotated[Replay, Depends(get)]


def set(replay_name: str, replay: Replay):
    """
    Set replay in cache.
    Replay will not be written to disk until save() is called.
    """
    cached_replays[replay_name] = replay
    dirty[replay_name] = True


def save():
    """
    Save all dirty replays to disk.
    """
    # todo: test it
    for replay_name in dirty:
        if dirty[replay_name]:
            _save_replay(replay_name)
            dirty[replay_name] = False


def _save_replay(replay_name: str):
    # todo: test it
    replay = cached_replays[replay_name]
    as_json = {
        "actions": replay.actions.to_dict(),
        "agents": replay.agents.to_dict(),
        "markets": replay.markets.to_dict(),
        "events": replay.events.to_dict(),
    }
    utils.write_json(f"output/{replay_name}_tabular.json", as_json)


def load_replay_from_output(
    name="last",
) -> Replay:
    """
    Load replay from disk.
    Reads tabular output file to load polars dataframes.
    Reads history output file to load network shape.
    """
    # load tabular output file
    (act, ag, mar, ev) = _tabular(utils.read_json(f"output/{name}_tabular.json"))

    return Replay(
        name=name,
        actions=act,
        agents=ag,
        markets=mar,
        events=ev,
        network=network_shape_from_history(name),
    )


def new_replay_from_incremental(name: str, tick_result: dict):
    (actions, agents, markets, events) = _tabular(tick_result)
    replay = Replay(
        name=name,
        actions=actions,
        agents=agents,
        markets=markets,
        events=events,
        network=network_shape_from_scenario(name),
    )
    set(name, replay)
    return replay


def network_shape_from_scenario(name: str):
    data = utils.read_json(f"input/{name}.json")
    return NetworkShape(
        nodes=[p["id"] for p in data["ports"]],
        edges=[Edge(e[0], e[1]) for e in data["edges"]],
    )


def network_shape_from_history(name: str):
    data = utils.read_json(f"output/{name}.json")
    nodes = data["static_info"]["graph"]["nodes"]
    raw_edges = data["static_info"]["graph"]["edges"]
    return NetworkShape(
        nodes=nodes, edges=[Edge(nodes[e[0]], nodes[e[1]]) for e in raw_edges]
    )


def _tabular(blob):
    actions = pl.DataFrame(blob["actions"])
    agents = pl.DataFrame(blob["agents"])
    markets = pl.DataFrame(blob["markets"])
    events = pl.DataFrame(blob["events"])
    markets = markets.select(pl.exclude("pricer"))
    return (
        actions.select(sorted(actions.columns)),
        agents.select(sorted(agents.columns)),
        markets.select(sorted(markets.columns)),
        events.select(sorted(events.columns)),
    )
