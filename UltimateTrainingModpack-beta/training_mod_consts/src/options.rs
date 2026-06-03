use crate::TOGGLE_MAX;
use byteflags::*;
use core::f64::consts::PI;

#[cfg(feature = "smash")]
use smash::lib::lua_const::*;
use training_mod_tui::{
    StatefulSlider, StatefulTable, SubMenu, SubMenuType, Toggle, NX_SUBMENU_COLUMNS,
    NX_SUBMENU_ROWS,
};

pub trait SubMenuTrait {
    fn to_submenu<'a>(
        title: &'a str,
        id: &'a str,
        help_text: &'a str,
        submenu_type: SubMenuType,
        allow_weights: bool,
    ) -> SubMenu<'a>;
}

#[macro_export]
macro_rules! impl_submenutrait {
    ($e:ty) => {
        impl SubMenuTrait for $e {
            fn to_submenu<'a>(
                title: &'a str,
                id: &'a str,
                help_text: &'a str,
                submenu_type: SubMenuType,
                allow_weights: bool,
            ) -> SubMenu<'a> {
                match submenu_type {
                    SubMenuType::ToggleSingle => {
                        let value = 0;
                        let max = 1;
                        let toggles_vec: Vec<Toggle> = Self::ALL_NAMES
                            .iter()
                            .map(|title| Toggle { title, value, max })
                            .collect();
                        SubMenu {
                            title: title,
                            id: id,
                            help_text: help_text,
                            submenu_type: submenu_type,
                            toggles: StatefulTable::with_items(
                                NX_SUBMENU_ROWS,
                                NX_SUBMENU_COLUMNS,
                                toggles_vec,
                            ),
                            slider: None,
                        }
                    }
                    SubMenuType::ToggleMultiple => {
                        let value = 0;
                        let max = if allow_weights { TOGGLE_MAX } else { 1 };
                        let toggles_vec: Vec<Toggle> = Self::ALL_NAMES
                            .iter()
                            .map(|title| Toggle { title, value, max })
                            .collect();
                        SubMenu {
                            title: title,
                            id: id,
                            help_text: help_text,
                            submenu_type: submenu_type,
                            toggles: StatefulTable::with_items(
                                NX_SUBMENU_ROWS,
                                NX_SUBMENU_COLUMNS,
                                toggles_vec,
                            ),
                            slider: None,
                        }
                    }
                    SubMenuType::Slider => {
                        let slider = StatefulSlider {
                            lower: 0,
                            upper: 150,
                            ..StatefulSlider::new()
                        };
                        SubMenu {
                            title: title,
                            id: id,
                            help_text: help_text,
                            submenu_type: submenu_type,
                            toggles: StatefulTable::with_items(
                                NX_SUBMENU_ROWS,
                                NX_SUBMENU_COLUMNS,
                                Vec::new(),
                            ),
                            slider: Some(slider),
                        }
                    }
                }
            }
        }
    };
}

pub fn get_random_int(_max: i32) -> i32 {
    #[cfg(feature = "smash")]
    unsafe {
        smash::app::sv_math::rand(smash::hash40("fighter"), _max)
    }

    #[cfg(not(feature = "smash"))]
    0
}

/// Generate a random float between _min and _max.
/// Note that (_min <= _max) is not enforced.
pub fn get_random_float(_min: f32, _max: f32) -> f32 {
    #[cfg(feature = "smash")]
    unsafe {
        _min + smash::app::sv_math::randf(smash::hash40("fighter"), _max - _min)
    }

    #[cfg(not(feature = "smash"))]
    _min
}

pub fn random_option<T>(arg: &[T]) -> &T {
    &arg[get_random_int(arg.len() as i32) as usize]
}

// DI
/*
 0, 0.785398, 1.570796, 2.356194, -3.14159, -2.356194,  -1.570796, -0.785398
 0, pi/4,     pi/2,     3pi/4,    pi,       5pi/4,      3pi/2,     7pi/4
*/

// DI / Left stick
byteflags! {
    pub struct Direction {
        pub OUT = "外",
        pub UP_OUT = "斜め上外",
        pub UP = "上",
        pub UP_IN = "斜め上内",
        pub IN = "内",
        pub DOWN_IN = "斜め下内",
        pub DOWN = "下",
        pub DOWN_OUT = "斜め下外",
        pub NEUTRAL = "ニュートラル",
        pub LEFT = "左",
        pub RIGHT = "右",
    }
}

impl_submenutrait!(Direction);

