pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

fn allergen_by_index(index: u32) -> Allergen {
    use Allergen::*;
    match index {
        0 => Eggs,
        1 => Peanuts,
        2 => Shellfish,
        3 => Strawberries,
        4 => Tomatoes,
        5 => Chocolate,
        6 => Pollen,
        7 => Cats,
        _ => panic!("Invalid index"),
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score_bit_set(*allergen as u32)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        (0..=7)
            .filter(|idx| self.score_bit_set(*idx))
            .map(allergen_by_index)
            .collect()
    }

    fn score_bit_set(&self, bit: u32) -> bool {
        (self.score >> bit) & 1 == 1
    }
}
