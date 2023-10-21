use std::collections::HashSet;

/// uniquely identifies a piece in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Id(u8);

#[derive(Debug, Clone, Copy)]
enum Color{
    Red = 0,
    Green = 1,
    Blue = 2,
    Yellow = 3,
}

#[derive(Debug, Clone, Copy)]
enum Size{
    Small = 0,
    Medium = 1,
    Large = 2,
}

#[derive(Debug, Clone, Copy)]
enum Action{
    CreateShip{
        ship: Id,
        star: Id,
    },
    DestroyShip{
        ship: Id,
        star: Id,
    },
    MoveShip{
        ship: Id,
        start_star: Id,
        end_star: Id,
    },
    CreateStar{
        star: Id,
    },
    DestroyStar{
        star: Id,
    },
}

#[derive(Debug, Clone)]
struct Star{
    star: Id,
    ships: Box<[Vec<Id>]>,
}
#[derive(Debug, Clone)]
struct HomeWorld{
    stars: Vec<Id>,
    ships: Box<[Vec<Id>]>,
}

#[derive(Debug, Clone)]
struct Board{
    current_player: u8,
    player_count: u8,
    bank: HashSet<Id>,
    stars: Vec<Star>,
    homeworlds: Box<[HomeWorld]>,
}

enum Either<A,B>{
    A(A),
    B(B),
}


impl Board{
    fn star_mut(&mut self, star: Id)->Option<Either<&mut Star, &mut HomeWorld>>{
        if let Some(hw) = self.homeworlds.iter_mut().find(|hw|hw.stars.contains(&star)){
            Some(Either::B(hw))
        }else if let Ok(i) = self.stars.binary_search_by_key(&star, |system|system.star){
            Some(Either::A(&mut self.stars[i]))
        }else{
            None
        }
    }
    fn apply_action(mut self, action: Action) -> Option<Self>{
        let player_i = self.current_player as usize;
        match action{
            Action::CreateShip { ship, star } => {
                let ship = self.bank.take(&ship)?;
                match self.star_mut(star)? {
                    Either::A(star) => {
                        star.ships[player_i].push(ship);
                    },
                    Either::B(hw) => {
                        hw.ships[player_i].push(ship);
                    },
                }
            },
            Action::DestroyShip { ship, star } => {
                let ships = match self.star_mut(star)?{
                    Either::A(star) => &mut star.ships[player_i],
                    Either::B(hw) => &mut hw.ships[player_i],
                };
                let i = ships.iter()
                    .enumerate()
                    .find_map(|(i,sh)|if *sh == ship {Some(i)}else{None})?;
                let ship = ships.remove(i);
                self.bank.insert(ship);
            },
            Action::MoveShip { ship, start_star, end_star } => {
                self = self.apply_action(Action::DestroyShip { ship, star: start_star })?;
                self = self.apply_action(Action::CreateShip { ship, star: end_star })?;
            },
            Action::CreateStar { star } => {
                let star = self.bank.take(&star)?;
                self.stars.push(Star { star, ships: Default::default() })
            },
            Action::DestroyStar { star } => {
                if let Ok(i) = self.stars.binary_search_by_key(&star, |system|system.star){
                    let star = self.stars.remove(i);
                    for player_ships in star.ships.into_iter(){
                        for ship in player_ships{
                            self.bank.insert(*ship);
                        }
                    }
                    self.bank.insert(star.star);
                }else if let Some(hw) =
                    self.homeworlds.iter_mut().find(|hw|hw.stars.contains(&star))
                {
                    let i = hw.stars.iter()
                        .enumerate()
                        .find_map(|(i,st)|if *st == star {Some(i)}else{None})?;
                    hw.stars.remove(i);
                    self.bank.insert(star);
                    if hw.stars.is_empty(){
                        for player_ships in hw.ships.into_iter(){
                            for ship in player_ships{
                                self.bank.insert(*ship);
                            }
                        }
                    }
                }
            },
        }
        Some(self)
    }
}