impl Direction {
    pub fn into_angle(self) -> Option<f64> {
        let index = self.into_index();

        if index == 0.0 {
            None
        } else {
            Some((index - 1.0) * PI / 4.0)
        }
    }
    fn into_index(self) -> f64 {
        if self == Direction::empty() {
            return 0.0;
        };
        match self {
            Direction::OUT => 1.0,
            Direction::UP_OUT => 2.0,
            Direction::UP => 3.0,
            Direction::UP_IN => 4.0,
            Direction::IN => 5.0,
            Direction::DOWN_IN => 6.0,
            Direction::DOWN => 7.0,
            Direction::DOWN_OUT => 8.0,
            Direction::NEUTRAL => 0.0,
            Direction::LEFT => 5.0,
            Direction::RIGHT => 1.0,
            _ => panic!("Invalid value in Direction::into_index: {}", self),
        }
    }
}

// Ledge Option
byteflags! {
    pub struct LedgeOption
    {
        pub NEUTRAL = "ニュートラル起き上がり",
        pub ROLL = "受け身ロール",
        pub JUMP = "ジャンプ",
        pub ATTACK = "起き上がり攻撃",
        pub WAIT = "待機",
        pub PLAYBACK_1 = "再生スロット1",
        pub PLAYBACK_2 = "再生スロット2",
        pub PLAYBACK_3 = "再生スロット3",
        pub PLAYBACK_4 = "再生スロット4",
        pub PLAYBACK_5 = "再生スロット5",
    }
}

impl_submenutrait!(LedgeOption);

impl LedgeOption {
    pub fn into_status(self) -> Option<i32> {
        #[cfg(feature = "smash")]
        {
            Some(match self {
                LedgeOption::NEUTRAL => *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
                LedgeOption::ROLL => *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
                LedgeOption::JUMP => *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
                LedgeOption::ATTACK => *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
                LedgeOption::WAIT => *FIGHTER_STATUS_KIND_CLIFF_WAIT,
                LedgeOption::PLAYBACK_1
                | LedgeOption::PLAYBACK_2
                | LedgeOption::PLAYBACK_3
                | LedgeOption::PLAYBACK_4
                | LedgeOption::PLAYBACK_5 => *FIGHTER_STATUS_KIND_NONE,
                _ => return None,
            })
        }

        #[cfg(not(feature = "smash"))]
        None
    }

    pub fn is_playback(self) -> bool {
        match self {
            LedgeOption::PLAYBACK_1
            | LedgeOption::PLAYBACK_2
            | LedgeOption::PLAYBACK_3
            | LedgeOption::PLAYBACK_4
            | LedgeOption::PLAYBACK_5 => true,
            _ => false,
        }
    }

    pub fn playback_slot(self) -> Option<usize> {
        Some(match self {
            LedgeOption::PLAYBACK_1 => 0,
            LedgeOption::PLAYBACK_2 => 1,
            LedgeOption::PLAYBACK_3 => 2,
            LedgeOption::PLAYBACK_4 => 3,
            LedgeOption::PLAYBACK_5 => 4,
            _ => return None,
        })
    }

    pub const fn default() -> LedgeOption {
        // Neutral,Roll,Jump,Attack (everything except wait)
        LedgeOption {
            NEUTRAL: 1,
            ROLL: 1,
            JUMP: 1,
            ATTACK: 1,
            ..LedgeOption::empty()
        }
    }
}

// Tech options
byteflags! {
    pub struct TechFlags {
        pub NO_TECH = "受け身なし",
        pub ROLL_F = "前方受け身",
        pub ROLL_B = "後方受け身",
        pub IN_PLACE = "その場受け身",
    }
}

impl_submenutrait!(TechFlags);

// Missed Tech Options
byteflags! {
    pub struct MissTechFlags {
        pub GETUP = "ニュートラル起き上がり",
        pub ATTACK = "起き上がり攻撃",
        pub ROLL_F = "前方受け身",
        pub ROLL_B = "後方受け身",
    }
}

impl_submenutrait!(MissTechFlags);

byteflags! {
    pub struct Shield {
        pub NONE = "なし",
        pub INFINITE = "無限",
        pub HOLD = "ホールド",
        pub CONSTANT = "常時",
    }
}

impl_submenutrait!(Shield);

byteflags! {
    pub struct SaveStateMirroring {
        pub NONE = "なし",
        pub ALTERNATE = "交互",
        pub RANDOM = "ランダム",
    }
}

impl_submenutrait!(SaveStateMirroring);

