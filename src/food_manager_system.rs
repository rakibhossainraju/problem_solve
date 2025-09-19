use std::collections::{BTreeSet, HashMap};
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Clone)]
struct FoodEntry {
    rating: i32,
    name: String,
}

impl Ord for FoodEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Sort by rating DESC
        match self.rating.cmp(&other.rating).reverse() {
            Ordering::Equal => self.name.cmp(&other.name), // tie → lexicographically smaller
            ord => ord,
        }
    }
}

impl PartialOrd for FoodEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct FoodRatings {
    food_to_cuisine: HashMap<String, String>,       // food -> cuisine
    food_to_rating: HashMap<String, i32>,           // food -> rating
    cuisines: HashMap<String, BTreeSet<FoodEntry>>, // cuisine -> sorted foods
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut food_to_cuisine = HashMap::new();
        let mut food_to_rating = HashMap::new();
        let mut cuisine_map: HashMap<String, BTreeSet<FoodEntry>> = HashMap::new();

        for ((food, cuisine), rating) in foods.into_iter().zip(cuisines).zip(ratings) {
            food_to_cuisine.insert(food.clone(), cuisine.clone());
            food_to_rating.insert(food.clone(), rating);

            cuisine_map
                .entry(cuisine)
                .or_insert_with(BTreeSet::new)
                .insert(FoodEntry {
                    rating,
                    name: food,
                });
        }

        Self {
            food_to_cuisine,
            food_to_rating,
            cuisines: cuisine_map,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        let cuisine = self.food_to_cuisine.get(&food).unwrap().clone();
        let old_rating = *self.food_to_rating.get(&food).unwrap();

        // Remove old entry
        self.cuisines
            .get_mut(&cuisine)
            .unwrap()
            .remove(&FoodEntry {
                rating: old_rating,
                name: food.clone(),
            });

        // Insert new entry
        self.cuisines
            .get_mut(&cuisine)
            .unwrap()
            .insert(FoodEntry {
                rating: new_rating,
                name: food.clone(),
            });

        // Update rating map
        self.food_to_rating.insert(food, new_rating);
    }

    fn highest_rated(&self, cuisine: String) -> String {
        self.cuisines
            .get(&cuisine)
            .unwrap()
            .iter()
            .next()
            .unwrap()
            .name
            .clone()
    }
}