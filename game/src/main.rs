use  game::sprites::{hero::Hero, monster::{Monster, createMonster, MonsterStruct}};

fn main() {
    let hero = Hero {
        name: String::from("sultan"),
        hp: 32,
        mp: 32,
        speed: 32,
        skill_ids: vec![1, 2, 3]
    };

    createMonster(Monster::PeiQi(MonsterStruct {
        name: String::from("peiqi"),
        hp: 300,
        mp: 100,
        speed: 7,
        skill_ids: vec![1000, 1001]
    }));


    let first_skill = &hero.skill_ids[0];

    match hero.skill_ids.get(1) {
        Some(value) => println!("the second skill is {}", value),
        None => println!("The second value is none")
    };

    println!("{} - {}", hero.name, first_skill)
}