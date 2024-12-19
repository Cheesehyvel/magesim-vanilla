import apl from "./apl";
import common from "./common";
import _ from "lodash";

const defaultApls = () => {
    let data = [];
    let item, cond;

    let manaCds = [];
    item = apl.item();
    item.condition.condition_type = apl.condition_type.CMP;
    item.condition.op = apl.condition_op.GTE;
    item.condition.values = [apl.value(), apl.value()];
    item.condition.values[0].value_type = apl.value_type.PLAYER_MANA_DEFICIT;
    item.condition.values[1].value_type = apl.value_type.CONST;
    item.condition.values[1].vfloat = 2250;
    item.action = apl.getAction("ManaPotion");
    manaCds.push(item);

    item = apl.item();
    item.condition.condition_type = apl.condition_type.AND;
    cond = apl.condition();
    cond.condition_type = apl.condition_type.CMP;
    cond.op = apl.condition_op.GTE;
    cond.values = [apl.value(), apl.value()];
    cond.values[0].value_type = apl.value_type.PLAYER_MANA_DEFICIT;
    cond.values[1].value_type = apl.value_type.CONST;
    cond.values[1].vfloat = 1200;
    item.condition.conditions.push(cond);
    cond = apl.condition();
    cond.condition_type = apl.condition_type.TRUE;
    cond.values = [apl.value()];
    cond.values[0].value_type = apl.value_type.PLAYER_COOLDOWN_EXISTS;
    cond.values[0].vint = common.cooldowns.MANA_POTION;
    item.condition.conditions.push(cond);
    item.action = apl.getAction("ManaGem");
    manaCds.push(item);

    item = apl.item();
    item.condition.condition_type = apl.condition_type.AND;
    cond = apl.condition();
    cond.condition_type = apl.condition_type.CMP;
    cond.op = apl.condition_op.GTE;
    cond.values = [apl.value(), apl.value()];
    cond.values[0].value_type = apl.value_type.PLAYER_MANA_DEFICIT;
    cond.values[1].value_type = apl.value_type.CONST;
    cond.values[1].vfloat = 625;
    item.condition.conditions.push(cond);
    cond = apl.condition();
    cond.condition_type = apl.condition_type.TRUE;
    cond.values = [apl.value()];
    cond.values[0].value_type = apl.value_type.PLAYER_COOLDOWN_EXISTS;
    cond.values[0].vint = common.cooldowns.MANA_GEM;
    item.condition.conditions.push(cond);
    item.action = apl.getAction("RobeArchmage");
    manaCds.push(item);

    item = apl.item();
    item.condition.condition_type = apl.condition_type.CMP;
    item.condition.op = apl.condition_op.LT;
    item.condition.values = [apl.value(), apl.value()];
    item.condition.values[0].value_type = apl.value_type.PLAYER_MANA_PERCENT;
    item.condition.values[1].value_type = apl.value_type.CONST;
    item.condition.values[1].vfloat = 10;
    item.action = apl.getAction("Evocation");
    manaCds.push(item);

    let cds = [
        apl.getAction("ArcanePower"),
        apl.getAction("Combustion"),
        apl.getAction("EssenceOfSapphiron"),
        apl.getAction("UnstablePower"),
        apl.getAction("EphemeralPower"),
        apl.getAction("ChaosFire"),
        apl.getAction("MindQuickening"),
    ];

    let frost = apl.apl();
    frost.id = "preset-frost";
    frost.name = "Frost";
    item = apl.item();
    item.condition.condition_type = apl.condition_type.CMP;
    item.condition.op = apl.condition_op.LT;
    item.condition.values = [apl.value(), apl.value()];
    item.condition.values[0].value_type = apl.value_type.SIM_TIME;
    item.condition.values[1].value_type = apl.value_type.CONST;
    item.condition.values[1].vfloat = 1.4;
    item.action = apl.getAction("Sequence");
    item.action.sequence = [
        apl.getAction("Frostbolt"),
        apl.getAction("PowerInfusion"),
        ...cds
    ];
    frost.items.push(item);
    frost.items = [...frost.items, ...manaCds];
    item = apl.item();
    item.action = apl.getAction("Sequence");
    item.condition.condition_type = apl.condition_type.CMP;
    item.condition.op = apl.condition_op.GT;
    item.condition.values = [apl.value(), apl.value()];
    item.condition.values[0].value_type = apl.value_type.SIM_TIME;
    item.condition.values[1].value_type = apl.value_type.CONST;
    item.condition.values[1].vfloat = 90;
    item.action.sequence = cds;
    frost.items.push(item);
    item = apl.item();
    item.action = apl.getAction("Frostbolt");
    frost.items.push(item);
    data.push(frost);

    let fire = apl.apl();
    fire.id = "preset-fire";
    fire.name = "Fire";
    item = apl.item();
    item.condition.condition_type = apl.condition_type.CMP;
    item.condition.op = apl.condition_op.LT;
    item.condition.values = [apl.value(), apl.value()];
    item.condition.values[0].value_type = apl.value_type.SIM_TIME;
    item.condition.values[1].value_type = apl.value_type.CONST;
    item.condition.values[1].vfloat = 1.4;
    item.action = apl.getAction("Sequence");
    item.action.sequence = [
        apl.getAction("Scorch"),
        apl.getAction("Scorch"),
        apl.getAction("Frostbolt"),
        apl.getAction("PowerInfusion"),
        ...cds.slice(1)
    ];
    fire.items.push(item);
    fire.items = [...fire.items, ...manaCds];
    item = apl.item();
    item.action = apl.getAction("Sequence");
    item.condition.condition_type = apl.condition_type.CMP;
    item.condition.op = apl.condition_op.GT;
    item.condition.values = [apl.value(), apl.value()];
    item.condition.values[0].value_type = apl.value_type.SIM_TIME;
    item.condition.values[1].value_type = apl.value_type.CONST;
    item.condition.values[1].vfloat = 90;
    item.action.sequence = cds;
    fire.items.push(item);
    item = apl.item();
    item.action = apl.getAction("Fireball");
    fire.items.push(item);
    data.push(fire);

    let f2 = _.cloneDeep(fire);
    f2.name = "Fire - delayed combustion"
    f2.id = "preset-fire-d1";
    f2.items[0].action.sequence.splice(3, 1); // Remove combustion
    item = apl.item();
    item.condition.condition_type = apl.condition_type.CMP;
    item.condition.op = apl.condition_op.GT;
    item.condition.values = [apl.value(), apl.value()];
    item.condition.values[0].value_type = apl.value_type.SIM_TIME;
    item.condition.values[1].value_type = apl.value_type.CONST;
    item.condition.values[1].vfloat = 10.0;
    item.action = apl.getAction("Combustion");
    f2.items.splice(1, 0, item);
    data.push(f2);

    let blank = apl.apl();
    blank.id = "preset-blank";
    blank.name = "Blank";
    data.push(blank);

    return data;
};


