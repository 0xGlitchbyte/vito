struct Item {
    name: String,
    info: String,
    weight: f32,
}

fn build_item_index() -> Vec<Item> {
    let mut item_index: Vec<Item> = Vec::new();

    let mut template = Item {
        name: String::from("Name"),
        info: String::from("Info goes here."),
        weight: 0.0,
    };

    let acidflask = Item {
        name: String::from("AcidFlask"),
        info: String::from(
            "This glass jar allows the owner to select a player.
            If that player dies during the phase that the Flask was consumed,
            their Role and Alignment will not be revealed.",
        ),
        ..template
    };
    item_index.push(acidflask);

    let armor = Item {
        name: String::from("Armor"),
        info: String::from(
            "This sturdy breastplate is for personal use.
            It protects the wearer from one attack, and would resolve after any other protections.
            Consumed when triggered.",
        ),
        ..template
    };
    item_index.push(armor);

    let cloak = Item {
        name: String::from("Cloak"),
        info: String::from(
            "This silky cowl allows the owner to target another player.
            When investigated, that player will not appear as having targeted anybody.",
        ),
        ..template
    };
    item_index.push(cloak);

    let crystalball = Item {
        name: String::from("CrystalBall"),
        info: String::from(
            "This orb of glass allows the user to poll the Graveyard with one Yes or No question.",
        ),
        ..template
    };
    item_index.push(crystalball);

    let dagger = Item {
        name: String::from("Dagger"),
        info: String::from(
            "This ornate blade allows the owner to target a player to kill during Winter.",
        ),
        ..template
    };
    item_index.push(dagger);

    let manacles = Item {
        name: String::from("Manacles"),
        info: String::from(
            "These cuffs of rusty iron allows the owner to target a player.
            That player will be prevented from performing any action, and is protected from most Winter Targets,
            including all attacks."
        ),
        ..template
    };
    item_index.push(manacles);

    let masterworktools = Item {
        name: String::from("MasterworkTools"),
        info: String::from(
            "This set of painstakingly-crafted implements allows a Conservationist with no remaining uses of thier
            Ability to use it once more.
            If owned by a conservationist, they may consume the Tools as an additional use of a Conserved Ability."
        ),
        ..template
    };
    item_index.push(masterworktools);

    let poisonvial = Item {
        name: String::from("PoisonVial"),
        info: String::from(
            "This corked glass tube allows the owner to target a player during a Summit.
            That player will die shortly thereafter.
            This will not end the Summit.",
        ),
        ..template
    };
    item_index.push(poisonvial);

    let shield = Item {
        name: String::from("Shield"),
        info: String::from(
            "This ancient aegis allows the owner to target another player.
            That player cannot be killed via attack during the phase that the Shield was consumed.",
        ),
        ..template
    };
    item_index.push(shield);

    let tarotcards = Item {
        name: String::from("TarotCards"),
        info: String::from(
            "This deck covered in ancient symbols allows the owner to investigate the Alignment of another member."
        ),
        ..template
    };
    item_index.push(tarotcards);

    item_index
}
