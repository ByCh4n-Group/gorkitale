pub enum AssetType {
    Texture,
    Sound,
}

pub struct AssetDefinition {
    pub name: &'static str,
    pub path: &'static str,
    pub asset_type: AssetType,
}

pub const ASSET_LIST: &[AssetDefinition] = &[
    AssetDefinition {
        name: "Player Front",
        path: "./assets/chara1.png",
        asset_type: AssetType::Texture,
    },
    AssetDefinition {
        name: "Player Left",
        path: "./assets/chara_left.png",
        asset_type: AssetType::Texture,
    },
    AssetDefinition {
        name: "Player Right",
        path: "./assets/chara_right.png",
        asset_type: AssetType::Texture,
    },
    AssetDefinition {
        name: "City Background",
        path: "./assets/city_bg.png",
        asset_type: AssetType::Texture,
    },
    AssetDefinition {
        name: "Gaster Standing",
        path: "./assets/npc_gaster_standing.png",
        asset_type: AssetType::Texture,
    },
    AssetDefinition {
        name: "Gaster Talking",
        path: "./assets/npc_gaster_talking.png",
        asset_type: AssetType::Texture,
    },
    AssetDefinition {
        name: "Rarity",
        path: "./assets/rarity_galla_right.png",
        asset_type: AssetType::Texture,
    },
    AssetDefinition {
        name: "Eilish",
        path: "./assets/eilish.png",
        asset_type: AssetType::Texture,
    },
    AssetDefinition {
        name: "Sans",
        path: "./assets/sans1.png",
        asset_type: AssetType::Texture,
    },
    AssetDefinition {
        name: "Sans Combat",
        path: "./assets/sans1.png",
        asset_type: AssetType::Texture,
    },
    AssetDefinition {
        name: "Sans Shrug",
        path: "./assets/sans_shrug.png",
        asset_type: AssetType::Texture,
    },
    AssetDefinition {
        name: "Sans Handshake",
        path: "./assets/sans_frisk_handshake.png",
        asset_type: AssetType::Texture,
    },
    AssetDefinition {
        name: "Heart",
        path: "./assets/heart.png",
        asset_type: AssetType::Texture,
    },
    AssetDefinition {
        name: "Music Box",
        path: "./assets/musicbox.png",
        asset_type: AssetType::Texture,
    },
    AssetDefinition {
        name: "Music Track",
        path: "./assets/g6_rmx.mp3",
        asset_type: AssetType::Sound,
    },
    AssetDefinition {
        name: "Ayasofya Entrance",
        path: "./assets/ayasofya_giris.jpg",
        asset_type: AssetType::Texture,
    },
    AssetDefinition {
        name: "Ayasofya Interior",
        path: "./assets/ayasofya_ici.jpg",
        asset_type: AssetType::Texture,
    },
    AssetDefinition {
        name: "Bone",
        path: "./assets/bone.png",
        asset_type: AssetType::Texture,
    },
    AssetDefinition {
        name: "Player Fes",
        path: "./assets/chara_fes.png",
        asset_type: AssetType::Texture,
    },
    AssetDefinition {
        name: "Player Takke",
        path: "./assets/chara_mavi_takke.png",
        asset_type: AssetType::Texture,
    },
];
