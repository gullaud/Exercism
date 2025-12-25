pub struct Allergies {
    score:u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergen {
    pub fn value(&self) -> u32 {
        *self as u32
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies {score}
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        allergen.value() & self.score != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        [Allergen::Eggs,Allergen::Peanuts,Allergen::Shellfish,Allergen::Strawberries,
        Allergen::Tomatoes,Allergen::Chocolate,Allergen::Pollen,Allergen::Cats]
        .into_iter().filter(|a| self.is_allergic_to(a)).collect::<Vec<Allergen>>()
    }
}