byteflags! {
    pub struct OnOff {
        pub ON = "オン",
        pub OFF = "オフ",
    }
}

impl_submenutrait!(OnOff);

impl OnOff {
    pub fn from_val(val: u32) -> Option<Self> {
        match val {
            1 => Some(OnOff::ON),
            0 => Some(OnOff::OFF),
            _ => None,
        }
    }

    pub fn as_bool(self) -> bool {
        match self {
            OnOff::OFF => false,
            OnOff::ON => true,
            _ => panic!("Invalid value in OnOff::as_bool: {}", self),
        }
    }
}

byteflags! {
    pub struct Action {
        pub AIR_DODGE = "空中回避",
        pub JUMP = "ジャンプ",
        pub SHIELD = "シールド",
        pub SPOT_DODGE = "その場回避",
        pub ROLL_F = "前方受け身",
        pub ROLL_B = "後方受け身",
        pub NAIR = "ニュートラル空中攻撃",
        pub FAIR = "前空中攻撃",
        pub BAIR = "後ろ空中攻撃",
        pub UAIR = "上空中攻撃",
        pub DAIR = "下空中攻撃",
        pub NEUTRAL_B = "ニュートラル必殺技",
        pub SIDE_B = "横必殺技",
        pub UP_B = "上必殺技",
        pub DOWN_B = "下必殺技",
        pub F_SMASH = "横スマッシュ",
        pub U_SMASH = "上スマッシュ",
        pub D_SMASH = "下スマッシュ",
        pub JAB = "弱攻撃",
        pub F_TILT = "横強攻撃",
        pub U_TILT  = "上強攻撃",
        pub D_TILT  = "下強攻撃",
        pub GRAB = "つかみ",
        pub DASH = "ダッシュ",
        pub DASH_ATTACK = "ダッシュ攻撃",
        pub PLAYBACK_1 = "再生スロット1",
        pub PLAYBACK_2 = "再生スロット2",
        pub PLAYBACK_3 = "再生スロット3",
        pub PLAYBACK_4 = "再生スロット4",
        pub PLAYBACK_5 = "再生スロット5",
    }
}

impl_submenutrait!(Action);

impl Action {
    pub fn into_attack_air_kind(self) -> Option<i32> {
        #[cfg(feature = "smash")]
        {
            Some(match self {
                Action::NAIR => *FIGHTER_COMMAND_ATTACK_AIR_KIND_N,
                Action::FAIR => *FIGHTER_COMMAND_ATTACK_AIR_KIND_F,
                Action::BAIR => *FIGHTER_COMMAND_ATTACK_AIR_KIND_B,
                Action::DAIR => *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW,
                Action::UAIR => *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI,
                _ => return None,
            })
        }

        #[cfg(not(feature = "smash"))]
        None
    }

    pub fn is_playback(self) -> bool {
        match self {
            Action::PLAYBACK_1
            | Action::PLAYBACK_2
            | Action::PLAYBACK_3
            | Action::PLAYBACK_4
            | Action::PLAYBACK_5 => true,
            _ => false,
        }
    }

    pub fn playback_slot(self) -> usize {
        match self {
            Action::PLAYBACK_1 => 0,
            Action::PLAYBACK_2 => 1,
            Action::PLAYBACK_3 => 2,
            Action::PLAYBACK_4 => 3,
            Action::PLAYBACK_5 => 4,
            _ => panic!("Invalid Action playback slot: {}", self.to_string()),
        }
    }
}
byteflags! {
    pub struct AttackAngle {
        pub NEUTRAL = "ニュートラル",
        pub UP = "上",
        pub DOWN = "下",
    }
}

impl_submenutrait!(AttackAngle);

byteflags! {
    pub struct Delay {
        pub D0 = "0",
        pub D1 = "1",
        pub D2 = "2",
        pub D3 = "3",
        pub D4 = "4",
        pub D5 = "5",
        pub D6 = "6",
        pub D7 = "7",
        pub D8 = "8",
        pub D9 = "9",
        pub D10 = "10",
        pub D11 = "11",
        pub D12 = "12",
        pub D13 = "13",
        pub D14 = "14",
        pub D15 = "15",
        pub D16 = "16",
        pub D17 = "17",
        pub D18 = "18",
        pub D19 = "19",
        pub D20 = "20",
        pub D21 = "21",
        pub D22 = "22",
        pub D23 = "23",
        pub D24 = "24",
        pub D25 = "25",
        pub D26 = "26",
        pub D27 = "27",
        pub D28 = "28",
        pub D29 = "29",
        pub D30 = "30",
    }
}

