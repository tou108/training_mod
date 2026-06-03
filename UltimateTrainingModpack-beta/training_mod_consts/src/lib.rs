#![allow(non_snake_case)]
extern crate byteflags;
extern crate num_derive;

use serde::{Deserialize, Serialize};

pub mod options;
pub use options::*;
pub mod files;
pub use files::*;
pub mod config;
pub use config::*;

use training_mod_sync::*;
use training_mod_tui::SubMenuType::*;
pub use training_mod_tui::*;

pub const TOGGLE_MAX: u8 = 5;

#[repr(C)]
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct TrainingModpackMenu {
    pub aerial_delay: Delay,
    pub air_dodge_dir: Direction,
    pub attack_angle: AttackAngle,
    pub buff_state: BuffOption,
    pub character_item: CharacterItem,
    pub clatter_strength: ClatterFrequency,
    pub crouch: OnOff,
    pub di_state: Direction,
    pub falling_aerials: BoolFlag,
    pub fast_fall_delay: Delay,
    pub fast_fall: BoolFlag,
    pub follow_up: Action,
    pub frame_advantage: OnOff,
    pub full_hop: BoolFlag,
    pub hitbox_vis: OnOff,
    pub input_display: InputDisplay,
    pub input_display_status: OnOff,
    pub hud: OnOff,
    pub input_delay: Delay,
    pub ledge_delay: LongDelay,
    pub ledge_state: LedgeOption,
    pub mash_state: Action,
    pub mash_triggers: MashTrigger,
    pub miss_tech_state: MissTechFlags,
    pub oos_offset: Delay,
    pub pummel_delay: MedDelay,
    pub reaction_time: Delay,
    pub save_damage_cpu: SaveDamage,
    pub save_damage_limits_cpu: DamagePercent,
    pub save_damage_player: SaveDamage,
    pub save_damage_limits_player: DamagePercent,
    pub save_state_autoload: OnOff,
    pub save_state_enable: OnOff,
    pub save_state_slot: SaveStateSlot,
    pub randomize_slots: SaveStateSlot,
    pub save_state_mirroring: SaveStateMirroring,
    pub save_state_playback: PlaybackSlot,
    pub sdi_state: Direction,
    pub sdi_strength: SdiFrequency,
    pub shield_state: Shield,
    pub shield_tilt: Direction,
    pub stage_hazards: OnOff,
    pub tech_state: TechFlags,
    pub throw_delay: MedDelay,
    pub throw_state: ThrowOption,
    pub ledge_neutral_override: Action,
    pub ledge_roll_override: Action,
    pub ledge_jump_override: Action,
    pub ledge_attack_override: Action,
    pub tech_action_override: Action,
    pub clatter_override: Action,
    pub tumble_override: Action,
    pub hitstun_override: Action,
    pub parry_override: Action,
    pub shieldstun_override: Action,
    pub footstool_override: Action,
    pub landing_override: Action,
    pub trump_override: Action,
    pub recording_slot: RecordSlot,
    pub record_trigger: RecordTrigger,
    pub recording_duration: RecordingDuration,
    pub playback_button_slots: PlaybackSlot,
    pub hitstun_playback: HitstunPlayback,
    pub playback_mash: OnOff,
    pub playback_loop: OnOff,
    pub menu_open_start_press: OnOff,
    pub save_state_save: ButtonConfig,
    pub save_state_load: ButtonConfig,
    pub input_record: ButtonConfig,
    pub input_playback: ButtonConfig,
    pub recording_crop: OnOff,
    pub stale_dodges: OnOff,
    pub tech_hide: OnOff,
    pub update_policy: UpdatePolicy,
    pub lra_reset: OnOff,
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct MenuJsonStruct {
    pub menu: TrainingModpackMenu,
    pub defaults_menu: TrainingModpackMenu,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FighterId {
    Player = 0,
    CPU = 1,
}

pub static BASE_MENU: TrainingModpackMenu = TrainingModpackMenu {
    aerial_delay: Delay::empty(),
    air_dodge_dir: Direction::empty(),
    attack_angle: AttackAngle::empty(),
    buff_state: BuffOption::empty(),
    character_item: CharacterItem::NONE,
    clatter_strength: ClatterFrequency::NONE,
    crouch: OnOff::OFF,
    di_state: Direction::empty(),
    falling_aerials: BoolFlag::FALSE,
    fast_fall_delay: Delay::empty(),
    fast_fall: BoolFlag::FALSE,
    follow_up: Action::empty(),
    frame_advantage: OnOff::OFF,
    full_hop: BoolFlag::TRUE,
    hitbox_vis: OnOff::OFF,
    input_display: InputDisplay::SMASH,
    input_display_status: OnOff::OFF,
    hud: OnOff::ON,
    input_delay: Delay::D0,
    ledge_delay: LongDelay::empty(),
    ledge_state: LedgeOption::default(),
    mash_state: Action::empty(),
    mash_triggers: MashTrigger::default(),
    miss_tech_state: MissTechFlags::all(),
    oos_offset: Delay::empty(),
    pummel_delay: MedDelay::empty(),
    reaction_time: Delay::empty(),
    save_damage_cpu: SaveDamage::DEFAULT,
    save_damage_limits_cpu: DamagePercent::default(),
    save_damage_player: SaveDamage::DEFAULT,
    save_damage_limits_player: DamagePercent::default(),
    save_state_autoload: OnOff::OFF,
    save_state_enable: OnOff::ON,
    save_state_slot: SaveStateSlot::S1,
    randomize_slots: SaveStateSlot::empty(),
    save_state_mirroring: SaveStateMirroring::NONE,
    save_state_playback: PlaybackSlot::empty(),
    sdi_state: Direction::empty(),
    sdi_strength: SdiFrequency::NONE,
    shield_state: Shield::NONE,
    shield_tilt: Direction::empty(),
    stage_hazards: OnOff::OFF,
    tech_state: TechFlags::all(),
    throw_delay: MedDelay::empty(),
    throw_state: ThrowOption::NONE,
    ledge_neutral_override: Action::empty(),
    ledge_roll_override: Action::empty(),
    ledge_jump_override: Action::empty(),
    ledge_attack_override: Action::empty(),
    tech_action_override: Action::empty(),
    clatter_override: Action::empty(),
    tumble_override: Action::empty(),
    hitstun_override: Action::empty(),
    parry_override: Action::empty(),
    shieldstun_override: Action::empty(),
    footstool_override: Action::empty(),
    landing_override: Action::empty(),
    trump_override: Action::empty(),
    recording_slot: RecordSlot::S1,
    recording_duration: RecordingDuration::F150,
    record_trigger: RecordTrigger::COMMAND,
    playback_button_slots: PlaybackSlot::S1,
    hitstun_playback: HitstunPlayback::HITSTUN,
    playback_mash: OnOff::ON,
    playback_loop: OnOff::OFF,
    menu_open_start_press: OnOff::ON,
    save_state_save: ButtonConfig {
        ZL: 1,
        DPAD_DOWN: 1,
        ..ButtonConfig::empty()
    },
    save_state_load: ButtonConfig {
        ZL: 1,
        DPAD_UP: 1,
        ..ButtonConfig::empty()
    },
    input_record: ButtonConfig {
        ZR: 1,
        DPAD_DOWN: 1,
        ..ButtonConfig::empty()
    },
    input_playback: ButtonConfig {
        ZR: 1,
        DPAD_UP: 1,
        ..ButtonConfig::empty()
    },
    recording_crop: OnOff::ON,
    stale_dodges: OnOff::ON,
    tech_hide: OnOff::OFF,
    update_policy: UpdatePolicy::default(),
    lra_reset: OnOff::ON,
};

pub static DEFAULTS_MENU: RwLock<TrainingModpackMenu> = RwLock::new(BASE_MENU);
pub static MENU: RwLock<TrainingModpackMenu> = RwLock::new(BASE_MENU);

pub unsafe fn create_app<'a>() -> App<'a> {
    let mut overall_menu = App::new();

    // Mash Tab
    let mut mash_tab_submenus: Vec<SubMenu> = Vec::new();
    mash_tab_submenus.push(Action::to_submenu(
        "マッシュ切替",
        "mash_state",
        "できるだけ早く行う行動",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(Action::to_submenu(
        "フォローアップ切替",
        "follow_up",
        "マッシュ後に行う行動",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(MashTrigger::to_submenu(
        "マッシュトリガー",
        "mash_triggers",
        "CPUがマッシュを行うトリガーを設定",
        ToggleMultiple,
        false,
    ));
    mash_tab_submenus.push(AttackAngle::to_submenu(
        "攻撃角度",
        "attack_angle",
        "横強攻撃などの角度付き攻撃の方向",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(ThrowOption::to_submenu(
        "投げオプション",
        "throw_state",
        "つかみ時に行う投げ技",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(MedDelay::to_submenu(
        "投げ遅延",
        "throw_delay",
        "投げを遅らせるフレーム数",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(MedDelay::to_submenu(
        "連続攻撃遅延",
        "pummel_delay",
        "つかみ後、連続攻撃を開始するまでのフレーム数",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(BoolFlag::to_submenu(
        "落下時空中攻撃",
        "falling_aerials",
        "空中攻撃を上昇中か落下中に行うか",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(BoolFlag::to_submenu(
        "大ジャンプ",
        "full_hop",
        "CPUが大ジャンプかショートホップか",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(Delay::to_submenu(
        "空中攻撃遅延",
        "aerial_delay",
        "空中攻撃を遅らせる時間",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(BoolFlag::to_submenu(
        "急降下",
        "fast_fall",
        "CPUがジャンプ中に急降下するか",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(Delay::to_submenu(
        "急降下遅延",
        "fast_fall_delay",
        "CPUが急降下を遅らせるフレーム数",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(Delay::to_submenu(
        "シールド後オフセット",
        "oos_offset",
        "マッシュ発動前にシールドが何回当たれるか",
        ToggleMultiple,
        true,
    ));
    mash_tab_submenus.push(Delay::to_submenu(
        "反応時間",
        "reaction_time",
        "シールドからマッシュ発動までの遅延フレーム数",
        ToggleMultiple,
        true,
    ));
    let mash_tab = Tab {
        id: "mash",
        title: "マッシュ設定",
        submenus: StatefulTable::with_items(NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS, mash_tab_submenus),
    };
    overall_menu.tabs.push(mash_tab);

    // Mash Override Tab
    let mut override_tab_submenus: Vec<SubMenu> = Vec::new();
    override_tab_submenus.push(Action::to_submenu(
        "崖ニュートラル起き上がり",
        "ledge_neutral_override",
        "崖ニュートラル起き上がり後のマッシュ行動",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "崖受け身ロール",
        "ledge_roll_override",
        "崖受け身ロール後のマッシュ行動",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "崖ジャンプ",
        "ledge_jump_override",
        "崖ジャンプ後のマッシュ行動",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "崖起き上がり攻撃",
        "ledge_attack_override",
        "崖起き上がり攻撃後のマッシュ行動",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "受け身動作",
        "tech_action_override",
        "受け身後のマッシュ行動",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "連打回避",
        "clatter_override",
        "連打回避状況（つかみ・埋め等）から抜け出た後のマッシュ行動",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "よろめき",
        "tumble_override",
        "よろめき状態終了後のマッシュ行動",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "ヒットストン",
        "hitstun_override",
        "ヒットストン終了後のマッシュ行動",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "パリィ",
        "parry_override",
        "パリィ後のマッシュ行動",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "シールドストン",
        "shieldstun_override",
        "シールドストン終了後のマッシュ行動",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "踏み台",
        "footstool_override",
        "踏み台状態終了後のマッシュ行動",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "着地",
        "landing_override",
        "着地後のマッシュ行動",
        ToggleMultiple,
        true,
    ));
    override_tab_submenus.push(Action::to_submenu(
        "崖奪い",
        "trump_override",
        "崖奪い状態から離れた後のマッシュ行動",
        ToggleMultiple,
        true,
    ));
    let override_tab = Tab {
        id: "override",
        title: "オーバーライド設定",
        submenus: StatefulTable::with_items(
            NX_SUBMENU_ROWS,
            NX_SUBMENU_COLUMNS,
            override_tab_submenus,
        ),
    };
    overall_menu.tabs.push(override_tab);

    // Defensive Tab
    let mut defensive_tab_submenus: Vec<SubMenu> = Vec::new();
    defensive_tab_submenus.push(Direction::to_submenu(
        "空中回避方向",
        "air_dodge_dir",
        "空中回避の角度方向",
        ToggleMultiple,
        true,
    ));
    defensive_tab_submenus.push(Direction::to_submenu(
        "DI方向",
        "di_state",
        "ヒットラグ中のDI方向",
        ToggleMultiple,
        true,
    ));
    defensive_tab_submenus.push(Direction::to_submenu(
        "SDI方向",
        "sdi_state",
        "ヒットラグ中のSDI方向",
        ToggleMultiple,
        true,
    ));
    defensive_tab_submenus.push(SdiFrequency::to_submenu(
        "SDI強度",
        "sdi_strength",
        "SDI入力の相対的強度",
        ToggleSingle,
        false,
    ));
    defensive_tab_submenus.push(ClatterFrequency::to_submenu(
        "連打回避強度",
        "clatter_strength",
        "CPUがつかみ・埋め等から抜ける速さを設定",
        ToggleSingle,
        false,
    ));
    defensive_tab_submenus.push(LedgeOption::to_submenu(
        "崖オプション",
        "ledge_state",
        "崖にいる時の行動",
        ToggleMultiple,
        true,
    ));
    defensive_tab_submenus.push(LongDelay::to_submenu(
        "崖遅延",
        "ledge_delay",
        "崖オプションを遅らせるフレーム数",
        ToggleMultiple,
        true,
    ));
    defensive_tab_submenus.push(TechFlags::to_submenu(
        "受け身オプション",
        "tech_state",
        "地面・壁に叩きつけられた時の受け身行動",
        ToggleMultiple,
        true,
    ));
    defensive_tab_submenus.push(MissTechFlags::to_submenu(
        "受け身ミスオプション",
        "miss_tech_state",
        "受け身ミス後の行動",
        ToggleMultiple,
        true,
    ));
    defensive_tab_submenus.push(Shield::to_submenu(
        "シールド切替",
        "shield_state",
        "CPUのシールド挙動",
        ToggleSingle,
        false,
    ));
    defensive_tab_submenus.push(Direction::to_submenu(
        "シールド傾け",
        "shield_tilt",
        "シールドの傾け方向",
        ToggleSingle,
        false,
    ));
    defensive_tab_submenus.push(OnOff::to_submenu(
        "しゃがみ",
        "crouch",
        "地上でCPUをしゃがませる",
        ToggleSingle,
        false,
    ));
    defensive_tab_submenus.push(OnOff::to_submenu("回避の劣化", "stale_dodges", "CPUの回避が繰り返し使用で劣化するかを制御\n(注意: この設定は元のゲームでは不可能なコンボを引き起こす可能性あり)", ToggleSingle, false));
    defensive_tab_submenus.push(OnOff::to_submenu("受け身アニメ非表示", "tech_hide", "受け身アニメと効果を7フレーム後に非表示にして受け身の始動に反応しやすくする", ToggleSingle, false));
    let defensive_tab = Tab {
        id: "defensive",
        title: "防御設定",
        submenus: StatefulTable::with_items(
            NX_SUBMENU_ROWS,
            NX_SUBMENU_COLUMNS,
            defensive_tab_submenus,
        ),
    };
    overall_menu.tabs.push(defensive_tab);

    // Input Recording Tab
    let mut input_recording_tab_submenus: Vec<SubMenu> = Vec::new();
    input_recording_tab_submenus.push(RecordSlot::to_submenu(
        "録音スロット",
        "recording_slot",
        "録音するスロットを選択",
        ToggleSingle,
        false,
    ));
    input_recording_tab_submenus.push(RecordTrigger::to_submenu(
        "録音トリガー",
        "record_trigger",
        "ボタンコンボまたはセーブステートロード時に録音を開始するか",
        ToggleSingle,
        false,
    ));
    input_recording_tab_submenus.push(RecordingDuration::to_submenu(
        "録音時間",
        "recording_duration",
        "インプット録音の持続フレーム数",
        ToggleSingle,
        false,
    ));
    input_recording_tab_submenus.push(OnOff::to_submenu(
        "録音トリミング",
        "recording_crop",
        "録音の末尾のニュートラル入力フレームを除去",
        ToggleSingle,
        false,
    ));
    input_recording_tab_submenus.push(PlaybackSlot::to_submenu(
        "再生ボタンスロット",
        "playback_button_slots",
        "ボタンコンボ押下時に再生するスロットを選択",
        ToggleMultiple,
        true,
    ));
    input_recording_tab_submenus.push(HitstunPlayback::to_submenu(
        "再生ヒットストンタイミング",
        "hitstun_playback",
        "ヒットストンマッシュトリガー発生時に入力再生を開始するタイミング",
        ToggleSingle,
        false,
    ));
    input_recording_tab_submenus.push(PlaybackSlot::to_submenu(
        "セーブステート再生",
        "save_state_playback",
        "セーブステートロード時に再生するスロットを選択",
        ToggleMultiple,
        true,
    ));
    input_recording_tab_submenus.push(OnOff::to_submenu(
        "再生マッシュ割り込み",
        "playback_mash",
        "マッシュトリガー発生時に入力再生を終了",
        ToggleSingle,
        false,
    ));
    input_recording_tab_submenus.push(OnOff::to_submenu(
        "再生ループ",
        "playback_loop",
        "トリガーされた入力再生を無限に繰り返す",
        ToggleSingle,
        false,
    ));
    let input_tab = Tab {
        id: "input",
        title: "インプット録音",
        submenus: StatefulTable::with_items(
            NX_SUBMENU_ROWS,
            NX_SUBMENU_COLUMNS,
            input_recording_tab_submenus,
        ),
    };
    overall_menu.tabs.push(input_tab);

    // Button Tab
    let mut button_tab_submenus: Vec<SubMenu> = Vec::new();
    button_tab_submenus.push(OnOff::to_submenu("メニュー開くSTARTボタン", "menu_open_start_press", "STARTを押し続けるかMINUSを押してModメニューを開く。オリジナルメニューはSTARTを押す。\nデフォルトのメニュー開く操作は十字キー上を押し続けながらBを押す。", ToggleSingle, false));
    button_tab_submenus.push(ButtonConfig::to_submenu(
        "セーブステート保存",
        "save_state_save",
        "1つのボタンを押し続けながら他を押してトリガー",
        ToggleMultiple,
        false,
    ));
    button_tab_submenus.push(ButtonConfig::to_submenu(
        "セーブステート読込",
        "save_state_load",
        "1つのボタンを押し続けながら他を押してトリガー",
        ToggleMultiple,
        false,
    ));
    button_tab_submenus.push(ButtonConfig::to_submenu(
        "インプット録音",
        "input_record",
        "1つのボタンを押し続けながら他を押してトリガー",
        ToggleMultiple,
        false,
    ));
    button_tab_submenus.push(ButtonConfig::to_submenu(
        "インプット再生",
        "input_playback",
        "1つのボタンを押し続けながら他を押してトリガー",
        ToggleMultiple,
        false,
    ));
    let button_tab = Tab {
        id: "button",
        title: "ボタン設定",
        submenus: StatefulTable::with_items(
            NX_SUBMENU_ROWS,
            NX_SUBMENU_COLUMNS,
            button_tab_submenus,
        ),
    };
    overall_menu.tabs.push(button_tab);

    // Save State Tab
    let mut save_state_tab_submenus: Vec<SubMenu> = Vec::new();
    save_state_tab_submenus.push(SaveStateMirroring::to_submenu(
        "ミラーリング",
        "save_state_mirroring",
        "ステージ中央を軸に左右にセーブステートを反転",
        ToggleSingle,
        false,
    ));
    save_state_tab_submenus.push(OnOff::to_submenu(
        "自動セーブステート",
        "save_state_autoload",
        "ファイターが死亡したときにセーブステートをロード",
        ToggleSingle,
        false,
    ));
    save_state_tab_submenus.push(SaveDamage::to_submenu(
        "ダメージ保存(CPU)",
        "save_damage_cpu",
        "セーブステートにCPUのダメージを保持するか",
        ToggleSingle,
        false,
    ));
    save_state_tab_submenus.push(DamagePercent::to_submenu(
        "ダメージ範囲(CPU)",
        "save_damage_limits_cpu",
        "セーブステートロード時にCPUに適用するランダムダメージの制限",
        Slider,
        false,
    ));
    save_state_tab_submenus.push(SaveDamage::to_submenu(
        "ダメージ保存(PL)",
        "save_damage_player",
        "セーブステートにプレイヤーのダメージを保持するか",
        ToggleSingle,
        false,
    ));
    save_state_tab_submenus.push(DamagePercent::to_submenu(
        "ダメージ範囲(PL)",
        "save_damage_limits_player",
        "セーブステートロード時にプレイヤーに適用するランダムダメージの制限",
        Slider,
        false,
    ));
    save_state_tab_submenus.push(OnOff::to_submenu(
        "セーブステート有効",
        "save_state_enable",
        "セーブステートを有効化！シールド+下アピールで保存、シールド+上アピールでロード。",
        ToggleSingle,
        false,
    ));
    save_state_tab_submenus.push(SaveStateSlot::to_submenu(
        "セーブステートスロット",
        "save_state_slot",
        "異なるスロットにセーブ・ロード",
        ToggleSingle,
        false,
    ));
    save_state_tab_submenus.push(SaveStateSlot::to_submenu(
        "スロットランダム",
        "randomize_slots",
        "セーブステートロード時にランダム化するスロット",
        ToggleMultiple,
        true,
    ));
    save_state_tab_submenus.push(CharacterItem::to_submenu(
        "キャラクターアイテム",
        "character_item",
        "セーブステートロード時にプレイヤーのファイターに渡すアイテム",
        ToggleSingle,
        false,
    ));
    save_state_tab_submenus.push(BuffOption::to_submenu(
        "バフオプション",
        "buff_state",
        "セーブステートロード時に各ファイターに適用するバフ",
        ToggleMultiple,
        false,
    ));
    let save_state_tab = Tab {
        id: "save_state",
        title: "セーブステート",
        submenus: StatefulTable::with_items(
            NX_SUBMENU_ROWS,
            NX_SUBMENU_COLUMNS,
            save_state_tab_submenus,
        ),
    };
    overall_menu.tabs.push(save_state_tab);

    // Miscellaneous Tab
    let mut misc_tab_submenus: Vec<SubMenu> = Vec::new();
    misc_tab_submenus.push(OnOff::to_submenu("フレーム有利", "frame_advantage", "プレイヤーとCPUの行動可能時間の差を表示\n(CPUがマッシュしていないことが条件)", ToggleSingle, false));
    misc_tab_submenus.push(OnOff::to_submenu(
        "ヒットボックス表示",
        "hitbox_vis",
        "アクティブなヒットボックスを視覚表示（他のエフェクト非表示）",
        ToggleSingle,
        false,
    ));
    misc_tab_submenus.push(InputDisplay::to_submenu(
        "インプット表示",
        "input_display",
        "画面左の入力ログを表示",
        ToggleSingle,
        false,
    ));
    misc_tab_submenus.push(OnOff::to_submenu(
        "インプット表示ステータス",
        "input_display_status",
        "発生したステータスでインプットログをグループ化",
        ToggleSingle,
        false,
    ));
    misc_tab_submenus.push(Delay::to_submenu(
        "インプット遅延",
        "input_delay",
        "プレイヤー入力を遅らせるフレーム数",
        ToggleSingle,
        false,
    ));
    misc_tab_submenus.push(OnOff::to_submenu(
        "ステージギミック",
        "stage_hazards",
        "ステージギミックのオン/オフ",
        ToggleSingle,
        false,
    ));
    misc_tab_submenus.push(OnOff::to_submenu(
        "HUD",
        "hud",
        "UIの要素を表示/非表示",
        ToggleSingle,
        false,
    ));
    misc_tab_submenus.push(UpdatePolicy::to_submenu(
        "自動アップデート",
        "update_policy",
        "自動適用するトレーニングModのアップデートの種類（本体のみ）",
        ToggleSingle,
        false,
    ));
    misc_tab_submenus.push(OnOff::to_submenu(
        "L+R+Aリセット",
        "lra_reset",
        "L+R+Aでトレーニングルームをリセット",
        ToggleSingle,
        false,
    ));
    let misc_tab = Tab {
        id: "misc",
        title: "その他設定",
        submenus: StatefulTable::with_items(NX_SUBMENU_ROWS, NX_SUBMENU_COLUMNS, misc_tab_submenus),
    };
    overall_menu.tabs.push(misc_tab);

    // Ensure that a tab is always selected
    if overall_menu.tabs.get_selected().is_none() {
        overall_menu.tabs.state.select(Some(0));
    }

    overall_menu
}
