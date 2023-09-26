from potions.model import Ingredient
from potions.model import Recipe
from potions import model


def process_cut(ingredient):
    return [
        # Vertical cut, keep left half.
        Ingredient(
            ingredient.key,
            ingredient.name,
            ingredient.process + ["crush"],
            ingredient.kind,
            [ingredient.parts[1], ingredient.parts[2]],
        ),
        # Horizontal cut, keep top half.
        Ingredient(
            ingredient.key,
            ingredient.name,
            ingredient.process + ["blanch"],
            ingredient.kind,
            [ingredient.parts[0], ingredient.parts[1]],
        ),
        # Vertical cut, keep right half.
        Ingredient(
            ingredient.key,
            ingredient.name,
            ingredient.process + ["dry"],
            ingredient.kind,
            [ingredient.parts[0], ingredient.parts[3]],
        ),
        # Horizontal cut, keep bottom half.
        Ingredient(
            ingredient.key,
            ingredient.name,
            ingredient.process + ["pickled"],
            ingredient.kind,
            [ingredient.parts[2], ingredient.parts[3]],
        ),
    ]


def process_ferment(ingredient):
    if "impurity" in ingredient.parts:
        # Turns Impurities into Stimulants
        return Ingredient(
            ingredient.key,
            ingredient.name,
            ingredient.process + ["ferment"],
            ingredient.kind,
            [part if part != "impurity" else "stimulant" for part in ingredient.parts],
        )


def process_infuse(ingredient):
    if any([part for part in model.infusions if part in ingredient.parts]):
        return Ingredient(
            ingredient.key,
            ingredient.name,
            ingredient.process + ["infuse"],
            ingredient.kind,
            [model.infusions.get(part, part) for part in ingredient.parts],
        )


def collect_parts(ingredients):
    result = []
    for ingredient in ingredients:
        for part in ingredient.parts:
            result.append(part)
    return result


def simulate(ingredients):
    all_parts = list(collect_parts(ingredients))
    main_effects = [part for part in all_parts if part in model.main_effects]
    elements = [part for part in all_parts if part in model.elements]

    if len(main_effects) != 1:
        return None

    if len(elements) != 1:
        return None

    potion_kind = model.potion_kinds_map[main_effects[0]][elements[0]]

    potency_contributions = []
    for part in all_parts:
        if part == "impurity":
            potency_contributions.append((-0.5, "impurity"))
        elif part == "stimulant":
            potency_contributions.append((0.5, "stimulant"))
        elif part in elements:
            potency_contributions.append((0.5, part))
        elif part in main_effects:
            potency_contributions.append((0.5, part))

    appeal_contributions = []
    for part in all_parts:
        if part == "impurity":
            appeal_contributions.append((-0.1, "impure"))
        if part == "toxin":
            appeal_contributions.append((-0.2, "toxic"))

    potency = sum([contribution[0] for contribution in potency_contributions])
    appeal = sum([contribution[0] for contribution in appeal_contributions])
    cost = sum([len(ingredient.process) for ingredient in ingredients])

    return Recipe(
        potion_kind,
        ingredients,
        appeal,
        potency,
        cost,
        appeal_contributions,
        potency_contributions,
    )