impl_submenutrait!(Delay);

impl Delay {
    pub fn into_delay(&self) -> u32 {
        if *self == Delay::empty() {
            return 0;
        };
        match *self {
            Delay::D0 => 0,
            Delay::D1 => 1,
            Delay::D2 => 2,
            Delay::D3 => 3,
            Delay::D4 => 4,
            Delay::D5 => 5,
            Delay::D6 => 6,
            Delay::D7 => 7,
            Delay::D8 => 8,
            Delay::D9 => 9,
            Delay::D10 => 10,
            Delay::D11 => 11,
            Delay::D12 => 12,
            Delay::D13 => 13,
            Delay::D14 => 14,
            Delay::D15 => 15,
            Delay::D16 => 16,
            Delay::D17 => 17,
            Delay::D18 => 18,
            Delay::D19 => 19,
            Delay::D20 => 20,
            Delay::D21 => 21,
            Delay::D22 => 22,
            Delay::D23 => 23,
            Delay::D24 => 24,
            Delay::D25 => 25,
            Delay::D26 => 26,
            Delay::D27 => 27,
            Delay::D28 => 28,
            Delay::D29 => 29,
            Delay::D30 => 30,
            _ => panic!("Invalid value in Delay::into_delay: {}", self),
        }
    }
}

byteflags! {
    pub struct MedDelay {
        pub D0 = "0",
        pub D5 = "5",
        pub D10 = "10",
        pub D15 = "15",
        pub D20 = "20",
        pub D25 = "25",
        pub D30 = "30",
        pub D35 = "35",
        pub D40 = "40",
        pub D45 = "45",
        pub D50 = "50",
        pub D55 = "55",
        pub D60 = "60",
        pub D65 = "65",
        pub D70 = "70",
        pub D75 = "75",
        pub D80 = "80",
        pub D85 = "85",
        pub D90 = "90",
        pub D95 = "95",
        pub D100 = "100",
        pub D105 = "105",
        pub D110 = "110",
        pub D115 = "115",
        pub D120 = "120",
        pub D125 = "125",
        pub D130 = "130",
        pub D135 = "135",
        pub D140 = "140",
        pub D145 = "145",
        pub D150 = "150",
    }
}

impl_submenutrait!(MedDelay);

impl MedDelay {
    pub fn into_meddelay(&self) -> u32 {
        if *self == MedDelay::empty() {
            return 0;
        };
        match *self {
            MedDelay::D0 => 0,
            MedDelay::D5 => 5,
            MedDelay::D10 => 10,
            MedDelay::D15 => 15,
            MedDelay::D20 => 20,
            MedDelay::D25 => 25,
            MedDelay::D30 => 30,
            MedDelay::D35 => 35,
            MedDelay::D40 => 40,
            MedDelay::D45 => 45,
            MedDelay::D50 => 50,
            MedDelay::D55 => 55,
            MedDelay::D60 => 60,
            MedDelay::D65 => 65,
            MedDelay::D70 => 70,
            MedDelay::D75 => 75,
            MedDelay::D80 => 80,
            MedDelay::D85 => 85,
            MedDelay::D90 => 90,
            MedDelay::D95 => 95,
            MedDelay::D100 => 100,
            MedDelay::D105 => 105,
            MedDelay::D110 => 110,
            MedDelay::D115 => 115,
            MedDelay::D120 => 120,
            MedDelay::D125 => 125,
            MedDelay::D130 => 130,
            MedDelay::D135 => 135,
            MedDelay::D140 => 140,
            MedDelay::D145 => 145,
            MedDelay::D150 => 150,
            _ => panic!("Invalid value in MedDelay::into_meddelay: {}", self),
        }
    }
}

byteflags! {
    pub struct LongDelay {
        pub D0 = "0",
        pub D10 = "10",
        pub D20 = "20",
        pub D30 = "30",
        pub D40 = "40",
        pub D50 = "50",
        pub D60 = "60",
        pub D70 = "70",
        pub D80 = "80",
        pub D90 = "90",
        pub D100 = "100",
        pub D110 = "110",
        pub D120 = "120",
        pub D130 = "130",
        pub D140 = "140",
        pub D150 = "150",
        pub D160 = "160",
        pub D170 = "170",
        pub D180 = "180",
        pub D190 = "190",
        pub D200 = "200",
        pub D210 = "210",
        pub D220 = "220",
        pub D230 = "230",
        pub D240 = "240",
        pub D250 = "250",
        pub D260 = "260",
        pub D270 = "270",
        pub D280 = "280",
        pub D290 = "290",
        pub D300 = "300",
    }
}

