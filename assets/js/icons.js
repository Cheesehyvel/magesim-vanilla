export default {
    icons: {
        alliance: "ui_allianceicon",
        horde: "ui_hordeicon-round",
        mage: "classicon_mage",
        warlock: "class_warlock",
        human: "achievement_character_human_female",
        gnome: "achievement_character_gnome_male",
        undead: "achievement_character_undead_male",
        troll: "achievement_character_troll_female",
        spec_arcane: "spell_holy_magicalsentry",
        spec_fire: "spell_fire_firebolt02",
        spec_frost: "spell_frost_frostbolt02",
        curse_of_elements: "spell_shadow_chilltouch",
        curse_of_shadows: "spell_shadow_curseofachimonde",
        arcane_intellect: "spell_holy_arcaneintellect",
        mage_armor: "spell_magearmor",
        divine_spirit: "spell_holy_prayerofspirit",
        motw: "spell_nature_giftofthewild",
        imp_motw: "spell_nature_regeneration",
        moonkin_aura: "spell_nature_moonglow",
        blessing_of_kings: "spell_magic_greaterblessingofkings",
        blessing_of_wisdom: "spell_holy_greaterblessingofwisdom",
        mana_spring: "spell_nature_manaregentotem",
        atiesh: "inv_staff_medivh",
        infallible_mind: "spell_ice_lament",
        dmf: "inv_misc_orb_02",
        songflower: "spell_holy_mindvision",
        rallying_cry: "inv_misc_head_dragon_01",
        warchiefs_blessing: "spell_arcane_teleportorgrimmar",
        spirit_of_zandalar: "ability_creature_poison_05",
        elixir_firepower: "inv_potion_33",
        elixir_greater_firepower: "inv_potion_60",
        elixir_arcane: "inv_potion_30",
        elixir_greater_arcane: "inv_potion_25",
        elixir_frost_power: "inv_potion_03",
        food_runn_tum: "inv_misc_food_63",
        food_nightfin: "inv_drink_17",
        food_well_fed: "spell_misc_food",
        flask_supreme_power: "inv_potion_41",
        flask_distilled_wisdom: "inv_potion_97",
        weapon_oil_brilliant_wizard: "inv_potion_105",
        weapon_oil_blessed_wizard: "inv_potion_26",
        weapon_oil_wizard: "inv_potion_104",
        weapon_oil_brilliant_mana: "inv_potion_100",
    },

    base_url: "https://wow.zamimg.com/images/wow/icons/large/",

    icon(name) {
        name = name.toLowerCase();
        if (this.icons.hasOwnProperty(name))
            name = this.icons[name];
        return this.base_url+name+".jpg";
    }
};