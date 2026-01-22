import toml

def load_config():
    with open("src/config.toml", "r") as file:
        config = toml.load(file)
    return config

config = load_config()