impl_submenutrait!(LongDelay);

impl LongDelay {
    pub fn into_longdelay(&self) -> u32 {
        if *self == LongDelay::empty() {
            return 0;
        };
        match *self {
            LongDelay::D0 => 0,
            LongDelay::D10 => 10,
            LongDelay::D20 => 20,
            LongDelay::D30 => 30,
            LongDelay::D40 => 40,
            LongDelay::D50 => 50,
            LongDelay::D60 => 60,
            LongDelay::D70 => 70,
            LongDelay::D80 => 80,
            LongDelay::D90 => 90,
            LongDelay::D100 => 100,
            LongDelay::D110 => 110,
            LongDelay::D120 => 120,
            LongDelay::D130 => 130,
            LongDelay::D140 => 140,
            LongDelay::D150 => 150,
            LongDelay::D160 => 160,
            LongDelay::D170 => 170,
            LongDelay::D180 => 180,
            LongDelay::D190 => 190,
            LongDelay::D200 => 200,
            LongDelay::D210 => 210,
            LongDelay::D220 => 220,
            LongDelay::D230 => 230,
            LongDelay::D240 => 240,
            LongDelay::D250 => 250,
            LongDelay::D260 => 260,
            LongDelay::D270 => 270,
            LongDelay::D280 => 280,
            LongDelay::D290 => 290,
            LongDelay::D300 => 300,
            _ => panic!("Invalid value in LongDelay::into_longdelay: {}", self),
        }
    }
}

byteflags! {
    pub struct BuffOption
    {
        pub ACCELERATLE = "アクセラトル",
        pub OOMPH = "ウンスン",
        pub PSYCHE = "サイキップ",
        pub BOUNCE = "バウンス",
        pub ARSENE = "アルセーヌ",
        pub BREATHING = "深呼吸",
        pub LIMIT = "リミット",
        pub KO = "KOパンチ",
        pub WING = "1-Winged Angel",
        pub MONAD_JUMP = "ジャンプ",
        pub MONAD_SPEED = "スピード",
        pub MONAD_SHIELD = "シールド",
        pub MONAD_BUSTER = "Buster",
        pub MONAD_SMASH = "Smash",
        pub POWER_DRAGON = "パワードラゴン",
        pub WAFT_MINI = "ミニおなら",
        pub WAFT_HALF = "ハーフおなら",
        pub WAFT_FULL = "フルおなら",
    }
}

impl_submenutrait!(BuffOption);

impl BuffOption {
    pub fn into_int(self) -> Option<i32> {
        #[cfg(feature = "smash")]
        {
            Some(match self {
                BuffOption::ACCELERATLE => *FIGHTER_BRAVE_SPECIAL_LW_COMMAND11_SPEED_UP,
                BuffOption::OOMPH => *FIGHTER_BRAVE_SPECIAL_LW_COMMAND12_ATTACK_UP,
                BuffOption::PSYCHE => *FIGHTER_BRAVE_SPECIAL_LW_COMMAND21_CHARGE,
                BuffOption::BOUNCE => *FIGHTER_BRAVE_SPECIAL_LW_COMMAND13_REFLECT,
                BuffOption::BREATHING => 1,
                BuffOption::ARSENE => 1,
                BuffOption::LIMIT => 1,
                BuffOption::KO => 1,
                BuffOption::WING => 1,
                BuffOption::MONAD_JUMP => *FIGHTER_SHULK_MONAD_TYPE_JUMP,
                BuffOption::MONAD_SPEED => *FIGHTER_SHULK_MONAD_TYPE_SPEED,
                BuffOption::MONAD_SHIELD => *FIGHTER_SHULK_MONAD_TYPE_SHIELD,
                BuffOption::MONAD_BUSTER => *FIGHTER_SHULK_MONAD_TYPE_BUSTER,
                BuffOption::MONAD_SMASH => *FIGHTER_SHULK_MONAD_TYPE_SMASH,
                BuffOption::POWER_DRAGON => 1,
                BuffOption::WAFT_MINI => *FIGHTER_WARIO_GASS_LEVEL_M,
                BuffOption::WAFT_HALF => *FIGHTER_WARIO_GASS_LEVEL_L,
                BuffOption::WAFT_FULL => *FIGHTER_WARIO_GASS_LEVEL_FLY,
                _ => return None,
            })
        }

        #[cfg(not(feature = "smash"))]
        None
    }

