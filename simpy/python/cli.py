import asyncio
import json
import subprocess
from time import sleep
import replay_cache
from replay_cache import Replay, new_replay_from_incremental
from utils import root_dir
import polars as pl

base = f"cd {root_dir};"


def shell(cmd):
    return subprocess.getoutput(base + cmd)


def run(
    input_path=None,
    output_path=None,
    output_tabular_path=None,
):
    args = " ".join(
        _args(
            input_path=input_path,
            output_path=output_path,
            output_tabular_path=output_tabular_path,
        )
    )
    return shell(f"cargo run --release run {args}")


def _args(
    push_tick_output=False,
    input_name=None,
    output_name=None,
    save_dir=root_dir,
) -> list[str]:
    args = [
        "./target/release/cli",
        "run",
    ]

    args.append("--stdout-behavior")
    if push_tick_output:
        args.append("push-tick-output")
    else:
        args.append("none")

    if input_name is not None:
        args.append("--input-name")
        args.append(input_name)

    if output_name is not None:
        args.append("--output-name")
        args.append(output_name)

    if save_dir is not None:
        args.append("--save-dir")
        args.append(save_dir)

    return args


async def run_channel(
    replay_name: str,
    save_dir=root_dir,
):
    replay = None

    shell("cargo build --release")

    proc = await asyncio.create_subprocess_exec(
        *_args(
            input_name=replay_name,
            save_dir=save_dir,
            push_tick_output=True,
        ),
        cwd=root_dir,
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.PIPE,
    )

    # Loop to read lines from stdout
    # tick = 1
    while True:
        await asyncio.sleep(0.1)
        line = await proc.stdout.readline()
        incoming_data = json.loads(line.strip())

        if incoming_data.get("done", False):
            break

        # print("tick: ", tick)
        # tick += 1

        if replay is None:
            replay = new_replay_from_incremental(replay_name, incoming_data)
        else:
            try:
                replay.actions = create_or_concat(
                    replay.actions, incoming_data.get("actions", {})
                )
                replay.agents = create_or_concat(
                    replay.agents, incoming_data.get("agents", {})
                )
                replay.markets = create_or_concat(
                    replay.markets, incoming_data.get("markets", {})
                )
                replay.events = create_or_concat(
                    replay.events, incoming_data.get("events", {})
                )
            except:
                
        print("tick", replay.markets.select("tick").max().to_series()[0])
        replay_cache.set(replay_name, replay)

    await proc.wait()


def create_or_concat(existing_df: pl.DataFrame, new_data: dict) -> pl.DataFrame:
    existing_df = existing_df.select(sorted(existing_df.columns))
    new_df = pl.DataFrame(new_data, schema=existing_df.schema)
    new_df = new_df.select(sorted(new_df.columns)).select(pl.exclude("pricer"))

    if existing_df is None:
        return new_df
    else:
        return pl.concat([existing_df, new_df])


##


def run_channel_sync(
    replay: Replay,
    input_path=None,
    output_path=None,
    output_tabular_path=None,
):
    shell("cargo build --release")

    proc = subprocess.Popen(
        _args(True, input_path, output_path, output_tabular_path),
        cwd=root_dir,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        bufsize=1,
        universal_newlines=True,
    )

    # Loop to read lines from stdout
    while True:
        line = proc.stdout.readline()
        if not line:
            sleep(0.1)
            continue
        incoming_data = json.loads(line.strip())
        if incoming_data.get("done", False):
            break

        replay.actions = create_or_concat(
            replay.actions, incoming_data.get("actions", {})
        )
        replay.agents = create_or_concat(replay.agents, incoming_data.get("agents", {}))
        replay.markets = create_or_concat(
            replay.markets, incoming_data.get("markets", {})
        )
        replay.events = create_or_concat(replay.events, incoming_data.get("events", {}))

    proc.wait()
