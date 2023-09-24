import subprocess
from utils import root_dir

base = f"cd {root_dir()};"


def shell(cmd):
    return subprocess.getoutput(base + cmd)


def run():
    return shell("cargo run --release run --no-log-to-term")


def run_with_args(
    input_path="input/last.json",
    output_path="output/last_run.json",
    output_tabular_path="output/last_run_tabular.json",
):
    return shell(
        f"cargo run --release run --no-log-to-term --input {input_path} --output {output_path} --tabular {output_tabular_path}"
    )
