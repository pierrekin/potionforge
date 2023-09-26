from potions import permute
from potions import model
from potions import printer
from pulp import LpMaximize, LpProblem, LpVariable, lpSum, PULP_CBC_CMD


def _suggest_pulp(recipes, available_ingredients):
    # Define the problem
    problem = LpProblem("Recipe_Suggestion", LpMaximize)

    # Pack recipes for PuLP
    recipe_kinds = [(recipe.potion_kind.key, i) for (i, recipe) in enumerate(recipes)]
    recipe_ingredients = [
        [ingredient.key for ingredient in recipe.ingredients] for recipe in recipes
    ]

    # Create variables
    recipe_vars = LpVariable.dicts(
        "Recipe",
        recipe_kinds,
        0,
        1,
        cat="Binary",
    )

    # Objective function: maximize the number of selected recipes
    problem += (
        lpSum([recipe_vars[kind] for kind in recipe_kinds]),
        "Recipe_Count",
    )

    # Constraint: At most one recipe per category
    for potion_kind in model.potion_kinds:
        problem += (
            lpSum(
                [
                    recipe_vars[(kind, i)]
                    for (kind, i) in recipe_kinds
                    if kind == potion_kind.key
                ]
            )
            <= 1,
            f"AtMostOne_{potion_kind.name}",
        )

    # Constraint: Available ingredients
    for ingredient in available_ingredients:
        problem += (
            lpSum(
                [
                    recipe_vars[(kind, i)]
                    for kind, i in recipe_kinds
                    if ingredient in recipe_ingredients[i]
                ]
            )
            <= available_ingredients[ingredient],
            f"Available_{ingredient}",
        )

    # Constraint: At most 5 recipes per department
    for department in model.departments:
        problem += (
            lpSum(
                [
                    recipe_vars[(kind, i)]
                    for kind, i in recipe_kinds
                    if recipes[i].potion_kind.department == department
                ]
            )
            <= 5,
            f"AtMostFive_{department}",
        )

    # Solve the problem
    # problem.solve(PULP_CBC_CMD(msg=0))
    # problem.writeLP("problem.lb")
    problem.writeMPS("problem.mps")
    problem.solve()

    # Output the selected recipes
    output = []
    for kind, i in recipe_kinds:
        if recipe_vars[(kind, i)].varValue == 1:
            output.append(recipes[i])
    return output


def suggest(available_ingredients, processes, arcane_power):
    ingredients = [
        model.raw_ingredients_map[ingredient] for ingredient in available_ingredients
    ]
    all_recipes = permute.get_all_recipes(ingredients, processes, arcane_power)
    suggested_recipes = _suggest_pulp(all_recipes, available_ingredients)
    printer.print_cheatsheet("Suggested Recipes", suggested_recipes)


if __name__ == "__main__":
    suggest(
        {ingredient_key: 3 for ingredient_key in model.raw_ingredients_map.keys()},
        ["cut", "infuse", "ferment"],
        3,
    )
