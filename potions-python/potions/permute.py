import itertools
from potions import model
from potions import simulate


def permute_ingredient(ingredient, processes=None):
    processes = processes if processes is not None else ["cut"]

    result = [ingredient]

    if "cut" in processes:
        result += simulate.process_cut(ingredient)

    if "ferment" in processes:
        result += [
            simulate.process_ferment(ingredient) for ingredient in result if ingredient
        ]

    if "infuse" in processes:
        result += [
            simulate.process_infuse(ingredient) for ingredient in result if ingredient
        ]

    return [ingredient for ingredient in result if ingredient]


def flatten_ingredients(nested_ingredients):
    result = []
    for ingredients in nested_ingredients:
        for ingredient in ingredients:
            result.append(ingredient)
    return result


def permute_ingredients(ingredients, processes):
    return flatten_ingredients(
        [permute_ingredient(ingredient, processes) for ingredient in ingredients]
    )


def combine_ingredients(ingredients, max_r):
    result = []
    for r in range(2, max_r + 1):
        for combination in itertools.combinations(ingredients, r):
            keys = [ingredient.key for ingredient in combination]
            if len(keys) == len(set(keys)):
                result.append(combination)
    return result


def get_all_recipes(raw_ingredients, processes, r):
    print("config", len(raw_ingredients), r)
    all_ingredients = permute_ingredients(raw_ingredients, processes)
    print("ingredients", len(all_ingredients))
    all_combinations = combine_ingredients(all_ingredients, r)
    print("combinations", len(all_combinations))
    all_results = [simulate.simulate(combination) for combination in all_combinations]
    all_recipes = [recipe for recipe in all_results if recipe]
    print("recipes", len(all_recipes))
    return all_recipes
