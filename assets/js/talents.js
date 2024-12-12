export const mage = {
    class: "mage",
    icon: "https://wow.zamimg.com/images/wow/icons/medium/class_mage.jpg",
    trees: [
        {
            name: "arcane",
            icon: "https://wow.zamimg.com/images/wow/icons/medium/spell_holy_magicalsentry.jpg",
            background: "https://wow.zamimg.com/images/wow/talents/backgrounds/classic/81.jpg",
            talents: {
                rows: [[
                    {
                        name: "arcane_subtlety",
                        spellIds: [11210,12592],
                    },
                    {
                        name: "arcane_focus",
                        spellIds: [11222,12839,12840,12841,12842],
                    },
                    {
                        name: "imp_arcane_missiles",
                        spellIds: [11237,12463,12464,16769,16770],
                    },
                ], [
                    {
                        name: "wand_specialization",
                        spellIds: [6057,6085],
                    },
                    {
                        name: "magic_absorption",
                        spellIds: [29441,29444,29445,29446,29447],
                    },
                    {
                        name: "arcane_concentration",
                        spellIds: [11213,12574,12575,12576,12577],
                    },
                ], [
                    {
                        name: "magic_attunement",
                        spellIds: [11247,12606],
                    },
                    {
                        name: "imp_arcane_explosions",
                        spellIds: [11242,12467,12469],
                    },
                    {
                        name: "arcane_resilience",
                        spellIds: [28574],
                    },
                ], [
                    {
                        name: "imp_mana_shield",
                        spellIds: [11252,12605],
                    },
                    {
                        name: "imp_counterspell",
                        spellIds: [11255,12598],
                    },
                    {
                        name: "arcane_mediation",
                        spellIds: [18462,18463,18464],
                        skip: 1,
                    },
                ], [
                    {
                        name: "presence_of_mind",
                        spellIds: [12043],
                        skip: 1,
                    },
                    {
                        name: "arcane_mind",
                        spellIds: [11232,12500,12501,12502,12503],
                        requires: "arcane_resilience",
                    },
                ], [
                    {
                        name: "arcane_instability",
                        spellIds: [15058,15059,15060],
                        skip: 1,
                        requires: "presence_of_mind",
                    },
                ], [
                    {
                        name: "arcane_power",
                        spellIds: [12042],
                        skip: 1,
                        requires: "arcane_instability",
                    },
                ]],
            },
        },
        {
            name: "fire",
            icon: "https://wow.zamimg.com/images/wow/icons/medium/spell_fire_firebolt02.jpg",
            background: "https://wow.zamimg.com/images/wow/talents/backgrounds/classic/41.jpg",
            talents: {
                rows: [[
                    {
                        name: "imp_fireball",
                        spellIds: [11069,12338,12339,12340,12341],
                        skip: 1,
                    },
                    {
                        name: "impact",
                        spellIds: [11103,12357,12358,12359,12360],
                    },
                ], [
                    {
                        name: "ignite",
                        spellIds: [11119,11120,12846,12847,12848],
                    },
                    {
                        name: "flamethrowing",
                        spellIds: [11100,12353],
                    },
                    {
                        name: "imp_fire_blast",
                        spellIds: [11078,11080,12342],
                    },
                ], [
                    {
                        name: "incinerate",
                        spellIds: [18459,18460],
                    },
                    {
                        name: "imp_flamestrike",
                        spellIds: [11108,12349,12350],
                    },
                    {
                        name: "pyroblast",
                        spellIds: [11366],
                    },
                    {
                        name: "burning_soul",
                        spellIds: [11083,12351],
                    },
                ], [
                    {
                        name: "imp_scorch",
                        spellIds: [11095,12872,12873],
                    },
                    {
                        name: "imp_fire_ward",
                        spellIds: [11094,13043],
                    },
                    {
                        name: "master_of_elements",
                        spellIds: [29074,29075,29076],
                        skip: 1,
                    },
                ], [
                    {
                        name: "critical_mass",
                        spellIds: [11115,11367,11368],
                        skip: 1,
                    },
                    {
                        name: "blast_wave",
                        spellIds: [11113],
                        requires: "pyroblast",
                    },
                ], [
                    {
                        name: "fire_power",
                        spellIds: [11124,12378,12398,12399,12400],
                        skip: 2,
                    },
                ], [
                    {
                        name: "combustion",
                        spellIds: [11129],
                        requires: "critical_mass",
                        skip: 1,
                    },
                ]],
            }
        },
        {
            name: "frost",
            icon: "https://wow.zamimg.com/images/wow/icons/medium/spell_frost_frostbolt02.jpg",
            background: "https://wow.zamimg.com/images/wow/talents/backgrounds/classic/61.jpg",
            talents: {
                rows: [[
                    {
                        name: "frost_wardning",
                        spellIds: [11189,28332],
                    },
                    {
                        name: "imp_frostbolt",
                        spellIds: [11070,12473,16763,16765,16766],
                    },
                    {
                        name: "elemental_precision",
                        spellIds: [29438,29439,29440],
                    },
                ], [
                    {
                        name: "ice_shards",
                        spellIds: [11207,12672,15047,15052,15053],
                    },
                    {
                        name: "frostbite",
                        spellIds: [11071,12496,12497],
                    },
                    {
                        name: "imp_frost_nova",
                        spellIds: [11165,12475],
                    },
                    {
                        name: "permafrost",
                        spellIds: [11175,12569,12571],
                    },
                ], [
                    {
                        name: "piercing_ice",
                        spellIds: [11151,12952,12953],
                    },
                    {
                        name: "coldsnap",
                        spellIds: [12472],
                    },
                    {
                        name: "imp_blizzard",
                        spellIds: [11185,12487,12488],
                        skip: 1,
                    },
                ], [
                    {
                        name: "arctic_reach",
                        spellIds: [16757,16758],
                    },
                    {
                        name: "frost_channeling",
                        spellIds: [11160,12518,12519],
                    },
                    {
                        name: "shatter",
                        spellIds: [11170,12982,12983,12984,12985],
                        requires: "imp_frost_nova",
                    },
                ], [
                    {
                        name: "ice_block",
                        spellIds: [11958],
                        skip: 1,
                    },
                    {
                        name: "imp_cone_of_cold",
                        spellIds: [11190,12489,12490],
                    },
                ], [
                    {
                        name: "winters_chill",
                        spellIds: [11180,28592,28593,28594,28595],
                        skip: 2,
                    },
                ], [
                    {
                        name: "ice_barrier",
                        spellIds: [11426],
                        requires: "ice_block",
                        skip: 1,
                    },
                ]],
            }
        }
    ],
};