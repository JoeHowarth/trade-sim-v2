import polars as pl
from polars import col, lit
import altair as alt


def plot_agents(agents: pl.DataFrame):
    """
    Plot agent coins over time.
    """
    base = alt.Chart(agents).encode(
        x="tick:Q",
        y=alt.Y("coins:Q").scale(zero=False),
        color=alt.Color("id:O").scale(scheme="dark2"),
    )
    lines = base.transform_loess(
        "tick", "coins", bandwidth=0.5, groupby=["id"]
    ).mark_line(size=4)
    return (base.mark_point() + lines).interactive()


def plot_agent_locations(agents: pl.DataFrame):
    """
    Plot agent locations over time.
    """
    base = alt.Chart(agents).encode(
        x="tick:Q",
        y="pos:N",
        color=alt.Color("id:O").scale(scheme="dark2"),
    )
    return base.mark_point().interactive()


def plot_prices_by_port(ports: pl.DataFrame, color="dark2"):
    """
    Plot prices by port over time.
    """
    base = alt.Chart(ports).encode(
        x="tick:Q",
        y=alt.Y("price:Q", scale=alt.Scale(zero=False)),
        color=alt.Color("port:O").scale(scheme=color),
    )
    lines = base.transform_loess(
        "tick", "price", bandwidth=0.1, groupby=["port"]
    ).mark_line(size=4)
    return (base.mark_point() + lines).interactive()


# def no_agent_markets(input_format) -> pl.DataFrame:
#     """
#     Where would prices have been if agents didn't trade?
#     Runs a scenario with no agents and returns the markets.
#     """
#     input_format = input_format.copy()
#     input_format["agents"] = []
#     (_, _, no_agent_markets, _) = run_scenario(input_format)
#     return no_agent_markets


def make_routes(events):
    """
    Make a dataframe of agent routes.
    A route is where an agent buys from one port and sells to another.
    """
    trade_events = events.filter(events["event"] == "Trade")

    def foo(df):
        buys = df.filter(df["amt"] > 0).select(
            "agent",
            "amt",
            pl.col("cost").alias("buy_cost"),
            pl.col("port").alias("src"),
            pl.col("tick").alias("buy_tick"),
        )
        sells = df.filter(df["amt"] < 0).select(
            pl.col("cost").alias("sell_cost"),
            pl.col("port").alias("dst"),
            pl.col("tick").alias("sell_tick"),
        )

        df = pl.concat([buys, sells], how="horizontal")
        df = df.with_columns((-df["sell_cost"] - df["buy_cost"]).alias("profit"))
        df = df.with_columns((df["buy_cost"] / df["amt"]).alias("buy_price"))
        df = df.with_columns((df["sell_cost"] / -df["amt"]).alias("sell_price"))
        return df

    return trade_events.groupby("agent").apply(foo)


def plot_trades(trades: pl.DataFrame):
    base = alt.Chart(trades).encode(
        x="sell_tick:Q",
        y="profit:Q",
        color=alt.Color("agent:N").scale(scheme="dark2"),
    )
    return base.mark_point().interactive()


def plot_buy_and_sell_prices(trades: pl.DataFrame):
    buy = alt.Chart(trades).encode(
        x="buy_tick:Q",
        y="buy_price:Q",
        color=alt.Color("dst:N").scale(scheme="dark2"),
    )
    sell = alt.Chart(trades).encode(
        x="sell_tick:Q",
        y="sell_price:Q",
        color=alt.Color("dst:N").scale(scheme="dark2"),
    )

    return (buy.mark_point() + sell.mark_point()).interactive()
