from collections import namedtuple

Ingredient = namedtuple(
    "Ingredient",
    ["key", "name", "process", "kind", "parts"],
)
PotionKind = namedtuple(
    "PotionKind",
    ["key", "name", "department", "parts"],
)
Recipe = namedtuple(
    "Recipe",
    [
        "potion_kind",
        "ingredients",
        "appeal",
        "potency",
        "cost",
        "appeal_contributions",
        "potency_contributions",
    ],
)

main_effects = {"cat", "bone", "soul", "beast"}
elements = {"fire", "aether", "water", "earth"}

infusions = {
    "aether": "earth",
    "earth": "aether",
    "fire": "water",
    "water": "fire",
}

raw_ingredients = [
    Ingredient(
        "catnip",
        "Catnip",
        ["raw"],
        "herb",
        ["stimulant", "impurity", "cat", "tasty"],
    ),
    Ingredient(
        "lupine",
        "Lupine",
        ["raw"],
        "herb",
        ["toxin", "tasty", "fire", "stimulant"],
    ),
    Ingredient(
        "mandrake",
        "Mandrake",
        ["raw"],
        "herb",
        ["unsavory", "stimulant", "bitter", "bone"],
    ),
    Ingredient(
        "nightshade",
        "Nightshade",
        ["raw"],
        "herb",
        ["toxin", "aether", "stimulant", "water"],
    ),
    Ingredient(
        "sage",
        "Sage",
        ["raw"],
        "herb",
        ["water", "tasty", "impurity", "sweet"],
    ),
    Ingredient(
        "thyme",
        "Thyme",
        ["raw"],
        "herb",
        ["tasty", "stimulant", "impurity", "cat"],
    ),
    Ingredient(
        "wormwood",
        "Wormwood",
        ["raw"],
        "herb",
        ["fire", "antitoxin", "bitter", "earth"],
    ),
    Ingredient(
        "deadmans",
        "Dead Man's Finger",
        ["raw"],
        "mushroom",
        ["stimulant", "soul", "toxin", "bitter"],
    ),
    Ingredient(
        "deathcap",
        "Death Cap",
        ["raw"],
        "mushroom",
        ["impurity", "stimulant", "toxin", "earth"],
    ),
    Ingredient(
        "elven",
        "Elven Saddle",
        ["raw"],
        "mushroom",
        ["earth", "stimulant", "water", "antitoxin"],
    ),
    Ingredient(
        "flyagaric",
        "Fly Agaric",
        ["raw"],
        "mushroom",
        ["stimulant", "toxin", "beast", "tasty"],
    ),
    Ingredient(
        "pluteus",
        "Pluteus",
        ["raw"],
        "mushroom",
        ["toxin", "fire", "stimulant", "aether"],
    ),
    Ingredient(
        "wizards",
        "Wizard's Hat",
        ["raw"],
        "mushroom",
        ["impurity", "aether", "unsavory", "stimulant"],
    ),
    Ingredient(
        "asporeus",
        "Asporeus",
        ["raw"],
        "mushroom",
        ["beast", "unsavory", "stimulant", "impurity"],
    ),
]

raw_ingredients_map = {ingredient.key: ingredient for ingredient in raw_ingredients}

departments = ["health", "sourcery", "provisions"]

potion_kinds = [
    PotionKind("speed", "Speed Potion", "health", ["cat", "fire"]),
    PotionKind("slow", "Potion of Slow", "provisions", ["cat", "water"]),
    PotionKind("mana", "Mana Potion", "sourcery", ["cat", "aether"]),
    PotionKind("warding", "Warding Potion", "sourcery", ["cat", "earth"]),
    PotionKind("strength", "Strength Potion", "health", ["bone", "fire"]),
    PotionKind("weakness", "Potion of Weakness", "provisions", ["bone", "water"]),
    PotionKind("necromancy", "Necromancy Potion", "sourcery", ["bone", "aether"]),
    PotionKind("skelleton", "Skelleton Repellent", "provisions", ["bone", "earth"]),
    PotionKind("speech", "Speech Potion", "health", ["soul", "fire"]),
    PotionKind("silence", "Potion of Silence", "provisions", ["soul", "water"]),
    PotionKind("conjuring", "Conjuring Potion", "sourcery", ["soul", "aether"]),
    PotionKind("exorcism", "Exorcism Potion", "sourcery", ["soul", "earth"]),
    PotionKind("vitality", "Potion of Vitality", "health", ["beast", "fire"]),
    PotionKind("sleep", "Sleep Potion", "provisions", ["beast", "water"]),
    PotionKind("summoning", "Potion of Summoning", "sourcery", ["beast", "aether"]),
    PotionKind("monster", "Monster Repellent", "sourcery", ["beast", "earth"]),
]

potion_kinds_map = {
    potion.parts[0]: {
        inner_potion.parts[1]: inner_potion
        for inner_potion in potion_kinds
        if inner_potion.parts[0] == potion.parts[0]
    }
    for potion in potion_kinds
}

raw_ingredient_names_map = {
    ingredient.key: ingredient.name for ingredient in raw_ingredients
}

department_names_map = {
    "health": "Health",
    "sourcery": "Sourcery",
    "provisions": "Provisions",
}