    pub fn hero_buffs(self) -> BuffOption {
        // Return a struct with only Hero's selected buffs
        let hero_buffs_byteflags = BuffOption::ACCELERATLE
            .union(BuffOption::OOMPH)
            .union(BuffOption::BOUNCE)
            .union(BuffOption::PSYCHE);
        self.left_intersection(hero_buffs_byteflags)
    }

    pub fn shulk_buffs(self) -> BuffOption {
        // Return a struct with only Shulk's selected arts
        let shulk_buffs_byteflags = BuffOption::MONAD_JUMP
            .union(BuffOption::MONAD_SPEED)
            .union(BuffOption::MONAD_SHIELD)
            .union(BuffOption::MONAD_BUSTER)
            .union(BuffOption::MONAD_SMASH);
        self.left_intersection(shulk_buffs_byteflags)
    }

    pub fn wario_buffs(self) -> BuffOption {
        let wario_buffs_byteflags = BuffOption::WAFT_MINI
            .union(BuffOption::WAFT_HALF)
            .union(BuffOption::WAFT_FULL);
        self.left_intersection(wario_buffs_byteflags)
    }
}

byteflags! {
    pub struct ThrowOption
    {
        NONE = "なし",
        FORWARD = "前投げ",
        BACKWARD = "後ろ投げ",
        UP = "上投げ",
        DOWN = "下投げ",
    }
}

impl_submenutrait!(ThrowOption);

impl ThrowOption {
    pub fn into_cmd(self) -> Option<i32> {
        #[cfg(feature = "smash")]
        {
            Some(match self {
                ThrowOption::NONE => 0,
                ThrowOption::FORWARD => *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_F,
                ThrowOption::BACKWARD => *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_B,
                ThrowOption::UP => *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_HI,
                ThrowOption::DOWN => *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_LW,
                _ => return None,
            })
        }

        #[cfg(not(feature = "smash"))]
        None
    }
}

// TODO!() Is this redundant with OnOff?
byteflags! {
    pub struct BoolFlag {
        pub TRUE = "あり",
        pub FALSE = "なし (重複)",
    }
}

impl_submenutrait!(BoolFlag);

impl BoolFlag {
    pub fn into_bool(self) -> bool {
        matches!(self, BoolFlag::TRUE)
    }
}

byteflags! {
    pub struct SdiFrequency {
        pub NONE = "なし",
        pub NORMAL = "普通",
        pub MEDIUM = "中程度",
        pub HIGH = "高",
    }
}

impl_submenutrait!(SdiFrequency);

impl SdiFrequency {
    pub fn into_u32(self) -> u32 {
        match self {
            SdiFrequency::NONE => u32::MAX,
            SdiFrequency::NORMAL => 8,
            SdiFrequency::MEDIUM => 6,
            SdiFrequency::HIGH => 4,
            _ => panic!("Invalid value in SdiFrequency::into_u32: {}", self),
        }
    }
}

byteflags! {
    pub struct ClatterFrequency {
        pub NONE = "なし",
        pub NORMAL = "普通",
        pub MEDIUM = "中程度",
        pub HIGH = "高",
    }
}

impl_submenutrait!(ClatterFrequency);

impl ClatterFrequency {
    pub fn into_u32(self) -> u32 {
        match self {
            ClatterFrequency::NONE => u32::MAX,
            ClatterFrequency::NORMAL => 8,
            ClatterFrequency::MEDIUM => 5,
            ClatterFrequency::HIGH => 2,
            _ => panic!("Invalid value in ClatterFrequency::into_u32: {}", self),
        }
    }
}

byteflags! {
    pub struct CharacterItem {
        pub NONE = "なし",
        pub PLAYER_VARIATION_1 = "プレイヤー1番目",
        pub PLAYER_VARIATION_2 = "プレイヤー2番目",
        pub PLAYER_VARIATION_3 = "プレイヤー3番目",
        pub PLAYER_VARIATION_4 = "プレイヤー4番目",
        pub PLAYER_VARIATION_5 = "プレイヤー5番目",
        pub PLAYER_VARIATION_6 = "プレイヤー6番目",
        pub PLAYER_VARIATION_7 = "プレイヤー7番目",
        pub PLAYER_VARIATION_8 = "プレイヤー8番目",
        pub CPU_VARIATION_1 = "CPU1番目",
        pub CPU_VARIATION_2 = "CPU2番目",
        pub CPU_VARIATION_3 = "CPU3番目",
        pub CPU_VARIATION_4 = "CPU4番目",
        pub CPU_VARIATION_5 = "CPU5番目",
        pub CPU_VARIATION_6 = "CPU6番目",
        pub CPU_VARIATION_7 = "CPU7番目",
        pub CPU_VARIATION_8 = "CPU8番目",
    }
}