export default {
    apls: defaultApls(),
    talents: [
        { name: "Fire", talents: common.parseWowheadTalents("230005230002-5052000123033151-003") },
        { name: "Frost Arcane", talents: common.parseWowheadTalents("2300250310231531--053500030013") },
        { name: "Frost WC", talents: common.parseWowheadTalents("230045200003--05350013122301051") },
    ],
    loadouts: [{
        name: "Naxxramas",
        loadout: {
            head: { item_id: 22498, enchant_id: 24164 },
            neck: { item_id: 23057, enchant_id: null },
            shoulder: { item_id: 22983, enchant_id: 29467 },
            back: { item_id: 23050, enchant_id: null },
            chest: { item_id: 22496, enchant_id: 20025 },
            wrist: { item_id: 21186, enchant_id: 20008 },
            hands: { item_id: 21585, enchant_id: 25078 },
            waist: { item_id: 22730, enchant_id: null },
            legs: { item_id: 23070, enchant_id: 24164 },
            feet: { item_id: 22500, enchant_id: 13890 },
            finger1: { item_id: 21709, enchant_id: null },
            finger2: { item_id: 23062, enchant_id: null },
            trinket1: { item_id: 23207, enchant_id: null },
            trinket2: { item_id: 23046, enchant_id: null },
            main_hand: { item_id: 22589, enchant_id: 22749 },
            off_hand: { item_id: null, enchant_id: null },
            ranged: { item_id: 22821, enchant_id: null }
        }
    }, {
        name: "Molten Core",
        loadout: {
            head: { item_id: 16795, enchant_id: 22844 },
            neck: { item_id: 18814, enchant_id: null },
            shoulder: { item_id: 11782, enchant_id: null },
            back: { item_id: 17078, enchant_id: null },
            chest: { item_id: 14152, enchant_id: 20025 },
            wrist: { item_id: 16799, enchant_id: 20008 },
            hands: { item_id: 16801, enchant_id: null },
            waist: { item_id: 19136, enchant_id: null },
            legs: { item_id: 16915, enchant_id: 22844 },
            feet: { item_id: 16800, enchant_id: 13890 },
            finger1: { item_id: 19147, enchant_id: null },
            finger2: { item_id: 19147, enchant_id: null },
            trinket1: { item_id: 18820, enchant_id: null },
            trinket2: { item_id: 12930, enchant_id: null },
            main_hand: { item_id: 17103, enchant_id: 22749 },
            off_hand: { item_id: "10796:1965", enchant_id: null },
            ranged: { item_id: "15283:1959", enchant_id: null }
        }
    }],
};