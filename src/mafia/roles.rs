use emojis::Emoji;

// Roles, modifiers, and items defined for Players
use crate::items::*;

struct Player {
    alignment: Alignment,
    role: Role,
    role_modifier: RoleModifier,
    item: Item,
    voted_for: String,
    voted_by: Vec<T>,
}

enum Role {
    Alchemist,
    Assassin,
    Banker,
    Baron,
    BeastMaster,
    Bureaucrat,
    Captain,
    Count,
    Detective,
    Duke,
    Ghost,
    Inquisitor,
    King,
    Mayor,
    Medium,
    Merchant,
    Patriot,
    Pyromancer,
    Sheriff,
    Spymaster,
    Squire,
    Undead,
    Undertaker,
    Warden,
}

enum RoleModifier {
    Archivist,
    Apprentice,
    Clansman,
    Collector,
    Conservationist,
    Crier,
    Deadeye,
    Duelist,
    Guildmaster,
    Resilient,
    Scribe,
    Teacher,
    Thief,
    Unstable,
    Vindictive,
    Witch,
}

enum Item {
    AcidFlask,
    Armor,
    Cloak,
    CrystalBall,
    Dagger,
    Manacles,
    MasterworkTools,
    PoisonVial,
    Shield,
    TarotCards,
}

enum Alignment {
    Kingdom(bool),
    Traitor(bool),
    ThirdParty(bool),
}

// struct Kingdom {
//     is_kingdom: bool,
// }
// impl Kingdom {
//     fn is_kingdom(&self, is_kingdom: bool) -> bool {
//         if is_kingdom == true {
//             true
//         } else {
//             false
//         }
//     }
// }
// struct Traitor {
//     is_traitor: bool,
// }
// impl Traitor {}
// struct ThirdParty {
//     is_thirdparty: bool,
// }
// impl ThirdParty {}

