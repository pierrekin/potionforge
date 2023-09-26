from potions import model
from potions import suggest
import yaml


def main():
    with open("potions.yml") as config_file:
        config = yaml.load(config_file, yaml.UnsafeLoader)

    suggest.suggest(**config)


if __name__ == "__main__":
    main()
