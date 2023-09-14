from simrs import History
import polars as pl


def tabular(h: History):
    tabular = h.tabular()
    actions = pl.DataFrame(tabular["actions"])
    agents = pl.DataFrame(tabular["agents"])
    markets = pl.DataFrame(tabular["markets"])
    events = pl.DataFrame(tabular["events"])
    return (actions, agents, markets, events)
