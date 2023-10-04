import json
from typing import Optional
import polars as pl


def get_root_dir():
    """Returns root directory for this project."""
    import git

    repo = git.Repo(".", search_parent_directories=True)
    return repo.working_tree_dir + "/"


root_dir = get_root_dir()


def init_logging():
    import logging
    import os

    # Path to the log file
    log_file_path = "pylog.log"

    # Remove the log file if it exists
    if os.path.exists(log_file_path):
        os.remove(log_file_path)

    logger = logging.getLogger()
    logger.setLevel(logging.DEBUG)
    # create file handler which logs even debug messages
    fh = logging.FileHandler(log_file_path, mode="w")
    # Create a formatter and set it for the handler
    formatter = logging.Formatter("%(name)s|%(levelname)s| %(message)s")
    fh.setFormatter(formatter)
    fh.setLevel(logging.DEBUG)
    logger.addHandler(fh)


def keyed_by(df: pl.DataFrame, index_col: str, extract: str = None, drop_index=True):
    x = df.unique(subset=[index_col], keep="last").partition_by(
        by=[index_col], as_dict=True, maintain_order=True
    )

    d = {}
    for index, frame in x.items():
        if drop_index:
            frame = frame.select(pl.exclude(index_col))
        row = frame.to_dicts()[0]
        if extract is not None:
            row = row[extract]
        d[index] = row
    return d


def read_json(path):
    with open(root_dir + path) as json_file:
        return json.load(json_file)


def write_json(path, data):
    with open(root_dir + path, "w") as fp:
        json.dump(data, fp, indent=2)


def price(table):
    return pricer(table["pricer"], table["supply"])


def pricer(info, supply):
    return (
        info["price_per_supply"] * (supply - info["base_supply"]) + info["base_price"]
    )


# class DotDict(dict):
#     def __getattr__(self, name):
#         return self.get(name, None)

#     def __setattr__(self, name, value):
#         self[name] = value

#     def __delattr__(self, name):
#         if name in self:
#             del self[name]


# def recursive_dotdict(obj):
#     if isinstance(obj, dict):
#         return DotDict({k: recursive_dotdict(v) for k, v in obj.items()})
#     elif isinstance(obj, list):
#         return [recursive_dotdict(element) for element in obj]
#     else:
#         return obj


# # Test the function
# nested_dict = {'a': 1, 'b': {'c': 2, 'd': {'e': 3}}, 'f': [1, {'g': 4}]}
# nested_dot_dict = recursive_dotdict(nested_dict)

# print(nested_dot_dict.b.d.e)  # Should print 3
# print(nested_dot_dict.f[1].g)  # Should print 4
