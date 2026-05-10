#[test]
fn run() {
    let mut cs: Vec<GameCharacter> = Vec::new();
    cs.push({
        GameCharacter {
            class: Class::Warrior,
            age: 23,
            weapon: Some(Weapon::Warrior(WarriorWeapon {
                shield: { Shield { damage: 3 } },
                sword: { Sword { damage: 100 } },
            })),
        }
    });

    cs.push({
        GameCharacter {
            class: Class::Paladin,
            age: 19,
            weapon: None,
        }
    });

    cs.push({
        GameCharacter {
            class: Class::Healer,
            age: 18,
            weapon: None,
        }
    });

    cs.push({
        GameCharacter {
            class: Class::Goblin,
            age: 200,
            weapon: None,
        }
    });

    cs.push({
        GameCharacter {
            class: Class::Archer,
            age: 40,
            weapon: None,
        }
    });

    cs.push({
        GameCharacter {
            class: Class::Elf,
            age: 90,
            weapon: Some(Weapon::Elf(ElfWeapon {
                staff: Staff { damage: 30 },
            })),
        }
    });

    for c in cs {
        println!("======================================");
        println!("Class: {}", c.title());
        println!("Age: {}", c.age);
        println!("Hit: {}", c.hit());
    }
}

struct GameCharacter {
    class: Class,
    age: i32,
    weapon: Option<Weapon>,
}

impl GameCharacter {
    fn hit(&self) -> i32 {
        match &self.weapon {
            Some(Weapon::Warrior(WarriorWeapon { shield, sword })) => sword.damage / shield.damage,
            Some(Weapon::Elf(ElfWeapon { staff })) => staff.damage,
            None => 0,
        }
    }
}

impl GameCharacter {
    fn title(&self) -> &str {
        match self.class {
            Class::Warrior => "⚔️  Warrior",
            Class::Paladin => "☀️  Paladin",
            Class::Healer => "💚 Healer",
            Class::Goblin => "👺 Goblin",
            Class::Archer => "🎯 Archer",
            Class::Elf => "🍃 Elf",
        }
    }
}

enum Class {
    Warrior,
    Paladin,
    Healer,
    Goblin,
    Archer,
    Elf,
}

struct Sword {
    damage: i32,
}

struct Staff {
    damage: i32,
}

struct Shield {
    damage: i32,
}

struct WarriorWeapon {
    sword: Sword,
    shield: Shield,
}

struct ElfWeapon {
    staff: Staff,
}

enum Weapon {
    Warrior(WarriorWeapon),
    Elf(ElfWeapon),
}