impl_submenutrait!(CharacterItem);

impl CharacterItem {
    pub fn as_idx(&self) -> usize {
        match *self {
            CharacterItem::NONE => 0,
            CharacterItem::PLAYER_VARIATION_1 => 1,
            CharacterItem::PLAYER_VARIATION_2 => 2,
            CharacterItem::PLAYER_VARIATION_3 => 3,
            CharacterItem::PLAYER_VARIATION_4 => 4,
            CharacterItem::PLAYER_VARIATION_5 => 5,
            CharacterItem::PLAYER_VARIATION_6 => 6,
            CharacterItem::PLAYER_VARIATION_7 => 7,
            CharacterItem::PLAYER_VARIATION_8 => 8,
            CharacterItem::CPU_VARIATION_1 => 9,
            CharacterItem::CPU_VARIATION_2 => 10,
            CharacterItem::CPU_VARIATION_3 => 11,
            CharacterItem::CPU_VARIATION_4 => 12,
            CharacterItem::CPU_VARIATION_5 => 13,
            CharacterItem::CPU_VARIATION_6 => 14,
            CharacterItem::CPU_VARIATION_7 => 15,
            CharacterItem::CPU_VARIATION_8 => 16,
            _ => panic!("Invalid value in CharacterItem::as_idx: {}", self),
        }
    }
}

byteflags! {
    pub struct MashTrigger {
        pub HIT = "ヒットストン",
        pub SHIELDSTUN = "シールドストン",
        pub PARRY = "パリィ",
        pub TUMBLE = "よろめき",
        pub LANDING = "着地",
        pub TRUMP = "崖奪い",
        pub FOOTSTOOL = "踏み台",
        pub CLATTER = "連打回避",
        pub LEDGE = "崖オプション",
        pub TECH = "受け身オプション",
        pub MISTECH = "受け身ミスオプション",
        pub GROUNDED = "地上",
        pub AIRBORNE = "空中",
        pub DISTANCE_CLOSE = "距離: 近",
        pub DISTANCE_MID = "距離: 中",
        pub DISTANCE_FAR = "距離: 遠",
        pub ALWAYS = "常に",
    }
}

impl_submenutrait!(MashTrigger);

impl MashTrigger {
    pub const fn default() -> MashTrigger {
        // Hit, block, clatter
        MashTrigger {
            HIT: 1,
            TUMBLE: 1,
            SHIELDSTUN: 1,
            CLATTER: 1,
            ..MashTrigger::empty()
        }
    }
}

byteflags! {
    pub struct DamagePercent {
        pub LOWER = "下向き",
        pub UPPER = "上向き",
    }
}

impl_submenutrait!(DamagePercent);

impl DamagePercent {
    pub const fn default() -> DamagePercent {
        DamagePercent {
            LOWER: 0,
            UPPER: 150,
        }
    }
}

byteflags! {
    pub struct SaveDamage {
        pub DEFAULT = "デフォルト",
        pub SAVED = "セーブステート",
        pub RANDOM = "ランダム値",
    }
}

impl_submenutrait!(SaveDamage);

byteflags! {
    pub struct SaveStateSlot
    {
        pub S1 = "スロット1",
        pub S2 = "スロット2",
        pub S3 = "スロット3",
        pub S4 = "スロット4",
        pub S5 = "スロット5",
    }
}

impl_submenutrait!(SaveStateSlot);

impl SaveStateSlot {
    pub fn into_idx(&self) -> Option<usize> {
        match *self {
            SaveStateSlot::S1 => Some(0),
            SaveStateSlot::S2 => Some(1),
            SaveStateSlot::S3 => Some(2),
            SaveStateSlot::S4 => Some(3),
            SaveStateSlot::S5 => Some(4),
            _ => None,
        }
    }
}

byteflags! {
    pub struct RecordSlot {
        pub S1 = "スロット1",
        pub S2 = "スロット2",
        pub S3 = "スロット3",
        pub S4 = "スロット4",
        pub S5 = "スロット5",
    }
}

impl_submenutrait!(RecordSlot);

impl RecordSlot {
    pub fn into_idx(&self) -> Option<usize> {
        match *self {
            RecordSlot::S1 => Some(0),
            RecordSlot::S2 => Some(1),
            RecordSlot::S3 => Some(2),
            RecordSlot::S4 => Some(3),
            RecordSlot::S5 => Some(4),
            _ => None,
        }
    }
}

