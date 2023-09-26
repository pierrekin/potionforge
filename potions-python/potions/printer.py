from potions import model


def print_cheatsheet(title, recipes):
    print(f"--- Potions ({title}) ---")

    def _ingredient_summary(ingredient):
        process_summary = ", ".join(ingredient.process)
        return f"{model.raw_ingredient_names_map[ingredient.key]} ({process_summary})"

    def _get_key(recipe):
        return f"{recipe.potion_kind.department}.{recipe.appeal}.{recipe.potency}.{recipe.potion_kind.name}"

    for recipe in sorted(recipes, key=_get_key, reverse=True):
        ingredients_summary = ", ".join(
            _ingredient_summary(ingredient) for ingredient in recipe.ingredients
        )
        department_name = model.department_names_map[recipe.potion_kind.department]
        potion_name = recipe.potion_kind.name
        recipe_cost = recipe.cost
        print(
            f"{department_name}:  {potion_name} [{recipe.potency} / {recipe_cost}] {ingredients_summary}"
        )
