using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

struct Recipe {
    public List<string> ingredients;
    public List<string> allergens;

    public Recipe(List<string> ingredients, List<string> allergens) {
        this.ingredients = ingredients;
        this.allergens = allergens;
    }
}

class Day21 {
    public static void Main() {
        var recipes = readRecipes();
        var allergens = getAllergens(recipes);

        Console.WriteLine("Part 1: " + part1(recipes, allergens));
        Console.WriteLine("Part 2: " + part2(allergens));
    }

    private static long part1(List<Recipe> recipes, List<string> allergens) {
        List<string> ingredients = recipes.Select(recipe => recipe.ingredients).SelectMany(ingredients => ingredients).ToList();
        List<string> nonAllergens = ingredients.Distinct().Except(allergens).ToList();
        return ingredients.Count(ingredient => nonAllergens.Contains(ingredient));
    }

    private static string part2(List<string> allergens) {
        return string.Join(',', allergens);
    }

    private static List<Recipe> readRecipes() {
        return File.ReadLines(@"input.txt")
            .Select(line => line.TrimEnd(')').Split(" (contains "))
            .Select(line => new Recipe(line[0].Split(' ').ToList(), line[1].Split(", ").ToList()))
            .ToList();
    }

    private static List<string> getAllergens(List<Recipe> recipes) {
        var allergenOptions = new Dictionary<string, HashSet<string>>();
        foreach (var recipe in recipes) {
            recipe.allergens.ForEach(allergen => {
                allergenOptions.TryAdd(allergen, new HashSet<string>(recipe.ingredients));
                allergenOptions[allergen].IntersectWith(recipe.ingredients);
            });
        }
        var allergens = new SortedDictionary<string, string>();
        while (allergenOptions.Count > 0) {
            var allergen = allergenOptions.First(allergenMap => allergenMap.Value.Count == 1);
            allergens.Add(allergen.Key, allergen.Value.First());
            allergenOptions.Remove(allergen.Key);
            allergenOptions.Values.ToList().ForEach(ingredients => ingredients.Remove(allergen.Value.First()));
        }
        return allergens.Values.ToList();
    }
}
