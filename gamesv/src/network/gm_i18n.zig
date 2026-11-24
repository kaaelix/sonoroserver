const std = @import("std");
const Allocator = std.mem.Allocator;

pub fn isAsciiPrintable(s: []const u8) bool {
    for (s) |c| {
        if (c < 0x20 or c >= 0x7F) return false;
    }
    return true;
}

pub fn questTypeName(quest_type: i32) []const u8 {
    return switch (quest_type) {
        1 => "Main Story",
        2 => "World / Side",
        3 => "Companion",
        4 => "Daily / Side",
        7 => "Exploration / Challenge",
        9 => "Exploration (POI)",
        10 => "Event",
        11 => "Logistics / Courier",
        14 => "Character Encounter",
        15 => "Event Replay",
        16 => "Collab Story",
        17 => "Domain Exploration",
        else => "Quest",
    };
}

pub fn regionName(region_id: i32) []const u8 {
    return switch (region_id) {
        1 => "Gorges of Spirits",
        2 => "Jinzhou",
        3 => "Central Plains",
        4 => "Desorock Highland",
        5 => "Dim Forest",
        6 => "Huanglong",
        7 => "Port City of Guixu",
        8 => "Wuming Bay",
        9 => "Whining Aix's Mire",
        10 => "Norfall Barrens",
        11 => "The Black Shores",
        12 => "Mt. Firmament",
        13 => "Penitent's End",
        14 => "Ragunna / Rinascita",
        15 => "Nimbus Sanctum",
        16 => "Riccioli Islands",
        17 => "Thessaleo Fells",
        18 => "Averardo Vault",
        19 => "Avinoleum",
        20 => "Vault Underground",
        21 => "Riccioli Voyage",
        22 => "Tethys System",
        23 => "Rinascita Isles",
        24 => "Septimont",
        25 => "Fabricatorium of the Deep",
        26 => "Lahai-Roi",
        27 => "Encounter Zone",
        28 => "Xuanfang City / Mengzhou",
        100 => "Roguelike / Instance",
        else => "Solaris-3",
    };
}

