from pulp import LpMaximize, LpProblem, LpVariable, lpSum, LpStatus, PULP_CBC_CMD


def add_constraints(prob, recipe_vars, available_ingredients, recipes_data):
    for d, categories in recipes_data.items():
        for c, recipes in categories.items():
            prob += (
                lpSum([recipe_vars[(d, c, n)] for n, _, _ in recipes]) <= 1,
                f"AtMostOne_{d}_{c}",
            )

    for ingredient in available_ingredients.keys():
        prob += (
            lpSum(
                [
                    recipe_vars[(d, c, n)]
                    for d, categories in recipes_data.items()
                    for c, recipes in categories.items()
                    for n, ingredients, _ in recipes
                    if ingredient in ingredients
                ]
            )
            <= available_ingredients[ingredient],
            f"Ingredient_{ingredient}",
        )

    for d in recipes_data.keys():
        prob += (
            lpSum(
                [
                    recipe_vars[(d, c, n)]
                    for c, recipes in recipes_data[d].items()
                    for n, _, _ in recipes
                ]
            )
            <= 5,
            f"AtMostFive_{d}",
        )


# Data in convenient format
recipes_data = {
    "sweet": {
        "pudding": [
            ("sticky_pudding", ["sugar", "milk"], 65),
            ("toffee_pudding", ["sugar", "toffee"], 24),
        ],
        "cake": [
            ("lemon_cake", ["flour", "egg"], 44),
            ("chocolate_cake", ["cocoa", "sugar"], 71),
        ],
    },
    "savory": {
        "soup": [
            ("chicken_soup", ["chicken", "water"], 50),
            ("veggie_soup", ["carrot", "water"], 47),
            ("beef_soup", ["beef", "water"], 55),
        ],
        "salad": [
            ("caesar_salad", ["lettuce", "chicken"], 60),
            ("vegan_salad", ["lettuce", "carrot"], 45),
            ("tuna_salad", ["lettuce", "tuna"], 70),
        ],
    },
    "beverage": {
        "tea": [
            ("green_tea", ["water", "tea_leaf"], 30),
            ("black_tea", ["water", "tea_leaf"], 35),
            ("herbal_tea", ["water", "herb"], 25),
        ],
        "coffee": [
            ("latte", ["water", "coffee"], 40),
            ("americano", ["water", "coffee"], 35),
            ("cappuccino", ["water", "coffee"], 45),
        ],
    },
    "snack": {
        "chips": [
            ("potato_chips", ["potato", "oil"], 20),
            ("corn_chips", ["corn", "oil"], 25),
            ("kale_chips", ["kale", "oil"], 30),
        ],
        "nuts": [
            ("almonds", ["almond"], 50),
            ("cashews", ["cashew"], 55),
            ("walnuts", ["walnut"], 60),
        ],
    },
}


available_ingredients = {
    "sugar": 2,
    "coffee": 1,
    "milk": 1,
    "water": 1,
    "lettuce": 2,
    "chicken": 1,
    "tea_leaf": 2,
    "potato": 1,
    "oil": 2,
    "almond": 1,
}

# Create variables
recipe_vars = LpVariable.dicts(
    "Recipe",
    [
        (d, c, n)
        for d, categories in recipes_data.items()
        for c, recipes in categories.items()
        for n, _, _ in recipes
    ],
    0,
    1,
    cat="Binary",
)

# Step 1: Maximize number of unique categories
prob1 = LpProblem("Max_Categories", LpMaximize)
prob1 += (
    lpSum(
        [
            recipe_vars[(d, c, n)]
            for d, categories in recipes_data.items()
            for c, recipes in categories.items()
            for n, _, _ in recipes
        ]
    ),
    "Total_Categories",
)

# Add common constraints
add_constraints(prob1, recipe_vars, available_ingredients, recipes_data)

# Solve first problem
prob1.solve(PULP_CBC_CMD(msg=0))
if LpStatus[prob1.status] != "Optimal":
    print("No feasible solution for step 1")
    exit()

max_categories = sum(
    int(recipe_vars[(d, c, n)].varValue)
    for d, categories in recipes_data.items()
    for c, recipes in categories.items()
    for n, _, _ in recipes
)

# Step 2: Maximize score while keeping max categories
prob2 = LpProblem("Max_Score", LpMaximize)
prob2 += (
    lpSum(
        [
            recipe_vars[(d, c, n)] * s
            for d, categories in recipes_data.items()
            for c, recipes in categories.items()
            for n, _, s in recipes
        ]
    ),
    "Total_Score",
)

# Add common constraints
add_constraints(prob2, recipe_vars, available_ingredients, recipes_data)

# Constraint to keep max categories
prob2 += (
    lpSum(
        [
            recipe_vars[(d, c, n)]
            for d, categories in recipes_data.items()
            for c, recipes in categories.items()
            for n, _, _ in recipes
        ]
    )
    == max_categories,
    "Keep_Max_Categories",
)

# Solve second problem
prob2.solve(PULP_CBC_CMD(msg=0))

# Output the selected recipes
selected_recipes = [
    (d, c, n)
    for d, categories in recipes_data.items()
    for c, recipes in categories.items()
    for n, _, _ in recipes
    if recipe_vars[(d, c, n)].varValue == 1
]
print("Selected recipes:", selected_recipes)
