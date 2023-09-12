
def price(table):
    return pricer(table["pricer"], table["supply"])

def pricer(info, supply):
    return info["price_per_supply"] * (supply - info["base_supply"]) + info["base_price"]

def root_dir():
    """Returns root directory for this project."""
    import git

    repo = git.Repo('.', search_parent_directories=True)
    return repo.working_tree_dir


class DotDict(dict):
    def __getattr__(self, name):
        return self.get(name, None)

    def __setattr__(self, name, value):
        self[name] = value

    def __delattr__(self, name):
        if name in self:
            del self[name]

def recursive_dotdict(obj):
    if isinstance(obj, dict):
        return DotDict({k: recursive_dotdict(v) for k, v in obj.items()})
    elif isinstance(obj, list):
        return [recursive_dotdict(element) for element in obj]
    else:
        return obj

# # Test the function
# nested_dict = {'a': 1, 'b': {'c': 2, 'd': {'e': 3}}, 'f': [1, {'g': 4}]}
# nested_dot_dict = recursive_dotdict(nested_dict)

# print(nested_dot_dict.b.d.e)  # Should print 3
# print(nested_dot_dict.f[1].g)  # Should print 4