pub fn chapterName(chapter_id: i32) []const u8 {
    return switch (chapter_id) {
        1 => "Ch.1 Act I: Utterance of Marvels",
        2 => "Ch.1 Act II: Echoing Marche",
        3 => "Ch.1 Act III (Pt.1): Stopping the Array",
        4 => "Ch.1 Act III: Ominous Star",
        5 => "Ch.1 Act IV: Clashing Blades",
        6 => "Ch.1 Act V: Rewinding Raindrops",
        7 => "Ch.1 Act VI: Grand Warstorm",
        8 => "Ch.1 Act VII: Thaw of Eons (1.1)",
        10 => "Ch.1 Act VIII: To the Shore's End (1.3)",
        11 => "Ch.2 Prologue: Through the Sea Thou Break",
        12 => "Ch.2 Act I: Like a Skiff Adrift",
        13 => "Ch.2 Act II: Veils Off in Sun or Shadow",
        14 => "Ch.2 Act III: What Yesterday Wept, Today It Singeth",
        15 => "Ch.2 Act IV: The Maiden, The Defier, The Death Crier",
        16 => "Ch.2 Act V: Shadow of Glory (Septimont Pt.1)",
        17 => "Ch.2 Act VI: Flames of Heart (Septimont Pt.2)",
        18 => "Ch.2 Act VII: Dreamcatchers in the Secret Gardens (2.5)",
        19 => "Ch.2 Interlude: Septimont Aftermath",
        20 => "Ch.2 Act VIII: By Sun's Burning Hand (2.6 Pt.1)",
        21 => "Ch.2 Act IX: By Moon's Fated Light (2.6 Pt.2)",
        22 => "Ch.2 Act X: The Dark Tide's Breaking Dawn (2.7 Pt.1)",
        23 => "Ch.2 Act XI: Dawn Breaks on Dark Tides (2.7 Pt.2)",
        24 => "Ch.2 Finale: Dawn Breaks on Dark Tides (2.7 Epilogue)",
        25 => "Ch.2 Afterstory: Stranger Things in Honami (2.8 Pt.1)",
        26 => "Ch.2 Afterstory: Honami Chronicle (2.8 Pt.2)",
        27 => "Ch.3 Act I: Lahai-Roi (3.0 Pt.1)",
        28 => "Ch.3 Act II: Lahai-Roi (3.0 Pt.2)",
        29 => "Ch.3 Prologue: Journey to Lahai-Roi (3.0)",
        30 => "Ch.3 Act III: Lahai-Roi Frozen Expanse (3.1 Pt.1)",
        31 => "Ch.3 Act IV: Lahai-Roi (3.1 Pt.2)",
        32 => "Ch.3 Act V: Story (3.2 Pt.1)",
        33 => "Ch.3 Interlude: Story (3.2 Pt.2)",
        34 => "Ch.3 Prologue: Story (3.3)",
        35 => "Ch.3 Act VI: Main Story (3.3)",
        36 => "Ch.3 Epilogue: Crimson Snow (3.3)",
        37 => "Ch.3 Act VII: Main Story (3.3 Pt.2)",
        38 => "Collab: Cyberpunk Edgerunners (3.4)",
        39 => "Ch.3 Act VIII: Rosella / Luosela (3.4)",
        40 => "Ch.3 Act IX: Xuanfang City (3.5 Pt.1)",
        41 => "Ch.3 Act X: Xuanfang City Battle (3.5 Pt.2)",
        42 => "Ch.3 Epilogue: Xuanfang City (3.5)",
        43 => "Ch.3 Act XI: Xuanfang City Sanctuary (3.6)",
        44 => "Ch.3 Interlude: Xuanfang City (3.6)",
        45 => "Ch.3 Act XII: Mengzhou Moon Festival (3.7)",
        1004 => "Companion: Jiyan (Nightmarchers' Trail)",
        1005 => "Companion: Encore (Small Wooly's Big Adventure)",
        1006 => "Companion: Yinlin (Solitary Path)",
        1007 => "Companion: Lingyang (Wild Heart's Return)",
        1009 => "Companion: Changli (Immortal Flame)",
        1010 => "Companion: Zhezhi (True Colors)",
        1011 => "Companion: Camellya (Forking Paths Among the Stars)",
        1012 => "Companion: Carlotta (If on a Winter's Night a Traveler)",
        1013 => "Companion: Roccia (Starry Night)",
        1014 => "Companion: Brant (Sail Day, Captain!)",
        1015 => "Companion: Cantarella (A Fleeting Night's Dream)",
        1016 => "Companion: Zani (A Blaze in the Dark)",
        2001 => "Exploration: Central Plains / Huanglong",
        2004 => "Exploration: Whining Aix's Mire",
        2010 => "Exploration: Wuming Bay",
        2011 => "Exploration: Echoes of the Past",
        2012 => "Exploration: Dim Forest (Moonlit Woods)",
        2013 => "Exploration: Mt. Firmament Dragon Pearl",
        2014 => "Exploration: Outpost Infrastructure",
        2015 => "Exploration: Dark Plain Guide",
        3001 => "Event: Moon-Chasing Festival",
        3002 => "Event: Moon-Chasing Companions",
        3003 => "Event: When the Night Knocks",
        3004 => "Event: Old Man and the Whale (Riccioli)",
        3005 => "Event: Treasures of the Isles",
        3006 => "Exploration: Averardo Vault",
        3007 => "Side: Starry Framing / Research",
        3008 => "Exploration: Lahai-Roi Glowing Woods",
        3009 => "World: Xuanfang District",
        3010 => "Exploration: Siming Well & Automata Factory",
        3011 => "Side: Mengzhou Encounters",
        3012 => "World: Clockwork Manor",
        3013 => "Domain: Dream Pivot (Xuxin)",
        3014 => "Chronicles: Qixin Domain",
        3015 => "Domain: Jinxin / Qixin POI",
        6001 => "Event: Ciaccona Story (2.3)",
        6003 => "Co-op Event: Lynae",
        else => "",
    };
}