byteflags! {
    pub struct PlaybackSlot {
        pub S1 = "スロット1",
        pub S2 = "スロット2",
        pub S3 = "スロット3",
        pub S4 = "スロット4",
        pub S5 = "スロット5",
    }
}

impl_submenutrait!(PlaybackSlot);

impl PlaybackSlot {
    pub fn into_idx(&self) -> Option<usize> {
        match *self {
            PlaybackSlot::S1 => Some(0),
            PlaybackSlot::S2 => Some(1),
            PlaybackSlot::S3 => Some(2),
            PlaybackSlot::S4 => Some(3),
            PlaybackSlot::S5 => Some(4),
            _ => None,
        }
    }
}

// If doing input recording out of hitstun, when does playback begin after?
byteflags! {
    pub struct HitstunPlayback {
        pub HITSTUN = "ヒットストン終了時",
        pub HITSTOP = "ヒットストップ終了時",
        pub INSTANT = "ヒットストップ開始時",
    }
}

impl_submenutrait!(HitstunPlayback);

byteflags! {
    pub struct RecordTrigger {
        pub COMMAND = "ボタンコンボ",
        pub SAVESTATE = "セーブステートロード時",
    }
}

impl_submenutrait!(RecordTrigger);

byteflags! {
    pub struct RecordingDuration {
        pub F60 = "60",
        pub F90 = "90",
        pub F120 = "120",
        pub F150 = "150",
        pub F180 = "180",
        pub F210 = "210",
        pub F240 = "240",
        pub F270 = "270",
        pub F300 = "300",
        pub F330 = "330",
        pub F360 = "360",
        pub F390 = "390",
        pub F420 = "420",
        pub F450 = "450",
        pub F480 = "480",
        pub F510 = "510",
        pub F540 = "540",
        pub F570 = "570",
        pub F600 = "600",
    }
}

impl_submenutrait!(RecordingDuration);

impl RecordingDuration {
    pub fn into_frames(&self) -> usize {
        match *self {
            RecordingDuration::F60 => 60,
            RecordingDuration::F90 => 90,
            RecordingDuration::F120 => 120,
            RecordingDuration::F150 => 150,
            RecordingDuration::F180 => 180,
            RecordingDuration::F210 => 210,
            RecordingDuration::F240 => 240,
            RecordingDuration::F270 => 270,
            RecordingDuration::F300 => 300,
            RecordingDuration::F330 => 330,
            RecordingDuration::F360 => 360,
            RecordingDuration::F390 => 390,
            RecordingDuration::F420 => 420,
            RecordingDuration::F450 => 450,
            RecordingDuration::F480 => 480,
            RecordingDuration::F510 => 510,
            RecordingDuration::F540 => 540,
            RecordingDuration::F570 => 570,
            RecordingDuration::F600 => 600,
            _ => panic!("Invalid value in RecordingDuration::into_frames: {}", self),
        }
    }
}

byteflags! {
    pub struct  ButtonConfig {
        pub A = "A",
        pub B = "B",
        pub X = "X",
        pub Y = "Y",
        pub L = "ProコンL",
        pub R = "ProコンR / GCCのZ",
        pub ZL = "ProコンZL / GCCのL",
        pub ZR = "ProコンZR / GCCのR",
        pub DPAD_UP = "十字キー上",
        pub DPAD_DOWN = "十字キー下",
        pub DPAD_LEFT = "十字キー左",
        pub DPAD_RIGHT = "十字キー右",
        pub PLUS = "＋ボタン",
        pub MINUS = "－ボタン",
        pub LSTICK = "左スティック押し込み",
        pub RSTICK = "右スティック押し込み",
    }
}

impl_submenutrait!(ButtonConfig);

byteflags! {
    pub struct UpdatePolicy {
        pub STABLE = "安定版",
        pub BETA = "ベータ版",
        pub DISABLED = "無効",
    }
}

impl_submenutrait!(UpdatePolicy);

impl UpdatePolicy {
    pub const fn default() -> UpdatePolicy {
        UpdatePolicy::STABLE
    }
}

byteflags! {
    pub struct InputDisplay {
        pub NONE = "なし",
        pub SMASH = "スマブラ入力表示",
        pub RAW = "RAW入力表示",
        pub STATUS = "ステータスのみ",
    }
}

impl_submenutrait!(InputDisplay);
