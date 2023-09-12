
def price(table):
    return pricer(table["pricer"], table["supply"])

def pricer(info, supply):
    return info["price_per_supply"] * (supply - info["base_supply"]) + info["base_price"]

def root_dir():
    """Returns root directory for this project."""
    import git

    repo = git.Repo('.', search_parent_directories=True)
    return repo.working_tree_dir