pub fn knownQuestTitle(quest_id: i32) ?[]const u8 {
    return switch (quest_id) {
        // Main Story (Type 1)
        139000025 => "Ch.1 Act I: Utterance of Marvels (Part 1 - Awakening)",
        139000026 => "Ch.1 Act I: Utterance of Marvels (Part 2 - Jinzhou Gate)",
        139000027 => "Ch.1 Act II: Echoing Marche",
        139000029 => "Ch.1 Act III (Pt.1): Clashing Array in Jinzhou",
        139000030 => "Ch.1 Act III: Ominous Star",
        139000031 => "Ch.1 Act IV: Clashing Blades",
        114000020 => "Ch.1 Act V: Rewinding Raindrops",
        140000004 => "Ch.1 Act VI: Grand Warstorm (Pt.1 - Central Plains)",
        102000000 => "Ch.1 Act VI: Grand Warstorm (Pt.2 - Norfall Barrens)",
        140800000 => "Ch.1 Act VI: Grand Warstorm (Finale - Havoc Rover)",
        880000013 => "Ch.1 Finale Epilogue: Abby Appears",
        125000120 => "Ch.1 Act VII (1.1): Traces in the Dream",
        125000121 => "Ch.1 Act VII (1.1): Journey to Mt. Firmament & Hongzhen",
        125000122 => "Ch.1 Act VII (1.1): Pilgrim's Path",
        125000123 => "Ch.1 Act VII (1.1): Master of Seasons (Jue)",
        125000124 => "Ch.1 Act VII (1.1): Snow Clears, Wind Calms",
        133000009 => "Ch.1 Act VII (1.1): Ascending Mt. Firmament",
        146000002 => "Ch.1 Act VII (1.1): Loong's Rest Sanctuary",
        125900000 => "1.1 Main Story Gallery Archive",
        145000004 => "Ch.1 Act VIII (1.3): To the Shore's End",
        114000026 => "Ch.2 Prologue (2.0): Journey to Rinascita",
        114000027 => "Ch.2 Act I (2.0): The Sacred Breeze So Often Blows",
        114000028 => "Ch.2 Act II (2.0): Veils Off in Sun or Shadow (Pt.1)",
        114000030 => "Ch.2 Act II (2.0): Veils Off in Sun or Shadow (Pt.2)",
        140000011 => "Ch.2 Act III (2.0): Penitent's End (Pt.1)",
        114000029 => "Ch.2 Act III (2.0): What Yesterday Wept (Pt.2)",
        145000005 => "Ch.2 Act IV (2.2): Avinoleum",
        125000125 => "Ch.2 Act V (2.4): Septimont Upheaval (Pt.1)",
        125000126 => "Ch.2 Act V (2.4): Septimont Upheaval (Pt.2)",
        121000038 => "Ch.2 Act VI (2.4): Septimont Second Half",
        171000004 => "Ch.2 Act VII (2.5): Fabricatorium Prologue",
        147000002 => "Ch.2 Act VII (2.5): Cultivation Grounds",
        147000005 => "Ch.2 Act VII (2.5): Sanctuary of Pilgrimage",
        150000000 => "Ch.2 Act VII (2.5): Energy Field Upper Sector",
        172000003 => "Ch.2 Act VII (2.5): Energy Field Lower Sector",
        172000000 => "Ch.2 Act VII (2.5): The Burial Grounds",
        172000001 => "Ch.2 Act VII (2.5): Central Tower Boss",
        164000000 => "Ch.2 Interlude (2.5): Overture",
        311000001 => "Ch.2 Act VIII (2.6 Pt.1): Hunting Grounds",
        311000002 => "Ch.2 Act VIII (2.6 Pt.2): False Sovereign",
        886000001 => "Ch.2 Act IX (2.6): Iuno's Moonlit Fate",
        168000001 => "Ch.2 Act X (2.7): Dawn Breaks on Dark Tides (Pt.1)",
        158800019 => "Ch.2 Act XI (2.7): Dawn Breaks on Dark Tides (Act XI)",
        175000000 => "Ch.2 Act XII (2.7): Finale",
        880000044 => "Ch.2 Epilogue (2.8): Chisa & Honami City",
        165000004 => "Ch.2 Epilogue (2.8): Honami Tides Chronicle",
        915700000 => "Ch.3 Prologue (3.0): Arrival at Lahai-Roi",
        915000001 => "Ch.3 Act I (3.0 Pt.1): Lahai-Roi Chapter 1",
        915000003 => "Ch.3 Act I (3.0 Pt.2): Starry Campus",
        915000004 => "Ch.3 Act I (3.0 Pt.3): Lahai-Roi Conclusion",
        155000002 => "Ch.3 Act II (3.0 Pt.1): Beneath the Frost",
        936000002 => "Ch.3 Act II (3.0 Pt.2): Resonant Frost",
        155000003 => "Ch.3 Act II (3.0 Pt.3): Aemeath Finale",
        690000001 => "Ch.3 Act III (3.1 Pt.1): Lahai-Roi Deep",
        114000031 => "Ch.3 Act III (3.1 Pt.2): Solitary Trail",
        114000032 => "Ch.3 Act III (3.1 Pt.3): Frozen Sanctum",
        189000000 => "Ch.3 Act IV (3.1 Pt.4): Luhesi's Oath",
        168800009 => "Ch.3 Act V (3.2): Northern Expanse",
        165800000 => "Ch.3 Interlude (3.2): Signal in the Snow (Pt.1)",
        165800001 => "Ch.3 Interlude (3.2): Signal in the Snow (Pt.2)",
        155000004 => "Ch.3 Prologue (3.3): Prelude",
        121000040 => "Ch.3 Act VI (3.3): Crimson Blade",
        121850000 => "Ch.3 Epilogue (3.3): Crimson Snow",
        119000000 => "Ch.3 Act VII (3.3): Winter's End",
        125000129 => "Ch.3 Act VIII (3.4): Luosela's Tale",
        189000005 => "Ch.3 Act IX (3.5): Xuanfang City - Yangyang",
        192000006 => "Ch.3 Act IX (3.5): Xuanfang Act I",
        880000045 => "Ch.3 Act X (3.5): Xuanfang Climax",
        189000006 => "Ch.3 Act X (3.5): Xuanfang Final Battle",
        192750000 => "Ch.3 Epilogue (3.5): Xuanfang Afterstory",
        192000007 => "Ch.3 Epilogue (3.5): Peace Returns",
        173000000 => "Ch.3 Act XI (3.6): Xuanfang City Heart",
        915000005 => "Ch.3 Act XI (3.6): Inner Demon Illusion",
        936000003 => "Ch.3 Act XI (3.6): Shrine Illusion",
        936000005 => "Ch.3 Epilogue (3.6): Dawn Over Xuanfang",
        936800000 => "Ch.3 Interlude (3.6): Gathering Storms",
        168000003 => "Ch.3 Act XII (3.7): Chaoyue Assembly",
        168850009 => "Ch.3 Epilogue (3.7): Afterstory",
        886000002 => "Ch.3 Act XII (3.7): Mengzhou Finale",
        880000017 => "Moon-Chasing Festival: Stage 1 Requests",
        880000021 => "Moon-Chasing Festival: Stage 2 Requests",
        880000025 => "Moon-Chasing Festival: Stage 3 Requests",
        880000032 => "Moon-Chasing Festival: Stage 4 Requests",
        // Collab Story (Type 16)
        159000000 => "Collab (3.4): Edgerunners Crossover Pt.1",
        159000001 => "Collab (3.4): Cyberpunk Edgerunners Pt.2",
        // Companion Stories (Type 3)
        125000080 => "Companion: Jiyan Pt.1 (Nightmarchers' Past)",
        125000081 => "Companion: Jiyan Pt.2 (Tide of Memory)",
        125000082 => "Companion: Jiyan Pt.3 (Verdant Edge)",
        121000036 => "Companion: Encore (Wooly Kingdom)",
        114000018 => "Companion: Yinlin (Solitary Path)",
        121000031 => "Companion: Lingyang (Wild Heart)",
        121000035 => "Companion: Changli (Blazing Path)",
        311000000 => "Companion: Zhezhi (True Colors)",
        886000000 => "Companion: Camellya (Stellar Fate)",
        121000037 => "Companion: Carlotta (Winter's Night)",
        154000000 => "Companion: Roccia (Starry Night)",
        155000000 => "Companion: Brant (All Sails Set)",
        880000033 => "Companion: Cantarella (Phantom Dream)",
        158800014 => "Companion: Zani (Night Shift)",
        else => null,
    };
}

