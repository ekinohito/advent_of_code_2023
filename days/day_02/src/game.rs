use std::str::FromStr;

pub struct Game {
    pub id: u64,
    min_red: u64,
    min_green: u64,
    min_blue: u64,
}

impl Game {
    pub fn is_possible(&self, red: u64, green: u64, blue: u64) -> bool {
        let Self {min_red, min_green, min_blue, ..} = self;
        *min_red <= red && *min_green <= green && *min_blue <= blue
    }

    pub fn power(&self) -> u64 {
        let Self {min_red, min_green, min_blue, ..} = self;
        min_red * min_green * min_blue
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_id, tail) = s.split_once(':').ok_or(())?;
        let (_, id) = game_id.split_once(' ').ok_or(())?;
        let id = id.parse().or(Err(()))?;
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for outcome in tail.split(';') {
            for part in outcome.split(',') {
                let (quantity, color) = part.trim().split_once(' ').ok_or(())?;
                let quantity: u64 = quantity.parse().or(Err(()))?;
                let color = match color {
                    "red" => &mut min_red,
                    "green" => &mut min_green,
                    "blue" => &mut min_blue,
                    _ => return Err(()),
                };
                *color = quantity.max(*color);
            }
        }
        Ok(Self {id, min_red, min_green, min_blue})
    }
}