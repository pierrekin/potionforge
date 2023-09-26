from potions import permute
from potions import model
from potions import smarter
from potions import dp_with_categories


def determine_best(potion_kind, ingredients, processes, r):
    eligible_parts = smarter.get_eligible_parts(potion_kind)
    eligible_ingredients = smarter.get_eligible_ingredients(eligible_parts, processes)
    present_ingredients = [
        ingredient
        for ingredient in eligible_ingredients
        if ingredient.key in ingredients
    ]
    possible_recipes = permute.get_all_recipes(present_ingredients, r)
    matching_recipes = smarter.prune_by_kind(possible_recipes, potion_kind)
    strongest_recipes = smarter.select_strongest_recipes(matching_recipes)
    return strongest_recipes


def determine_all_best(ingredients, processes, r):
    best_recipes = []
    for main_effect, elements in model.potion_kinds.items():
        for element in elements:
            potion_kind = model.potion_kinds[main_effect][element]
            best = determine_best(potion_kind, ingredients, processes, r)
            best_recipes.extend(best)

    return best_recipes


def determine_possible(recipes, ingredients):
    result = []
    for recipe in recipes:
        have = ingredients.keys()
        need = [ingredient.key for ingredient in recipe.ingredients]
        if not set(need) - set(have):
            result.append(recipe)

    return result


def suggest(recipes, ingredients):
    prepared_recipes = [
        (
            i,
            recipe.potion_kind.key,
            [ingredient.key for ingredient in recipe.ingredients],
            recipe.potency,
        )
        for (i, recipe) in enumerate(recipes)
    ]
    result = dp_with_categories.max_score_recipes(prepared_recipes, ingredients)
    return [recipes[i] for i in result]


def main(ingredients, processes, r, outputs):
    best_recipes = determine_all_best(ingredients, processes, r)
    possible_recipes = determine_possible(best_recipes, ingredients)
    if "possible" in outputs:
        permute.print_cheatsheet("Possible", possible_recipes)
    suggestions = suggest(possible_recipes, ingredients)
    if "suggested" in outputs:
        permute.print_cheatsheet("Suggested", suggestions)