pub fn resolveQuestTitle(
    arena: Allocator,
    id: i32,
    quest_type: i32,
    chapter_id: i32,
    region_id: i32,
    raw_key: []const u8,
) []const u8 {
    if (knownQuestTitle(id)) |exact| return exact;
    if (raw_key.len > 0 and isAsciiPrintable(raw_key)) return raw_key;

    const ch_name = chapterName(chapter_id);
    const reg_name = regionName(region_id);
    const type_name = questTypeName(quest_type);

    // Friendly prefix translation for common repeatable/world Chinese patterns
    if (std.mem.indexOf(u8, raw_key, "\xE7\x89\xA9\xE6\xB5\x81") != null) { // 物流
        return std.fmt.allocPrint(arena, "Lollo Logistics - {s} (#{d})", .{ reg_name, id }) catch raw_key;
    }
    if (std.mem.indexOf(u8, raw_key, "\xE8\xB7\x91\xE9\x85\xB7") != null) { // 跑酷
        return std.fmt.allocPrint(arena, "Parkour Challenge - {s} (#{d})", .{ reg_name, id }) catch raw_key;
    }
    if (std.mem.indexOf(u8, raw_key, "\xE5\xA3\xB0\xE5\x8C\xA3") != null) { // 声匣
        return std.fmt.allocPrint(arena, "Sonance Casket Delivery - {s} (#{d})", .{ reg_name, id }) catch raw_key;
    }
    if (std.mem.indexOf(u8, raw_key, "\xE5\x80\xBE\xE5\x90\xAC\xE5\x8A\xA8\xE7\x89\xA9") != null) { // 倾听动物
        return std.fmt.allocPrint(arena, "Animal Translator - {s} (#{d})", .{ reg_name, id }) catch raw_key;
    }
    if (std.mem.indexOf(u8, raw_key, "\xE7\x9B\xB8\xE5\xA3\xB0\xE5\xA4\xA7\xE5\xB8\x88") != null) { // 相声大师
        return std.fmt.allocPrint(arena, "Crosstalk Master - {s} (#{d})", .{ reg_name, id }) catch raw_key;
    }
    if (std.mem.indexOf(u8, raw_key, "\xE7\x83\xB9\xE9\xA5\xAA") != null) { // 烹饪
        return std.fmt.allocPrint(arena, "Cooking Commission - {s} (#{d})", .{ reg_name, id }) catch raw_key;
    }
    if (std.mem.indexOf(u8, raw_key, "\xE5\x9B\x9E\xE9\xA1\xBE") != null) { // 回顾
        const ctx_lbl = if (ch_name.len > 0) ch_name else reg_name;
        return std.fmt.allocPrint(arena, "Story Replay - {s} (#{d})", .{ ctx_lbl, id }) catch raw_key;
    }
    if (std.mem.indexOf(u8, raw_key, "\xE5\x81\xB6\xE9\x81\x87") != null) { // 偶遇
        return std.fmt.allocPrint(arena, "Character Encounter - {s} (#{d})", .{ reg_name, id }) catch raw_key;
    }

    if (ch_name.len > 0) {
        return std.fmt.allocPrint(arena, "{s} - {s} (#{d})", .{ type_name, ch_name, id }) catch raw_key;
    }
    return std.fmt.allocPrint(arena, "{s} - {s} (#{d})", .{ type_name, reg_name, id }) catch raw_key;
}

pub fn weaponTypeName(wt: i32) []const u8 {
    return switch (wt) {
        1 => "Broadblade",
        2 => "Sword",
        3 => "Pistols",
        4 => "Gauntlets",
        5 => "Rectifier",
        else => "Weapon",
    };
}

pub fn itemKindName(main_type: i32, item_type: i32) []const u8 {
    return switch (main_type) {
        0 => "Currency / EXP",
        1 => "Resonator Token",
        2 => "Weapon Item",
        3 => "Echo Item",
        4 => switch (item_type) {
            1 => "Upgrade Material",
            2 => "Ascension / Skill Material",
            3 => "Consumable / Food",
            else => "Material",
        },
        5 => "Quest Item",
        6 => "Recipe / Blueprint",
        7 => "Cosmetic / Skin",
        8 => "Data / Trophy",
        9 => "Tide / Bundle",
        else => "Misc Item",
    };
}
