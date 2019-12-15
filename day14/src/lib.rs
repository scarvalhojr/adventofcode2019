use std::collections::HashMap;
use std::str::FromStr;

const FUEL: &str = "FUEL";
const ORE: &str = "ORE";

#[derive(Clone, Debug)]
struct Ingredient {
    quantity: u64,
    chemical: String,
}

#[derive(Debug)]
pub struct Reaction {
    input: Vec<Ingredient>,
    chemical: String,
    quantity: u64,
}

struct NanoFactory {
    reactions: HashMap<String, (u64, Vec<Ingredient>)>,
}

impl From<&[Reaction]> for NanoFactory {
    fn from(reaction_list: &[Reaction]) -> Self {
        let reactions = reaction_list
            .iter()
            .map(|r| (r.chemical.clone(), (r.quantity, r.input.clone())))
            .collect();
        Self { reactions }
    }
}

impl NanoFactory {
    fn min_ore_per_unit_of_fuel(&self, units: i64) -> Option<u64> {
        let mut total_ore = 0;
        let mut needed: HashMap<String, i64> = [(FUEL.to_string(), units)].iter().cloned().collect();
        loop {
            let chem_needed;
            let qty_needed;
            if let Some((chem, qty)) = needed.iter().filter(|&(_, q)| *q > 0).next() {
                chem_needed = chem.clone();
                qty_needed = *qty;
            } else {
                break;
            }
            needed.remove(&chem_needed);
            // println!("Finding how to produce {} of {}", qty_needed, chem_needed);

            if let Some((quantity, ingredients)) = self.reactions.get(&chem_needed) {
                let multiplier = (qty_needed as f32 / *quantity as f32).ceil() as u64;
                // println!("Multiplier: {}", multiplier);
                let excess = (multiplier * quantity) as i64 - qty_needed;
                if excess != 0 {
                    // println!("Excess {} of {}", -excess, chem_needed);
                    needed.insert(chem_needed.clone(), -excess);
                }
                for ingredient in ingredients {
                    let chem_qty = multiplier * ingredient.quantity;
                    if ingredient.chemical == ORE {
                        // println!("=> Need {} of ORE", chem_qty);
                        total_ore += chem_qty;
                        // dbg!(total_ore);
                    } else {
                        // println!("* Need {} of {}", chem_qty, ingredient.chemical);
                        needed
                            .entry(ingredient.chemical.clone())
                            .and_modify(|q| {*q += chem_qty as i64})
                            .or_insert(chem_qty as i64);
                    }
                }
            } else {
                return None;
            }
        }
        Some(total_ore)
    }
}

pub fn part1(reactions: &[Reaction]) -> u64 {
    let factory = NanoFactory::from(reactions);
    factory.min_ore_per_unit_of_fuel(1).unwrap_or(0)
}

pub fn part2(reactions: &[Reaction]) -> i64 {
    let factory = NanoFactory::from(reactions);
    let mut units = 1;
    loop {
        if factory.min_ore_per_unit_of_fuel(units).unwrap() > 1_000_000_000_000 {
            break;
        }
        units += 1;
        if units % 100_000 == 0 {
            println!("Units {}", units);
        }
    }
    units - 1
}

impl FromStr for Ingredient {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split_whitespace();
        let quantity = parts
            .next()
            .ok_or_else(|| "Missing quantity".to_string())
            .and_then(|qty| {
                qty.trim()
                    .parse()
                    .map_err(|_| "Invalid quantity".to_string())
            })?;
        let chemical = parts
            .next()
            .map(|chem| chem.trim().to_string())
            .ok_or_else(|| "Missing chemical".to_string())?;
        Ok(Self { quantity, chemical })
    }
}

impl FromStr for Reaction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("=>");
        let input = parts
            .next()
            .ok_or_else(|| "Missing input".to_string())
            .and_then(|ingredients| {
                ingredients
                    .split(',')
                    .map(|ingredient| ingredient.parse())
                    .collect::<Result<Vec<_>, _>>()
            })?;
        let output = parts
            .next()
            .ok_or_else(|| "Missing output".to_string())
            .and_then(|ingredient| ingredient.trim().parse::<Ingredient>())?;
        Ok(Self { input, chemical: output.chemical, quantity: output.quantity })
    }
}
