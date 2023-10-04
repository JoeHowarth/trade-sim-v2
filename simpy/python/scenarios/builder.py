# Helpers to construct InputFormat
from typing import Tuple, Union
import utils


def _opts(ticks=10):
    return {"ticks": ticks}


def _route(from_: str, to: str):
    return [from_, to]


def _agent(id: str, pos: str, coins: int, behavior: str, cargo: str = None):
    return {"id": id, "pos": pos, "cargo": cargo, "coins": coins, "behavior": behavior}


# defaults to 1000 supply, -0.2 price per supply, 100 base price
# at 0 supply, price is 300
# for 5 additional supply, price is reduced by 1
def _linearp(base_price=100, price_per_supply=-0.2, base_supply=1000):
    assert price_per_supply < 0
    return {
        "tag": "Linear",
        "base_price": base_price,
        "base_supply": base_supply,
        "price_per_supply": price_per_supply,
    }


def _inversep(coef=1000):
    assert coef > 0
    return {
        "tag": "Inverse",
        "coef": coef,
    }


def _market_info(
    production: float = None,
    consumption: float = None,
    supply: float = 1000,
    pricer: dict = _linearp(),
    net=None,
):
    if net is not None:
        assert consumption is None
        assert production is None
        consumption = 100
        production = net + consumption

    return {
        "consumption": consumption,
        "supply": supply,
        "production": production,
        "pricer": pricer,
    }


def _market(table: Union[str, list[Tuple[str, dict]]], maybe_info: dict = None):
    if isinstance(table, str):
        assert maybe_info is not None
        table = [(table, maybe_info)]
    return {
        "table": {k: v for k, v in table},
    }


def _port(id: str, market: Union[dict, list]):
    if isinstance(market, list):
        market = _market(market)
    return {"id": id, "market": market}


def _inputFormat(
    agents: list[dict], ports: list[dict], edges: list[list[str]], opts: dict = {}
):
    return {"agents": agents, "ports": ports, "edges": edges, "opts": opts}


def save_scenario(x, name):
    if name is not None and name != "last":
        utils.write_json(f"{utils.root_dir}input/{name}.json", x)
    utils.write_json(f"{utils.root_dir}input/last.json", x)
