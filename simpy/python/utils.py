
def price(table):
    return pricer(table["pricer"], table["supply"])

def pricer(info, supply):
    return info["price_per_supply"] * (supply - info["base_supply"]) + info["base_price"]