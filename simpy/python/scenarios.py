from dataclasses import dataclass
from typing import Dict, List, Tuple
import input_builder as b
import json
from pydantic import BaseModel
import utils
import cli
import polars as pl


@dataclass
class Edge:
    u: str
    v: str


class NetworkShape(BaseModel):
    nodes: List[str]
    edges: List[Edge]


@dataclass
class Scenario:
    actions: pl.DataFrame
    agents: pl.DataFrame
    markets: pl.DataFrame
    events: pl.DataFrame
    network: NetworkShape


def save_scenario(x, name):
    if name is not None and name != "last":
        with open(f"{utils.root_dir()}input/{name}.json", "w") as fp:
            json.dump(x, fp, indent=2)
    with open(f"{utils.root_dir()}input/last.json", "w") as fp:
        json.dump(x, fp, indent=2)
    return x


def run_scenario(
    input, name="last"
) -> Tuple[pl.DataFrame, pl.DataFrame, pl.DataFrame, pl.DataFrame]:
    save_scenario(input, name)
    cli.run_with_args(
        input_path=f"input/{name}.json",
        output_tabular_path=f"output/{name}_tabular.json",
        output_path=f"output/{name}.json",
    )
    (actions, agents, markets, events) = utils.load_tabular(
        path=f"output/{name}_tabular.json"
    )
    markets = markets.select(pl.exclude("pricer"))
    return (actions, agents, markets, events)


def load_scneario(
    name="last",
) -> Scenario:
    # load tabular output file
    (act, ag, mar, ev) = utils.load_tabular(path=f"output/{name}_tabular.json")

    # load history output file
    with open(f"{utils.root_dir()}output/{name}.json") as json_file:
        data = json.load(json_file)

    # join raw_edges with nodes to create readable network
    nodes = data["static_info"]["graph"]["nodes"]
    raw_edges = data["static_info"]["graph"]["edges"]
    network = NetworkShape(
        nodes=nodes, edges=[Edge(nodes[e[0]], nodes[e[1]]) for e in raw_edges]
    )

    return Scenario(actions=act, agents=ag, markets=mar, events=ev, network=network)


def small_scenario(ticks=100, num_agents=10):
    # Agents
    agent_ids = ["a" + str(i) for i in range(1, num_agents + 1)]

    # Goods
    wheat = "Wheat"
    goods = [wheat]

    # Ports
    genoa = "Genoa"
    rome = "Rome"
    milan = "Milan"
    venice = "Venice"
    marsailles = "Marsailles"
    port_ids = [genoa, rome, milan, venice, marsailles]

    # genoa -> milan -> marsailles
    #   v         v
    # rome -> venice
    edges = [
        (genoa, milan),
        (milan, marsailles),
        (rome, venice),
        (rome, genoa),
        (venice, milan),
    ]
    _market = lambda x: b._market(
        wheat, b._market_info(production=10, consumption=10 - x, supply=1000 + x)
    )

    # net balanced
    ports = [
        b._port(genoa, _market(2)),
        b._port(milan, _market(1)),
        b._port(rome, _market(0)),
        b._port(venice, _market(-1)),
        b._port(marsailles, _market(-2)),
    ]

    _agent = lambda id, pos: b._agent(id, pos, 1000, "Exhaustive")
    agents = [
        _agent(id, port_id) for (id, port_id) in zip(agent_ids, port_ids * num_agents)
    ]

    return b._inputFormat(
        agents=agents, ports=ports, edges=edges, opts=b._opts(ticks=ticks)
    )


def barbell_scenario(ticks=100, num_agents=10):
    # Agents
    agent_ids = ["a" + str(i) for i in range(1, num_agents + 1)]

    # Goods
    wheat = "Wheat"
    goods = [wheat]

    # Ports
    la = "la"
    lb = "lb"
    m1 = "m1"
    m2 = "m2"
    m3 = "m3"
    ra = "ra"

    # la
    #     m1 m2 m3 ra
    # lb
    edges = [
        (la, lb),
        (la, m1),
        (lb, m1),
        (m1, m2),
        (m2, m3),
        (m3, ra),
    ]
    _market = lambda c, s: b._market(
        wheat,
        b._market_info(
            production=100,
            consumption=100 + c,
            supply=1000 - s,
            pricer=b._inversep(100_000),
        ),
    )

    # net balance
    ports = [
        b._port(la, _market(0, 20)),
        b._port(lb, _market(0, 20)),
        b._port(m1, _market(0, 20)),
        b._port(m2, _market(0, 0)),
        b._port(m3, _market(0, 0)),
        b._port(ra, _market(0, -20)),
    ]

    _agent = lambda id, pos: b._agent(id, pos, 1000, "Exhaustive")
    agents = [
        _agent(id, port_id) for (id, port_id) in zip(agent_ids, [la] * num_agents)
    ]

    return b._inputFormat(
        agents=agents, ports=ports, edges=edges, opts=b._opts(ticks=ticks)
    )
