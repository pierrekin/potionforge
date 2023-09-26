from potions import permute
from potions import model


def get_eligible_parts(potion_kind):
    result = [
        "stimulant",
        "impurity",
        "cat",
        "bitter",
        "tasty",
        "unsavory",
        "sweet",
        "toxin",
        "antitoxin",
    ]
    result.extend(potion_kind.parts)
    result.extend(
        [model.infusions[part] for part in potion_kind.parts if part in model.infusions]
    )
    return result


def get_eligible_ingredients(eligible_parts, processes):
    result = []
    for ingredient in permute.get_all_ingredients(processes):
        extra_parts = set(ingredient.parts) - set(eligible_parts)
        if not extra_parts:
            result.append(ingredient)
    return result


def prune_by_kind(recipes, potion_kind):
    result = []
    for recipe in recipes:
        if recipe.potion_kind == potion_kind:
            result.append(recipe)
    return result


def select_strongest_recipes(recipes):
    best_potency = 0
    for recipe in recipes:
        if recipe.potency > best_potency:
            best_potency = recipe.potency

    result = []
    for recipe in recipes:
        if recipe.potency == best_potency:
            result.append(recipe)
    return result


def select_best_recipes(recipes, n):
    best_recipes = sorted(recipes, key=lambda x: x.cost)
    return best_recipes[:n]


def determine_best(potion_kind, processes, r):
    print(f"Determining possible recipes for {potion_kind.name}")
    eligible_parts = get_eligible_parts(potion_kind)
    eligible_ingredients = get_eligible_ingredients(eligible_parts, processes)
    possible_recipes = permute.get_all_recipes(eligible_ingredients, r)
    matching_recipes = prune_by_kind(possible_recipes, potion_kind)
    strongest_recipes = select_strongest_recipes(matching_recipes)
    best_recipes = select_best_recipes(strongest_recipes, 5)
    permute.print_cheatsheet(best_recipes)


def main():
    potion_kind = model.potion_kinds["bone"]["fire"]
    determine_best(potion_kind, r=5)


if __name__ == "__main__":
    main()