struct BaseRole {
    name: String,
    info: String,
    has_night_action: bool,
    has_day_action: bool,
    number_of_targets: u8,
    priority: i8,
    sends_mafia_kill: bool, // Set to enum Alignment::Traitor
    can_self_target: bool,
    shots: u8,
    //emoji: Emoji,
    weight: f32,
}
// Possible way to build roles with enum
fn build_role_index() -> Vec<BaseRole> {
    let mut role_index: Vec<BaseRole> = Vec::new();

    let mut template = BaseRole {
        name: String::from("Name"),
        info: String::from("Info goes here."),
        has_night_action: false,
        has_day_action: false,
        number_of_targets: 0,
        priority: 0,
        sends_mafia_kill: false,
        can_self_target: false,
        shots: 0,
        //emoji: emojis::get_by_shortcode("rocket"),
        weight: 0.0,
    };

    let baron = BaseRole {
        name: String::from("Baron"),
        info: String::from("Barons have a unique power in numbers. While they possess no special Abilities, the vast population of simple Barons and the unique strengths of individual players allow Barons to engage in much riskier gambits than their peers, such as subtly hinting at holding more power than they do to draw Traitorous attention away from specialized Roles, or simply freely calling out Traitorous behavior knowing that lynching them is not what the Traitors want.

        The Baron is the most versatile Role in Mafia, due to its simplicity."),
        ..template
    };
    role_index.push(baron);

    let mayor = BaseRole {
        name: String::from("Mayor"),
        info: String::from("If Mayor is in the Setup, all Players will be made aware before the Game begins.
        The Mayor is an Elected Role.
        During the Summit, all Members will vote to Elect a Mayor *instead* of voting for who to Execute. Members must select a Mayor each and every Summit, and may not vote to Stay.
        If no majority is reached by the Voting Deadline, then the Mayor will be whoever has the most votes at the Deadline. A tie is broken with whoever has the longest-tenured vote.
        When a Mayor is Elected, they are given the power to determine who is Condemned. The Mayor may also choose to Stay.
        Upon Election, the Mayor is given 24 hours (Harvest) to privately submit their selection. The Mayor may not Extend this phase.
        If the Mayor does not make a submission, the Mayor will be Executed.
        The Mayor Role is held only until the end of Harvest, and the Elected player then reverts to their previous role."),
        ..template
    };
    role_index.push(mayor);

    let alchemist = BaseRole {
        name: String::from("Alchemist"),
        info: String::from(
            "You are a skilled apothecary, with a powerful recipe for disaster.
        You may target another Summit Member during the Summit (not Harvest) to poison.
        You will anonymously poison them, and they will die shortly thereafter. 
        This will not end the Summit.",
        ),
        ..template
    };
    role_index.push(alchemist);

    let assassin = BaseRole {
        name: String::from("Assassin"),
        info: String::from(
            "You understand the human weakness, and how to exploit it.
        You may target another Summit Member per Winter.
        You will attack and murder them.",
        ),
        ..template
    };
    role_index.push(assassin);

    let banker = BaseRole {
        name: String::from("Banker"),
        info: String::from(
            "You make a living off of the thrill of guesswork.
            Each Winter, you may place a bet that one Member in particular will be
            Executed the next Summit.
            If you are correct, you are rewarded with an Item.
            This bet is considered an action, but not a target.",
        ),
        ..template
    };
    role_index.push(banker);

    let beastmaster = BaseRole {
        name: String::from("BeastMaster"),
        info: String::from(
            "You grew up among the wild beasts of nature, you have a special relationship with them.
            You may target one Summit Member per Winter to investigate. You will identify anybody they targeted."
        ),
        ..template
    };
    role_index.push(beastmaster);

    let bureaucrat = BaseRole {
        name: String::from("Bureaucrat"),
        info: String::from(
            "You have a refined patience and a hand in the cogs of the Council.
            You may target another Summit Member per Winter to block.
            You will prevent them from performing any actions.",
        ),
        ..template
    };
    role_index.push(bureaucrat);

    let captain = BaseRole {
        name: String::from("Captain"),
        info: String::from(
            "You were once the head guard of the King's Watch, and have resigned to a life of private protection.
            You may target another Summit Member per Winter to protect.
            You will save that Summit Member from one attack.
            If the Member is attacked, a coin flip determines whether you or the attacker dies."
        ),
        ..template
    };
    role_index.push(captain);

    let count = BaseRole {
        name: String::from("Count"),
        info: String::from(
            "You are among a collective of like-minded associates that works as one for all.
            All Counts are able to communicate privately during the Winter.
            You are certain that your fellow Counts are loyal to the Kingdom.",
        ),
        ..template
    };
    role_index.push(count);

    let detective = BaseRole {
        name: String::from("Detective"),
        info: String::from(
            "You are a well-travelled jack of all trades, with a strong intuition.
            You may target another Summit Member per Winter to investigate.
            You will learn their Role.",
        ),
        ..template
    };
    role_index.push(detective);

    let duke = BaseRole {
        name: String::from("Duke"),
        info: String::from(
            "For a lifetime of commitment to the Kingdom, you have been awarded a powerful right.
            You may select one Summit Member during the Summit (not Harvest) to pardon.
            If voted to die, the Execution will fail and Winter will begin.
            You will anonymously prevent their Execution, and will henceforth be unable to select a member to pardon.
            A Duke will always be loyal to the Kingdom."
        ),
        ..template
    };
    role_index.push(duke);

    let ghost = BaseRole {
        name: String::from("Ghost"),
        info: String::from(
            "Despite your Execution, you continue to haunt the living.
            While you are a Ghost, you may continue to participate in Summits, and cannot be Executed.
            However, you do not have a Vote, and you do not count toward Total Vote Counts.
            You are still able to be targeted by other Power Roles, and can be killed."
        ),
        ..template
    };
    role_index.push(ghost);

    let inquisitor = BaseRole {
        name: String::from("Inquisitor"),
        info: String::from(
            "You have a drive to lead the way in cleansing the populace. 
            You may target another Summit Member per Winter. 
            You will attack and murder them. 
            If you murder a Member that shares your Win Condition, you will no longer be able to target Members each Winter."
        ),
        ..template
    };
    role_index.push(inquisitor);

    let king = BaseRole {
        name: String::from("King"),
        info: String::from(
            "You are responsible for the well-being of your vassals.
            You may target another Summit Member per Winter to protect.
            You will save them from all attacks.",
        ),
        ..template
    };
    role_index.push(king);

    let medium = BaseRole {
        name: String::from("Medium"),
        info: String::from(
            "The spirits have always been talkative with you.
            During Winter, you may submit one “Yes or No” question, subject to Mod approval, to the Graveyard. 
            All dead players may vote Yes or No, and you will receive a tally of how many answered each option the following Winter."
        ),
        ..template
    };
    role_index.push(medium);

    let merchant = BaseRole {
        name: String::from("Merchant"),
        info: String::from(
            "You are well connected in the world market, and have amassed a collection of Items.
            At the beginning of a Game, you will be told exactly which Items you possess.
            
            You may target another Summit Member per Winter to Gift.
            You will anonymously give them an Item of your choice.
            The receiver will get a notification if the Gift resolves.
            The receiver may not use an item and an Ability in the same phase.
            You may not use your Items.
            You may give each item only once..",
        ),
        ..template
    };
    role_index.push(merchant);

    let patriot = BaseRole {
        name: String::from("Patriot"),
        info: String::from(
            "You go above and beyond with your nationalism.
            Though you are no different in Ability from a Baron, you will always be loyal to the Kingdom.
            You will be announced as a Patriot at the start of the first Summit."
        ),
        ..template
    };
    role_index.push(patriot);

    let pyromancer = BaseRole {
        name: String::from("Pyromancer"),
        info: String::from(
            "You are gifted in the arcane school of Flame.
            You may target another Summit Member per Winter to mark.
            You will anonymously place a hex them.
            During any Summit (not Harvest), you may choose to begin quietly chanting.
            Your ritual will trigger all marked Members, as well as yourself, to die in a burst of flames.
            This will not end the Summit.
            Your ritual ignition is considered an action, but not a target."
        ),
        ..template
    };
    role_index.push(pyromancer);

    let sheriff = BaseRole {
        name: String::from("Sheriff"),
        info: String::from(
            "You have a keen eye for identifying inter-personal associations.
            You may target another Summit Member per Winter to investigate.
            You will learn their Alignment.",
        ),
        ..template
    };
    role_index.push(sheriff);

    let spymaster = BaseRole {
        name: String::from("Spymaster"),
        info: String::from(
            "You are an astute observer.
            You may target one Summit Member per Winter to investigate.
            You will identify anybody who targeted them.",
        ),
        ..template
    };
    role_index.push(spymaster);

    let squire = BaseRole {
        name: String::from("Squire"),
        info: String::from(
            "You are a dedicated, yet fickle fan of your peers.
            You may target another Summit Member per Winter.
            If they are targeted by other Members, you will intercept the Actions and become the target instead."
        ),
        ..template
    };
    role_index.push(squire);

    let undead = BaseRole {
        name: String::from("Undead"),
        info: String::from(
            "While you appear normal on the outside, you know that your existence does not end at death.
            If you are Executed, your Role changes to Ghost."
        ),
        ..template
    };
    role_index.push(undead);

    let undertaker = BaseRole {
        name: String::from("Undertaker"),
        info: String::from(
            "As a gravedigger, you've spent plenty of time around dead bodies. 
            You may select one Summit Member per Winter.
            If they die before you reselect another Member, their Role and Alignment will not be revealed."
        ),
        ..template
    };
    role_index.push(undertaker);

    let warden = BaseRole {
        name: String::from("Warden"),
        info: String::from(
            "You have climbed the ranks of the Justice system, and rule your prison with an iron fist.
            You may target another Summit Member per Winter to imprison.
            Your prisoner will be prevented from performing any actions, and protected from all attacks."
        ),
        ..template
    };
    role_index.push(warden);

    let lich = BaseRole {
        name: String::from("Lich"),
        info: String::from(
            "You are impervious to any normal means of death, and appear as a Traitor if your Alignment is investigated.
            You must target another Summit Member per Winter to mark.
            You will place on them your phylactery.
            If they die, you will die as well.
            You will attack and murder anybody who targets you, except inducting Scribes or Counts.
            If you are voted for Execution, it will fail and Winter will begin."
        ),
        ..template
    };
    role_index.push(lich);

    let saboteur = BaseRole {
        name: String::from("Saboteur"),
        info: String::from(
            "You are a life-long thrill seeker, and have always just been along for the ride.
            If your Alignment is investigated, you will return No Result.
            Your goal is simply to be alive when another faction would otherwise fulfill their Win Condition.
            If you succeed in surviving until another Faction would normally have won were you not Living,
            you will murder all other living Members, and win the Game."
        ),
        ..template
    };
    role_index.push(saboteur);

    let vampire = BaseRole {
        name: String::from("Saboteur"),
        info: String::from(
            "You have only one goal: quench your thirst for blood. You care not for what the Kingdom desires,
            nor for what the Traitors fight, but will appear as Kingdom if your Alignment is investigated.
            You may target another Summit Member per Winter to attack.
            You will feast on them, killing them."
        ),
        ..template
    };
    role_index.push(vampire);

    role_index
}

// ROLE MODIFIERS
struct BaseRoleModifier {
    name: String,
    info: String,
    weight: f32,
}

fn build_role_modifiers() -> Vec<BaseRoleModifier> {
    let mut role_modifier_index: Vec<BaseRoleModifier> = Vec::new();

    let template = BaseRoleModifier {
        name: String::from("Name"),
        info: String::from("info"),
        weight: 0.0,
    };
    role_modifier_index.push(template);

    let archivist = BaseRoleModifier{
        name: String::from("Archivist"),
        info: String::from("You spend your spare time chronicling your life, and have amassed quite a collection of recorded anecdotes.
        Upon your death, you may relay a mod-approved note for all to read.
        You may submit a new note via PDC once per Winter."),
        weight: 0.5,
    };
    role_modifier_index
}
