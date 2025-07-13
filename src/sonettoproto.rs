pub use prost::Message;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TalentCubeInfo {
    #[prost(int32, optional, tag = "1")]
    pub cube_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub direction: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub pos_x: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub pos_y: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssistHeroInfo {
    #[prost(int64, optional, tag = "1")]
    pub hero_uid: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub user_id: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "4")]
    pub user_level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub portrait: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub bg: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "7")]
    pub is_friend: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "8")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "9")]
    pub level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "10")]
    pub rank: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "11")]
    pub skin: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "12")]
    pub passive_skill_level: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "13")]
    pub ex_skill_level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "14")]
    pub talent: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "15")]
    pub talent_cube_infos: ::prost::alloc::vec::Vec<TalentCubeInfo>,
    #[prost(int32, optional, tag = "16")]
    pub balance_level: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "17")]
    pub is_open_talent: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "18")]
    pub style: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "19")]
    pub destiny_rank: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "20")]
    pub destiny_level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "21")]
    pub destiny_stone: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroAttribute {
    #[prost(int32, optional, tag = "1")]
    pub hp: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub attack: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub defense: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub mdefense: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub technic: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub multi_hp_idx: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub multi_hp_num: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroExAttribute {
    #[prost(int32, optional, tag = "1")]
    pub cri: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub recri: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub cri_dmg: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub cri_def: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub add_dmg: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub drop_dmg: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroSpAttribute {
    #[prost(int32, optional, tag = "1")]
    pub revive: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub heal: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub absorb: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub defense_ignore: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub clutch: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub final_add_dmg: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub final_drop_dmg: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub normal_skill_rate: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "9")]
    pub play_add_rate: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "10")]
    pub play_drop_rate: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "11")]
    pub dizzy_resistances: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "12")]
    pub sleep_resistances: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "13")]
    pub petrified_resistances: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "14")]
    pub frozen_resistances: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "15")]
    pub disarm_resistances: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "16")]
    pub forbid_resistances: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "17")]
    pub seal_resistances: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "18")]
    pub cant_get_exskill_resistances: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "19")]
    pub del_ex_point_resistances: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "20")]
    pub stress_up_resistances: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "21")]
    pub control_resilience: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "22")]
    pub del_ex_point_resilience: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "23")]
    pub stress_up_resilience: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "24")]
    pub charm_resistances: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "25")]
    pub rebound_dmg: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "26")]
    pub extra_dmg: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "27")]
    pub reuse_dmg: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "28")]
    pub big_skill_rate: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroAllAttribute {
    #[prost(message, optional, tag = "1")]
    pub base_attr: ::core::option::Option<HeroAttribute>,
    #[prost(message, optional, tag = "2")]
    pub ex_attr: ::core::option::Option<HeroExAttribute>,
    #[prost(message, optional, tag = "3")]
    pub sp_attr: ::core::option::Option<HeroSpAttribute>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroEquipAttribute {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub equip_attr: ::core::option::Option<HeroAttribute>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroGroupEquip {
    #[prost(int32, optional, tag = "1")]
    pub index: ::core::option::Option<i32>,
    #[prost(int64, repeated, packed = "false", tag = "2")]
    pub equip_uid: ::prost::alloc::vec::Vec<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroGroupInfo {
    #[prost(int32, required, tag = "1")]
    pub group_id: i32,
    #[prost(int64, repeated, packed = "false", tag = "2")]
    pub hero_list: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, optional, tag = "3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "4")]
    pub cloth_id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "5")]
    pub equips: ::prost::alloc::vec::Vec<HeroGroupEquip>,
    #[prost(message, repeated, tag = "6")]
    pub activity104_equips: ::prost::alloc::vec::Vec<HeroGroupEquip>,
    #[prost(int32, optional, tag = "7")]
    pub assist_boss_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkinInfo {
    #[prost(int32, optional, tag = "1")]
    pub skin: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub expire_sec: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TalentTemplateInfo {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub talent_cube_infos: ::prost::alloc::vec::Vec<TalentCubeInfo>,
    #[prost(string, optional, tag = "3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "4")]
    pub style: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroInfo {
    #[prost(int64, required, tag = "1")]
    pub uid: i64,
    #[prost(int64, required, tag = "2")]
    pub user_id: i64,
    #[prost(int32, required, tag = "3")]
    pub hero_id: i32,
    #[prost(int64, optional, tag = "4")]
    pub create_time: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "5")]
    pub level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub exp: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub rank: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub breakthrough: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "9")]
    pub skin: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "10")]
    pub faith: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "11")]
    pub active_skill_level: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "12")]
    pub passive_skill_level: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "13")]
    pub ex_skill_level: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "14")]
    pub voice: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "15")]
    pub voice_heard: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "16")]
    pub skin_info_list: ::prost::alloc::vec::Vec<SkinInfo>,
    #[prost(message, optional, tag = "17")]
    pub base_attr: ::core::option::Option<HeroAttribute>,
    #[prost(message, optional, tag = "18")]
    pub ex_attr: ::core::option::Option<HeroExAttribute>,
    #[prost(message, optional, tag = "19")]
    pub sp_attr: ::core::option::Option<HeroSpAttribute>,
    #[prost(message, repeated, tag = "20")]
    pub equip_attr_list: ::prost::alloc::vec::Vec<HeroEquipAttribute>,
    #[prost(bool, optional, tag = "21")]
    pub is_new: ::core::option::Option<bool>,
    #[prost(int32, repeated, packed = "false", tag = "22")]
    pub item_unlock: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "23")]
    pub talent: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "24")]
    pub talent_cube_infos: ::prost::alloc::vec::Vec<TalentCubeInfo>,
    #[prost(int64, optional, tag = "25")]
    pub default_equip_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "26")]
    pub duplicate_count: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "27")]
    pub talent_templates: ::prost::alloc::vec::Vec<TalentTemplateInfo>,
    #[prost(int32, optional, tag = "28")]
    pub use_talent_template_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "29")]
    pub talent_style_unlock: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "30")]
    pub talent_style_red: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "31")]
    pub is_favor: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "32")]
    pub destiny_rank: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "33")]
    pub destiny_level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "34")]
    pub destiny_stone: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "35")]
    pub destiny_stone_unlock: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "36")]
    pub red_dot: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroSimpleInfo {
    #[prost(int32, required, tag = "1")]
    pub hero_id: i32,
    #[prost(int32, optional, tag = "2")]
    pub level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub rank: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub ex_skill_level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub skin: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkillInfo {
    #[prost(int32, optional, tag = "1")]
    pub skill_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub max_level: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LimiterResNo {
    #[prost(int32, optional, tag = "1")]
    pub add_emblem: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub use_limit_buff_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeInfo {
    #[prost(int32, optional, tag = "1")]
    pub layer: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub stage: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub node_id: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "4")]
    pub last_node_list: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "5")]
    pub next_node_list: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "6")]
    pub event_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "7")]
    pub event_data: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "8")]
    pub path_way: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "9")]
    pub interactive9drop: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "10")]
    pub interactive10drop: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "11")]
    pub fog: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeBagPos {
    #[prost(int32, optional, tag = "1")]
    pub row: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub col: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeItemAttr {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub attr_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int64, repeated, packed = "false", tag = "2")]
    pub attr_vals: ::prost::alloc::vec::Vec<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeItem {
    #[prost(int64, optional, tag = "1")]
    pub id: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub item_id: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub hold_items: ::prost::alloc::vec::Vec<i32>,
    #[prost(int64, repeated, packed = "false", tag = "4")]
    pub hold_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(message, optional, tag = "5")]
    pub attr: ::core::option::Option<RougeItemAttr>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeItemLayoutRelation {
    #[prost(int32, optional, tag = "1")]
    pub effect_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub show_type: ::core::option::Option<i32>,
    #[prost(int64, repeated, packed = "false", tag = "3")]
    pub true_guids: ::prost::alloc::vec::Vec<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeItemLayout {
    #[prost(message, optional, tag = "1")]
    pub pos: ::core::option::Option<RougeBagPos>,
    #[prost(int32, optional, tag = "2")]
    pub rotation: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub item: ::core::option::Option<RougeItem>,
    #[prost(int32, repeated, packed = "false", tag = "4")]
    pub base_effects: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "5")]
    pub relations: ::prost::alloc::vec::Vec<RougeItemLayoutRelation>,
    #[prost(message, optional, tag = "6")]
    pub attr: ::core::option::Option<RougeItemAttr>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeBag {
    #[prost(message, repeated, tag = "1")]
    pub layouts: ::prost::alloc::vec::Vec<RougeItemLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeBattleHero {
    #[prost(int32, optional, tag = "1")]
    pub index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "3")]
    pub equip_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "4")]
    pub support_hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub support_hero_skill: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeEffectInfo {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub effect_id: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeEntrustInfo {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub count: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeGameLimiterInfo {
    #[prost(int32, optional, tag = "1")]
    pub risk_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub risk_value: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub limit_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "4")]
    pub limit_buff_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeHeroInfo {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub stress_value: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub stress_value_limit: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeHeroLife {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub life: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeLastReward {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub param: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeLayerChoiceInfo {
    #[prost(int32, optional, tag = "1")]
    pub layer_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub map_rule_id: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub cur_layer_collection: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "4")]
    pub map_rule_can_fresh_num: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeLayerInfo {
    #[prost(int32, optional, tag = "1")]
    pub layer_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub cur_stage: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub cur_node: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "4")]
    pub node_info: ::prost::alloc::vec::Vec<NodeInfo>,
    #[prost(message, optional, tag = "5")]
    pub layer_choice_info: ::core::option::Option<RougeLayerChoiceInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougePieceInfo {
    #[prost(int32, optional, tag = "1")]
    pub index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub talk_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub select_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "5")]
    pub trigger_str: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "6")]
    pub finish: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeMiddleLayerInfo {
    #[prost(int32, optional, tag = "1")]
    pub layer_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub middle_layer_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub position_index: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "4")]
    pub piece_info: ::prost::alloc::vec::Vec<RougePieceInfo>,
    #[prost(message, repeated, tag = "5")]
    pub layer_choice_info: ::prost::alloc::vec::Vec<RougeLayerChoiceInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeMapSkillInfo {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub use_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub step_record: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeMapInfo {
    #[prost(int32, optional, tag = "1")]
    pub map_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub layer_info: ::core::option::Option<RougeLayerInfo>,
    #[prost(message, optional, tag = "3")]
    pub middle_layer_info: ::core::option::Option<RougeMiddleLayerInfo>,
    #[prost(int32, optional, tag = "4")]
    pub cur_interactive_index: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "5")]
    pub cur_interactive: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub interactive_json: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "7")]
    pub rouge_entrust: ::core::option::Option<RougeEntrustInfo>,
    #[prost(message, repeated, tag = "8")]
    pub map_skill: ::prost::alloc::vec::Vec<RougeMapSkillInfo>,
    #[prost(int32, optional, tag = "9")]
    pub monster_rule_fresh_num: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "10")]
    pub monster_rule_can_fresh_num: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "11")]
    pub choice_collection: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeTeamInfo {
    #[prost(message, repeated, tag = "1")]
    pub battle_hero_list: ::prost::alloc::vec::Vec<RougeBattleHero>,
    #[prost(message, repeated, tag = "2")]
    pub hero_life_list: ::prost::alloc::vec::Vec<RougeHeroLife>,
    #[prost(message, optional, tag = "3")]
    pub assist_hero_info: ::core::option::Option<AssistHeroInfo>,
    #[prost(message, repeated, tag = "4")]
    pub hero_info_list: ::prost::alloc::vec::Vec<RougeHeroInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeWarehouse {
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<RougeItem>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeTalent {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub is_active: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeTalentTree {
    #[prost(message, repeated, tag = "1")]
    pub rouge_talent: ::prost::alloc::vec::Vec<RougeTalent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeInfo {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub version: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "3")]
    pub state: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub difficulty: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "5")]
    pub last_reward: ::prost::alloc::vec::Vec<RougeLastReward>,
    #[prost(int32, optional, tag = "6")]
    pub select_reward_num: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub style: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub team_level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "9")]
    pub team_exp: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "10")]
    pub team_size: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "11")]
    pub coin: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "12")]
    pub talent_point: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "13")]
    pub power: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "14")]
    pub map_info: ::core::option::Option<RougeMapInfo>,
    #[prost(message, optional, tag = "15")]
    pub team_info: ::core::option::Option<RougeTeamInfo>,
    #[prost(message, optional, tag = "16")]
    pub bag: ::core::option::Option<RougeBag>,
    #[prost(message, optional, tag = "17")]
    pub warehouse: ::core::option::Option<RougeWarehouse>,
    #[prost(message, optional, tag = "18")]
    pub talent_tree: ::core::option::Option<RougeTalentTree>,
    #[prost(int32, optional, tag = "19")]
    pub end_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "20")]
    pub retry_num: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "21")]
    pub effect_info: ::prost::alloc::vec::Vec<RougeEffectInfo>,
    #[prost(int32, optional, tag = "22")]
    pub power_limit: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "23")]
    pub limiter_info: ::core::option::Option<RougeGameLimiterInfo>,
    #[prost(int32, optional, tag = "24")]
    pub gamint32: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeItemLayoutEffectUpdate {
    #[prost(int64, optional, tag = "1")]
    pub id: ::core::option::Option<i64>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub base_effects: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "3")]
    pub relations: ::prost::alloc::vec::Vec<RougeItemLayoutRelation>,
    #[prost(message, optional, tag = "4")]
    pub attr: ::core::option::Option<RougeItemAttr>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeReviewInfo {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub player_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub player_level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub portrait: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "5")]
    pub finish_time: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "6")]
    pub difficulty: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub style: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub team_level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "9")]
    pub collection_num: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "10")]
    pub gain_coin: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "11")]
    pub layouts: ::prost::alloc::vec::Vec<RougeItemLayout>,
    #[prost(message, optional, tag = "12")]
    pub team_info: ::core::option::Option<RougeTeamInfo>,
    #[prost(int32, optional, tag = "13")]
    pub end_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "14")]
    pub layer_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "15")]
    pub middle_layer_id: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "16")]
    pub version: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "17")]
    pub risk_value: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeResultInfo {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub init_hero_id: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, optional, tag = "3")]
    pub collection2_num_str: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub compose_res2_num_str: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, repeated, packed = "false", tag = "5")]
    pub finish_event_id: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "6")]
    pub finish_entrust_id: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "7")]
    pub consume_coin: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub consume_power: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "9")]
    pub max_damage: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "10")]
    pub dead_num: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "11")]
    pub revivint32: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "12")]
    pub repair_shop_num: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "13")]
    pub displacint32: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "14")]
    pub step_num: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "15")]
    pub badge2_score: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "16")]
    pub normal_fight2_score: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "17")]
    pub difficult_fight2_score: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "18")]
    pub dangerous_fight2_score: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "19")]
    pub collection2_score: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "20")]
    pub layer2_score: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "21")]
    pub entrust2_score: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "22")]
    pub end2_score: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "23")]
    pub score_reward: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "24")]
    pub before_score: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "25")]
    pub final_score: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "26")]
    pub add_point: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "27")]
    pub remain_score2_point: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "28")]
    pub add_genius_point: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "29")]
    pub remain_score2_genius_point: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "30")]
    pub review_info: ::core::option::Option<RougeReviewInfo>,
    #[prost(int32, optional, tag = "31")]
    pub pre_remain_score2_point: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "32")]
    pub pre_remain_score2_genius_point: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "33")]
    pub limiter_res_no: ::core::option::Option<LimiterResNo>,
    #[prost(int32, optional, tag = "34")]
    pub extra_add_point: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeSimpleMapInfo {
    #[prost(int32, optional, tag = "1")]
    pub map_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub layer_info: ::core::option::Option<RougeLayerInfo>,
    #[prost(message, optional, tag = "3")]
    pub middle_layer_info: ::core::option::Option<RougeMiddleLayerInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewReddotNo {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlockSkillNo {
    #[prost(string, optional, tag = "1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeGameRecordInfo {
    #[prost(int32, optional, tag = "1")]
    pub max_difficulty: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub pass_layer_id: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub pass_event_id: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "4")]
    pub pass_end_id: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "5")]
    pub pass_entrust_id: ::prost::alloc::vec::Vec<i32>,
    #[prost(int64, optional, tag = "6")]
    pub last_game_time: ::core::option::Option<i64>,
    #[prost(int32, repeated, packed = "false", tag = "7")]
    pub pass_collections: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "8")]
    pub unlock_story_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "9")]
    pub unlock_illustration_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "10")]
    pub dlc_version_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "11")]
    pub unlock_skills: ::prost::alloc::vec::Vec<UnlockSkillNo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeLimitBuffNo {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub cd: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeLimiterClientNo {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub limit_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub limit_buff_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeLimiterInfo {
    #[prost(message, optional, tag = "1")]
    pub client_no: ::core::option::Option<RougeLimiterClientNo>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub unlock_limit_group_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "3")]
    pub unlock_limit_buffs: ::prost::alloc::vec::Vec<RougeLimitBuffNo>,
    #[prost(int32, optional, tag = "4")]
    pub emblem: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeOutsideBonusStageNo {
    #[prost(int32, optional, tag = "1")]
    pub stage: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub bonus_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeOutsideBonusNo {
    #[prost(message, repeated, tag = "1")]
    pub bonus_stages: ::prost::alloc::vec::Vec<RougeOutsideBonusStageNo>,
    #[prost(bool, optional, tag = "2")]
    pub is_new_stage: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeOutsideInfo {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub genius_point: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub genius_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "4")]
    pub point: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub have_get_point: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "6")]
    pub bonus: ::core::option::Option<RougeOutsideBonusNo>,
    #[prost(message, repeated, tag = "7")]
    pub review: ::prost::alloc::vec::Vec<RougeReviewInfo>,
    #[prost(message, optional, tag = "8")]
    pub game_record_info: ::core::option::Option<RougeGameRecordInfo>,
    #[prost(bool, optional, tag = "9")]
    pub is_genius_new_stage: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "10")]
    pub limiter_info: ::core::option::Option<RougeLimiterInfo>,
    #[prost(int32, optional, tag = "11")]
    pub cur_extra_point: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRougeOutsideInfoReply {
    #[prost(message, optional, tag = "1")]
    pub rouge_info: ::core::option::Option<RougeOutsideInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRougeOutsideInfoRequest {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeActiveGeniusReply {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub genius_id: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub genius_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeActiveGeniusRequest {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub genius_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeDlcSettingSaveReply {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub dlc_version_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeDlcSettingSaveRequest {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub dlc_version_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeGetNewReddotInfoReply {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub new_reddots: ::prost::alloc::vec::Vec<NewReddotNo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeGetNewReddotInfoRequest {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeGetUnlockCollectionsReply {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub unlock_collection_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeGetUnlockCollectionsRequest {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeLimiterSettingSaveReply {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub client_no: ::core::option::Option<RougeLimiterClientNo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeLimiterSettingSaveRequest {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub client_no: ::core::option::Option<RougeLimiterClientNo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeLimiterSpeedUpBuffCdReply {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub limit_buff_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeLimiterSpeedUpBuffCdRequest {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub limit_buff_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeLimiterUnlockBuffReply {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub limit_buff_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeLimiterUnlockBuffRequest {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub limit_buff_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeMarkBonusNewStageReply {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeMarkBonusNewStageRequest {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeMarkGeniusNewStageReply {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeMarkGeniusNewStageRequest {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeMarkNewReddotReply {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeMarkNewReddotRequest {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeReceivePointBonusReply {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub bonus_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub bonus_stage: ::core::option::Option<RougeOutsideBonusStageNo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeReceivePointBonusRequest {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub bonus_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeReddotUpdatePush {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub new_reddots: ::prost::alloc::vec::Vec<NewReddotNo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeUnlockStoryReply {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub story_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeUnlockStoryRequest {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub story_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeUpdateGeniusPointPush {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub genius_point: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RougeUpdatePointPush {
    #[prost(int32, optional, tag = "1")]
    pub season: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub point: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerCardInfo {
    #[prost(string, repeated, tag = "1")]
    pub show_settings: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub progress_setting: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub base_setting: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub hero_cover: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "5")]
    pub theme_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "6")]
    pub show_achievement: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub critter: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub room_collection: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "9")]
    pub weekwalk_deep_layer_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "10")]
    pub explore_collection: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "11")]
    pub rouge_difficulty: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "12")]
    pub act128_sss_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "13")]
    pub achievement_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "14")]
    pub assist_times: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "15")]
    pub hero_cover_times: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "16")]
    pub max_faith_hero_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "17")]
    pub total_cost_power: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "18")]
    pub skin_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "19")]
    pub tower_layer: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "20")]
    pub tower_boss_pass_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "21")]
    pub hero_max_level_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "22")]
    pub weekwalk_ver2_platinum_cup: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerCloth {
    #[prost(int32, optional, tag = "1")]
    pub cloth_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub exp: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerClothInfo {
    #[prost(message, repeated, tag = "1")]
    pub clothes: ::prost::alloc::vec::Vec<PlayerCloth>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerInfo {
    #[prost(uint64, optional, tag = "1")]
    pub user_id: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub portrait: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub exp: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "6")]
    pub signature: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub birthday: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "8")]
    pub show_heros: ::prost::alloc::vec::Vec<HeroSimpleInfo>,
    #[prost(int64, optional, tag = "9")]
    pub register_time: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "10")]
    pub hero_rare_nn_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "11")]
    pub hero_rare_n_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "12")]
    pub hero_rare_r_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "13")]
    pub hero_rare_sr_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "14")]
    pub hero_rare_ssr_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "15")]
    pub last_episode_id: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "16")]
    pub last_login_time: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "17")]
    pub last_logout_time: ::core::option::Option<i64>,
    #[prost(int32, repeated, packed = "false", tag = "18")]
    pub character_age: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, optional, tag = "19")]
    pub show_achievement: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "20")]
    pub bg: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "21")]
    pub total_login_days: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimplePlayerInfo {
    #[prost(uint64, optional, tag = "1")]
    pub user_id: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub portrait: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub level: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "5")]
    pub is_online: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "6")]
    pub zone_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub datetime: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientStatBaseInfoReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientStatBaseInfoRequest {
    #[prost(string, optional, tag = "1")]
    pub info: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatInfoPush {
    #[prost(bool, optional, tag = "1")]
    pub frist_charge: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "2")]
    pub total_charge_amount: ::core::option::Option<i64>,
    #[prost(bool, optional, tag = "3")]
    pub is_first_login: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "4")]
    pub player_info: ::core::option::Option<PlayerInfo>,
    #[prost(string, optional, tag = "5")]
    pub user_tag: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateClientStatBaseInfoReply {
    #[prost(int32, optional, tag = "1")]
    pub account_bind_bonus: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateClientStatBaseInfoRequest {
    #[prost(string, optional, tag = "1")]
    pub info: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServerTimeReply {
    #[prost(uint64, optional, tag = "1")]
    pub server_time: ::core::option::Option<u64>,
    #[prost(int64, optional, tag = "2")]
    pub offset_time: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServerTimeRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenInfo {
    #[prost(int32, required, tag = "1")]
    pub id: i32,
    #[prost(bool, required, tag = "2")]
    pub is_open: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOpenPush {
    #[prost(message, repeated, tag = "1")]
    pub open_infos: ::prost::alloc::vec::Vec<OpenInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClothUpdatePush {
    #[prost(message, optional, tag = "1")]
    pub update_infos: ::core::option::Option<PlayerClothInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePlayerReply {
    #[prost(message, optional, tag = "1")]
    pub player_info: ::core::option::Option<PlayerInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePlayerRequest {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssistBonusReply {
    #[prost(int32, optional, tag = "1")]
    pub assist_bonus: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub has_receive_assist_bonus: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssistBonusRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClothInfoReply {
    #[prost(message, optional, tag = "1")]
    pub cloth_infos: ::core::option::Option<PlayerClothInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClothInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOtherPlayerInfoReply {
    #[prost(message, optional, tag = "1")]
    pub player_info: ::core::option::Option<PlayerInfo>,
    #[prost(string, optional, tag = "2")]
    pub hero_cover: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOtherPlayerInfoRequest {
    #[prost(int64, optional, tag = "1")]
    pub user_id: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPlayerInfoReply {
    #[prost(message, optional, tag = "1")]
    pub player_info: ::core::option::Option<PlayerInfo>,
    #[prost(message, repeated, tag = "2")]
    pub openinfos: ::prost::alloc::vec::Vec<OpenInfo>,
    #[prost(bool, optional, tag = "3")]
    pub can_rename: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub main_thumbnail: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "5")]
    pub ext_rename: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPlayerInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimpleProperty {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub property: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSimplePropertyReply {
    #[prost(message, repeated, tag = "1")]
    pub simple_properties: ::prost::alloc::vec::Vec<SimpleProperty>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSimplePropertyRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkMainThumbnailReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkMainThumbnailRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerInfoPush {
    #[prost(message, optional, tag = "1")]
    pub player_info: ::core::option::Option<PlayerInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveAssistBonusReply {
    #[prost(int32, optional, tag = "1")]
    pub assist_bonus: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub has_receive_assist_bonus: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveAssistBonusRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameReply {
    #[prost(bool, optional, tag = "1")]
    pub can_rename: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "2")]
    pub ext_rename: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameRequest {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub guide_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub step_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerResultCodePush {
    #[prost(int32, optional, tag = "1")]
    pub result_code: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetBirthdayReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetBirthdayRequest {
    #[prost(string, optional, tag = "1")]
    pub birthday: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCharacterAgeReply {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub character_age: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCharacterAgeRequest {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub character_age: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMainSceneSkinReply {
    #[prost(int32, optional, tag = "1")]
    pub item_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMainSceneSkinRequest {
    #[prost(int32, optional, tag = "1")]
    pub item_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPlayerBgReply {
    #[prost(int32, optional, tag = "1")]
    pub bg_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPlayerBgRequest {
    #[prost(int32, optional, tag = "1")]
    pub bg_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPortraitReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPortraitRequest {
    #[prost(int32, optional, tag = "1")]
    pub portrait: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetShowHeroUniqueIdsReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetShowHeroUniqueIdsRequest {
    #[prost(int64, repeated, packed = "false", tag = "1")]
    pub show_hero_unique_ids: ::prost::alloc::vec::Vec<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSignatureReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSignatureRequest {
    #[prost(string, optional, tag = "1")]
    pub signature: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSimplePropertyReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSimplePropertyRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub property: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimplePropertyPush {
    #[prost(message, optional, tag = "1")]
    pub simple_property: ::core::option::Option<SimpleProperty>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseCdKeyReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseCdKeyRequset {
    #[prost(string, optional, tag = "1")]
    pub gift_code: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyPowerReply {
    #[prost(int32, optional, tag = "1")]
    pub can_buy_count: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyPowerRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Currency {
    #[prost(uint32, optional, tag = "1")]
    pub currency_id: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2")]
    pub quantity: ::core::option::Option<i32>,
    #[prost(uint64, optional, tag = "3")]
    pub last_recover_time: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "4")]
    pub expired_time: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrencyChangePush {
    #[prost(message, repeated, tag = "1")]
    pub change_currency: ::prost::alloc::vec::Vec<Currency>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeDiamondReply {
    #[prost(int32, optional, tag = "1")]
    pub exchange_diamond: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub op_type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeDiamondRequest {
    #[prost(int32, optional, tag = "1")]
    pub exchange_diamond: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub op_type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBuyPowerInfoReply {
    #[prost(int32, optional, tag = "1")]
    pub can_buy_count: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBuyPowerInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrencyListReply {
    #[prost(message, repeated, tag = "1")]
    pub currency_list: ::prost::alloc::vec::Vec<Currency>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrencyListRequest {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub currency_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishGuideReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishGuideRequest {
    #[prost(int32, required, tag = "1")]
    pub guide_id: i32,
    #[prost(int32, required, tag = "2")]
    pub step_id: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuideInfo {
    #[prost(int32, required, tag = "1")]
    pub guide_id: i32,
    #[prost(int32, required, tag = "2")]
    pub step_id: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuideInfoReply {
    #[prost(message, repeated, tag = "1")]
    pub guide_infos: ::prost::alloc::vec::Vec<GuideInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuideInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGuidePush {
    #[prost(message, repeated, tag = "1")]
    pub guide_infos: ::prost::alloc::vec::Vec<GuideInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedDotInfo {
    #[prost(int64, required, tag = "1")]
    pub id: i64,
    #[prost(int32, required, tag = "2")]
    pub value: i32,
    #[prost(int32, optional, tag = "3")]
    pub time: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub ext: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedDotGroup {
    #[prost(int32, required, tag = "1")]
    pub define_id: i32,
    #[prost(message, repeated, tag = "2")]
    pub infos: ::prost::alloc::vec::Vec<RedDotInfo>,
    #[prost(bool, optional, tag = "3")]
    pub replace_all: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRedDotInfosReply {
    #[prost(message, repeated, tag = "1")]
    pub red_dot_infos: ::prost::alloc::vec::Vec<RedDotGroup>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRedDotInfosRequest {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShowRedDotReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShowRedDotRequest {
    #[prost(int32, optional, tag = "1")]
    pub define_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub is_visible: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRedDotPush {
    #[prost(message, repeated, tag = "1")]
    pub red_dot_infos: ::prost::alloc::vec::Vec<RedDotGroup>,
    #[prost(bool, optional, tag = "2")]
    pub replace_all: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHeroBirthdayReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHeroBirthdayRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonthCardHistory {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub start_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub end_time: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSignInInfoReply {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub has_sign_in_days: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "2")]
    pub addup_sign_in_day: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub has_get_addup_bonus: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "4")]
    pub open_function_time: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "5")]
    pub has_month_card_days: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "6")]
    pub month_card_history: ::prost::alloc::vec::Vec<MonthCardHistory>,
    #[prost(int32, repeated, packed = "false", tag = "7")]
    pub birthday_hero_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "8")]
    pub reward_mark: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSignInInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignInAddupReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignInAddupRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignInHistoryReply {
    #[prost(int32, optional, tag = "1")]
    pub month: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub has_sign_in_days: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub has_month_card_days: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "4")]
    pub birthday_hero_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignInHistoryRequest {
    #[prost(int32, optional, tag = "1")]
    pub month: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignInReply {
    #[prost(int32, optional, tag = "1")]
    pub day: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub birthday_hero_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignInRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignInTotalRewardAllReply {
    #[prost(int32, optional, tag = "1")]
    pub mark: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignInTotalRewardAllRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignInTotalRewardReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub mark: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignInTotalRewardRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyGoodsReply {
    #[prost(int32, required, tag = "1")]
    pub store_id: i32,
    #[prost(int32, required, tag = "2")]
    pub goods_id: i32,
    #[prost(int32, required, tag = "3")]
    pub num: i32,
    #[prost(int32, optional, tag = "4")]
    pub select_cost: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyGoodsRequest {
    #[prost(int32, required, tag = "1")]
    pub store_id: i32,
    #[prost(int32, required, tag = "2")]
    pub goods_id: i32,
    #[prost(int32, required, tag = "3")]
    pub num: i32,
    #[prost(int32, optional, tag = "4")]
    pub select_cost: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoodsInfo {
    #[prost(int32, required, tag = "1")]
    pub goods_id: i32,
    #[prost(int32, required, tag = "2")]
    pub buy_count: i32,
    #[prost(int64, optional, tag = "3")]
    pub offline_time: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreInfo {
    #[prost(int32, required, tag = "1")]
    pub id: i32,
    #[prost(int64, required, tag = "2")]
    pub next_refresh_time: i64,
    #[prost(message, repeated, tag = "3")]
    pub goods_infos: ::prost::alloc::vec::Vec<GoodsInfo>,
    #[prost(int64, optional, tag = "4")]
    pub offline_time: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoreInfosReply {
    #[prost(message, repeated, tag = "1")]
    pub store_infos: ::prost::alloc::vec::Vec<StoreInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoreInfosRequest {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub store_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadStoreNewReply {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub goods_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadStoreNewRequest {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub goods_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoReadMailPush {
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    pub incr_ids: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMailBatchReply {
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    pub incr_ids: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMailBatchRequest {
    #[prost(uint32, optional, tag = "1")]
    pub r#type: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMailsPush {
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    pub incr_ids: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mail {
    #[prost(uint64, optional, tag = "1")]
    pub incr_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub mail_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "3")]
    pub params: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub attachment: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "5")]
    pub state: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "6")]
    pub create_time: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "7")]
    pub sender: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub content: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub copy: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "11")]
    pub expire_time: ::core::option::Option<u64>,
    #[prost(int32, optional, tag = "12")]
    pub sender_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "13")]
    pub jump_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "14")]
    pub jump: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllMailsReply {
    #[prost(message, repeated, tag = "1")]
    pub mails: ::prost::alloc::vec::Vec<Mail>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllMailsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkMailJumpReply {
    #[prost(uint64, optional, tag = "1")]
    pub incr_id: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkMailJumpRequest {
    #[prost(uint64, optional, tag = "1")]
    pub incr_id: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewMailPush {
    #[prost(message, optional, tag = "1")]
    pub mail: ::core::option::Option<Mail>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadMailBatchReply {
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    pub incr_ids: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadMailBatchRequest {
    #[prost(uint32, optional, tag = "1")]
    pub r#type: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadMailReply {
    #[prost(uint64, optional, tag = "1")]
    pub incr_id: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadMailRequest {
    #[prost(uint64, optional, tag = "1")]
    pub incr_id: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityInfo {
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "2")]
    pub start_time: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "3")]
    pub end_time: ::core::option::Option<u64>,
    #[prost(bool, optional, tag = "4")]
    pub online: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "5")]
    pub is_new_stage: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "6")]
    pub current_stage: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "7")]
    pub is_unlock: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "8")]
    pub is_receive_all_bonus: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityNewStageReadReply {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityNewStageReadRequest {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndActivityPush {
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetActivityInfosReply {
    #[prost(message, repeated, tag = "1")]
    pub activity_infos: ::prost::alloc::vec::Vec<ActivityInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetActivityInfosRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetActivityInfosWithParamReply {
    #[prost(message, repeated, tag = "1")]
    pub activity_infos: ::prost::alloc::vec::Vec<ActivityInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetActivityInfosWithParamRequest {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub activity_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlockPermanentReply {
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlockPermanentRequest {
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateActivityPush {
    #[prost(message, optional, tag = "1")]
    pub activity_info: ::core::option::Option<ActivityInfo>,
    #[prost(int32, optional, tag = "2")]
    pub time: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatperElementInfo {
    #[prost(int32, optional, tag = "1")]
    pub element: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub dialog_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Handbook {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub is_read: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHandbookInfoReply {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<Handbook>,
    #[prost(message, repeated, tag = "2")]
    pub element_info: ::prost::alloc::vec::Vec<ChatperElementInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHandbookInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HandbookReadReply {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HandbookReadRequest {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserChapterTypint32 {
    #[prost(int32, optional, tag = "1")]
    pub chapter_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub today_pass_num: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub today_total_num: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserDungeon {
    #[prost(int32, optional, tag = "1")]
    pub chapter_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub episode_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub star: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub challenge_count: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "5")]
    pub has_record: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "6")]
    pub left_return_all_num: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub today_pass_num: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub today_total_num: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserDungeonSpStatus {
    #[prost(int32, optional, tag = "1")]
    pub chapter_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub episode_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub status: ::core::option::Option<i32>,
    #[prost(uint64, optional, tag = "4")]
    pub refresh_time: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardEffectint32 {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EffectTypeint32 {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuffInfo {
    #[prost(int32, optional, tag = "1")]
    pub buff_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub duration: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "3")]
    pub uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "4")]
    pub ex_info: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "5")]
    pub from_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "6")]
    pub count: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "7")]
    pub act_common_params: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "8")]
    pub layer: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "9")]
    pub r#type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipRecord {
    #[prost(int64, optional, tag = "1")]
    pub equip_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub equip_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub equip_lv: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub refine_lv: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PowerInfo {
    #[prost(int32, optional, tag = "1")]
    pub power_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub num: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub max: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Activity104EquipRecord {
    #[prost(int64, optional, tag = "1")]
    pub equip_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub equip_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SummonedInfo {
    #[prost(int32, optional, tag = "1")]
    pub summoned_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub level: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "3")]
    pub uid: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "4")]
    pub from_uid: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnhanceInfoBox {
    #[prost(int64, optional, tag = "1")]
    pub uid: ::core::option::Option<i64>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub can_upgrade_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub upgraded_options: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightEntityInfo {
    #[prost(int64, optional, tag = "1")]
    pub uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub model_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub skin: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub position: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub entity_type: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "6")]
    pub user_id: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "7")]
    pub ex_point: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "9")]
    pub current_hp: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "10")]
    pub attr: ::core::option::Option<HeroAttribute>,
    #[prost(message, repeated, tag = "11")]
    pub buffs: ::prost::alloc::vec::Vec<BuffInfo>,
    #[prost(int32, repeated, packed = "false", tag = "12")]
    pub skill_group1: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "13")]
    pub skill_group2: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "14")]
    pub passive_skill: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "15")]
    pub ex_skill: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "16")]
    pub shield_value: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "17")]
    pub no_effect_buffs: ::prost::alloc::vec::Vec<BuffInfo>,
    #[prost(int32, optional, tag = "18")]
    pub expoint_max_add: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "19")]
    pub buff_harm_statistic: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "20")]
    pub equip_uid: ::core::option::Option<i64>,
    #[prost(message, optional, tag = "21")]
    pub trial_equip: ::core::option::Option<EquipRecord>,
    #[prost(int32, optional, tag = "22")]
    pub ex_skill_level: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "23")]
    pub power_infos: ::prost::alloc::vec::Vec<PowerInfo>,
    #[prost(int64, repeated, packed = "false", tag = "24")]
    pub act104_equip_uids: ::prost::alloc::vec::Vec<i64>,
    #[prost(message, repeated, tag = "25")]
    pub trial_act104_equips: ::prost::alloc::vec::Vec<Activity104EquipRecord>,
    #[prost(message, repeated, tag = "26")]
    pub summoned_list: ::prost::alloc::vec::Vec<SummonedInfo>,
    #[prost(message, optional, tag = "27")]
    pub base_attr: ::core::option::Option<HeroAttribute>,
    #[prost(int32, optional, tag = "28")]
    pub ex_skill_point_change: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "29")]
    pub team_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "30")]
    pub enhance_info_box: ::core::option::Option<EnhanceInfoBox>,
    #[prost(int32, optional, tag = "31")]
    pub trial_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "32")]
    pub career: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "33")]
    pub status: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "34")]
    pub guard: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "35")]
    pub sub_cd: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "36")]
    pub ex_point_type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MagicCircleInfo {
    #[prost(int32, optional, tag = "1")]
    pub magic_circle_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub round: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "3")]
    pub create_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "4")]
    pub electric_level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub electric_progress: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardEnchant {
    #[prost(int32, optional, tag = "1")]
    pub enchant_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub duration: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub ex_info: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardExtraInfo {
    #[prost(int32, optional, tag = "1")]
    pub key: ::core::option::Option<i32>,
    #[prost(string, repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardInfo {
    #[prost(int64, optional, tag = "1")]
    pub uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub skill_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub card_effect: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "4")]
    pub temp_card: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "5")]
    pub enchants: ::prost::alloc::vec::Vec<CardEnchant>,
    #[prost(int32, optional, tag = "6")]
    pub card_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub status: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "9")]
    pub target_uid: ::core::option::Option<i64>,
    #[prost(message, optional, tag = "10")]
    pub extra_info: ::core::option::Option<CardExtraInfo>,
    #[prost(int32, optional, tag = "11")]
    pub energy: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "12")]
    pub extra_infos: ::prost::alloc::vec::Vec<CardExtraInfo>,
    #[prost(int32, optional, tag = "13")]
    pub area_red_or_blue: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "14")]
    pub heat_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightStep {
    #[prost(int32, optional, tag = "1")]
    pub act_type: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "2")]
    pub from_id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "3")]
    pub to_id: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "4")]
    pub act_id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "5")]
    pub act_effect: ::prost::alloc::vec::Vec<ActEffect>,
    #[prost(int32, optional, tag = "6")]
    pub card_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub support_hero_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "8")]
    pub fake_timeline: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssistBossSkillInfo {
    #[prost(int32, optional, tag = "1")]
    pub skill_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub need_power: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub power_low: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub power_high: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssistBossInfo {
    #[prost(message, repeated, tag = "1")]
    pub skills: ::prost::alloc::vec::Vec<AssistBossSkillInfo>,
    #[prost(int32, optional, tag = "2")]
    pub curr_cd: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub cd_cfg: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub form_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub round_use_limit: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub exceed_use_free: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "7")]
    pub params: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmitterInfo {
    #[prost(int32, optional, tag = "1")]
    pub energy: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerFinisherSkillInfo {
    #[prost(int32, optional, tag = "1")]
    pub skill_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub need_power: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerFinisherInfo {
    #[prost(message, repeated, tag = "1")]
    pub skills: ::prost::alloc::vec::Vec<PlayerFinisherSkillInfo>,
    #[prost(int32, optional, tag = "2")]
    pub round_use_limit: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardHeatValue {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub upper_limit: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub lower_limit: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub value: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub change_value: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightTaskValue {
    #[prost(int32, optional, tag = "1")]
    pub index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub progress: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub max_progress: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "4")]
    pub finished: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightTask {
    #[prost(int32, optional, tag = "1")]
    pub task_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub status: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "3")]
    pub values: ::prost::alloc::vec::Vec<FightTaskValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerSkillInfo {
    #[prost(int32, optional, tag = "1")]
    pub skill_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub cd: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub need_power: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub r#type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndicatorInfo {
    #[prost(int32, optional, tag = "1")]
    pub inticator_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub num: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardHeatInfo {
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<CardHeatValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightBloodPool {
    #[prost(int32, optional, tag = "1")]
    pub value: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub max: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightTeam {
    #[prost(message, repeated, tag = "1")]
    pub entitys: ::prost::alloc::vec::Vec<FightEntityInfo>,
    #[prost(message, repeated, tag = "2")]
    pub sub_entitys: ::prost::alloc::vec::Vec<FightEntityInfo>,
    #[prost(int32, optional, tag = "3")]
    pub power: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub cloth_id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "5")]
    pub skill_infos: ::prost::alloc::vec::Vec<PlayerSkillInfo>,
    #[prost(message, repeated, tag = "6")]
    pub sp_entitys: ::prost::alloc::vec::Vec<FightEntityInfo>,
    #[prost(message, repeated, tag = "7")]
    pub indicators: ::prost::alloc::vec::Vec<IndicatorInfo>,
    #[prost(string, optional, tag = "8")]
    pub ex_team_str: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "9")]
    pub assist_boss: ::core::option::Option<FightEntityInfo>,
    #[prost(message, optional, tag = "10")]
    pub assist_boss_info: ::core::option::Option<AssistBossInfo>,
    #[prost(message, optional, tag = "11")]
    pub emitter: ::core::option::Option<FightEntityInfo>,
    #[prost(message, optional, tag = "12")]
    pub emitter_info: ::core::option::Option<EmitterInfo>,
    #[prost(message, optional, tag = "13")]
    pub player_entity: ::core::option::Option<FightEntityInfo>,
    #[prost(message, optional, tag = "14")]
    pub player_finisher_info: ::core::option::Option<PlayerFinisherInfo>,
    #[prost(int32, optional, tag = "15")]
    pub energy: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "16")]
    pub card_heat: ::core::option::Option<CardHeatInfo>,
    #[prost(int32, optional, tag = "17")]
    pub card_deck_size: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "18")]
    pub blood_pool: ::core::option::Option<FightBloodPool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightParam {
    #[prost(int32, optional, tag = "1")]
    pub key: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub value: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomData {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub data: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightTaskBox {
    #[prost(message, repeated, tag = "1")]
    pub tasks: ::prost::alloc::vec::Vec<FightTask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fight {
    #[prost(message, optional, tag = "1")]
    pub attacker: ::core::option::Option<FightTeam>,
    #[prost(message, optional, tag = "2")]
    pub defender: ::core::option::Option<FightTeam>,
    #[prost(int32, optional, tag = "3")]
    pub cur_round: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub max_round: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "5")]
    pub is_finish: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "6")]
    pub cur_wave: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub battle_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "8")]
    pub magic_circle: ::core::option::Option<MagicCircleInfo>,
    #[prost(int32, optional, tag = "9")]
    pub version: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "10")]
    pub is_record: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "11")]
    pub episode_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "12")]
    pub fight_act_type: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "13")]
    pub last_change_hero_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "14")]
    pub progress: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "15")]
    pub progress_max: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "16")]
    pub param: ::prost::alloc::vec::Vec<FightParam>,
    #[prost(message, repeated, tag = "17")]
    pub custom_data: ::prost::alloc::vec::Vec<CustomData>,
    #[prost(message, optional, tag = "18")]
    pub fight_task_box: ::core::option::Option<FightTaskBox>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActEffect {
    #[prost(int64, optional, tag = "1")]
    pub target_id: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub effect_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub effect_num: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "4")]
    pub buff: ::core::option::Option<BuffInfo>,
    #[prost(message, optional, tag = "5")]
    pub entity: ::core::option::Option<FightEntityInfo>,
    #[prost(int32, optional, tag = "6")]
    pub config_effect: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub buff_act_id: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "8")]
    pub reserve_id: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "9")]
    pub reserve_str: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "10")]
    pub summoned: ::core::option::Option<SummonedInfo>,
    #[prost(message, optional, tag = "11")]
    pub magic_circle: ::core::option::Option<MagicCircleInfo>,
    #[prost(message, optional, tag = "12")]
    pub card_info: ::core::option::Option<CardInfo>,
    #[prost(message, repeated, tag = "13")]
    pub card_info_list: ::prost::alloc::vec::Vec<CardInfo>,
    #[prost(int32, optional, tag = "14")]
    pub team_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "15")]
    pub fight_step: ::core::option::Option<FightStep>,
    #[prost(message, optional, tag = "16")]
    pub assist_boss_info: ::core::option::Option<AssistBossInfo>,
    #[prost(int32, optional, tag = "17")]
    pub effect_num1: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "18")]
    pub emitter_info: ::core::option::Option<EmitterInfo>,
    #[prost(message, optional, tag = "19")]
    pub player_finisher_info: ::core::option::Option<PlayerFinisherInfo>,
    #[prost(message, optional, tag = "20")]
    pub power_info: ::core::option::Option<PowerInfo>,
    #[prost(message, optional, tag = "21")]
    pub card_heat_value: ::core::option::Option<CardHeatValue>,
    #[prost(message, repeated, tag = "22")]
    pub fight_tasks: ::prost::alloc::vec::Vec<FightTask>,
    #[prost(message, optional, tag = "23")]
    pub fight: ::core::option::Option<Fight>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginRoundOper {
    #[prost(int32, optional, tag = "1")]
    pub oper_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub param1: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub param2: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "4")]
    pub to_id: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardDeckInfo {
    #[prost(int64, optional, tag = "1")]
    pub uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub skill_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub num: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightActivity104EquipRecord {
    #[prost(int64, optional, tag = "1")]
    pub hero_uid: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "2")]
    pub activity104_equip_records: ::prost::alloc::vec::Vec<Activity104EquipRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightEntityDetailInfo {
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<FightEntityInfo>,
    #[prost(message, optional, tag = "2")]
    pub ex_attr: ::core::option::Option<HeroExAttribute>,
    #[prost(message, optional, tag = "3")]
    pub sp_attr: ::core::option::Option<HeroSpAttribute>,
    #[prost(message, optional, tag = "4")]
    pub add_attr_per: ::core::option::Option<HeroAttribute>,
    #[prost(message, optional, tag = "5")]
    pub add_ex_attr: ::core::option::Option<HeroExAttribute>,
    #[prost(message, optional, tag = "6")]
    pub add_sp_attr: ::core::option::Option<HeroSpAttribute>,
    #[prost(message, optional, tag = "7")]
    pub test_add_attr_per: ::core::option::Option<HeroAttribute>,
    #[prost(message, optional, tag = "8")]
    pub test_add_ex_attr: ::core::option::Option<HeroExAttribute>,
    #[prost(message, optional, tag = "9")]
    pub test_add_sp_attr: ::core::option::Option<HeroSpAttribute>,
    #[prost(message, optional, tag = "10")]
    pub part_attr_base: ::core::option::Option<HeroAttribute>,
    #[prost(message, optional, tag = "11")]
    pub part_ex_attr: ::core::option::Option<HeroExAttribute>,
    #[prost(message, optional, tag = "12")]
    pub part_sp_attr: ::core::option::Option<HeroSpAttribute>,
    #[prost(message, optional, tag = "13")]
    pub test_part_attr_base: ::core::option::Option<HeroAttribute>,
    #[prost(message, optional, tag = "14")]
    pub test_part_ex_attr: ::core::option::Option<HeroExAttribute>,
    #[prost(message, optional, tag = "15")]
    pub test_part_sp_attr: ::core::option::Option<HeroSpAttribute>,
    #[prost(message, optional, tag = "16")]
    pub final_attr_base: ::core::option::Option<HeroAttribute>,
    #[prost(message, optional, tag = "17")]
    pub final_ex_attr: ::core::option::Option<HeroExAttribute>,
    #[prost(message, optional, tag = "18")]
    pub final_sp_attr: ::core::option::Option<HeroSpAttribute>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightEquip {
    #[prost(int64, optional, tag = "1")]
    pub hero_uid: ::core::option::Option<i64>,
    #[prost(int64, repeated, packed = "false", tag = "2")]
    pub equip_uid: ::prost::alloc::vec::Vec<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightEquipRecord {
    #[prost(int64, optional, tag = "1")]
    pub hero_uid: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "2")]
    pub equip_records: ::prost::alloc::vec::Vec<EquipRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightExPointInfo {
    #[prost(int64, optional, tag = "1")]
    pub uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub ex_point: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "3")]
    pub power_infos: ::prost::alloc::vec::Vec<PowerInfo>,
    #[prost(int32, optional, tag = "4")]
    pub current_hp: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub ex_point_type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrialHero {
    #[prost(int32, optional, tag = "1")]
    pub trial_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub pos: ::core::option::Option<i32>,
    #[prost(int64, repeated, packed = "false", tag = "3")]
    pub equip_uid: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, packed = "false", tag = "4")]
    pub act104_equip_uid: ::prost::alloc::vec::Vec<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightGroup {
    #[prost(int64, repeated, packed = "false", tag = "1")]
    pub hero_list: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, packed = "false", tag = "2")]
    pub sub_hero_list: ::prost::alloc::vec::Vec<i64>,
    #[prost(int32, optional, tag = "3")]
    pub cloth_id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "4")]
    pub equips: ::prost::alloc::vec::Vec<FightEquip>,
    #[prost(message, repeated, tag = "5")]
    pub trial_hero_list: ::prost::alloc::vec::Vec<TrialHero>,
    #[prost(message, repeated, tag = "6")]
    pub activity104_equips: ::prost::alloc::vec::Vec<FightEquip>,
    #[prost(int32, repeated, packed = "false", tag = "7")]
    pub ex_infos: ::prost::alloc::vec::Vec<i32>,
    #[prost(int64, optional, tag = "8")]
    pub assist_user_id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "9")]
    pub assist_hero_uid: ::core::option::Option<i64>,
    #[prost(string, repeated, tag = "10")]
    pub extra_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "11")]
    pub assist_boss_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightHeroRecord {
    #[prost(int64, optional, tag = "1")]
    pub hero_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub skin: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrialHeroRecord {
    #[prost(int32, optional, tag = "1")]
    pub trial_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub pos: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "3")]
    pub equip_records: ::prost::alloc::vec::Vec<EquipRecord>,
    #[prost(message, repeated, tag = "4")]
    pub activity104_equip_records: ::prost::alloc::vec::Vec<Activity104EquipRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightGroupRecord {
    #[prost(message, repeated, tag = "1")]
    pub hero_list: ::prost::alloc::vec::Vec<FightHeroRecord>,
    #[prost(message, repeated, tag = "2")]
    pub sub_hero_list: ::prost::alloc::vec::Vec<FightHeroRecord>,
    #[prost(int32, optional, tag = "3")]
    pub cloth_id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "4")]
    pub equips: ::prost::alloc::vec::Vec<FightEquipRecord>,
    #[prost(message, repeated, tag = "5")]
    pub trial_hero_list: ::prost::alloc::vec::Vec<TrialHeroRecord>,
    #[prost(message, repeated, tag = "6")]
    pub activity104_equips: ::prost::alloc::vec::Vec<FightActivity104EquipRecord>,
    #[prost(int32, repeated, packed = "false", tag = "7")]
    pub ex_infos: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "8")]
    pub version: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "9")]
    pub assist_user_id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "10")]
    pub assist_hero_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "11")]
    pub record_round: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "12")]
    pub assist_boss_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightHeroSpAttributeInfo {
    #[prost(int64, optional, tag = "1")]
    pub uid: ::core::option::Option<i64>,
    #[prost(message, optional, tag = "2")]
    pub attribute: ::core::option::Option<HeroSpAttribute>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightReason {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub content: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub battle_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub multiplication: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "5")]
    pub data: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseCardStatistics {
    #[prost(int32, optional, tag = "1")]
    pub skill_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub use_count: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightStatistics {
    #[prost(int64, optional, tag = "1")]
    pub hero_uid: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub harm: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "3")]
    pub hurt: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "4")]
    pub heal: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "5")]
    pub cards: ::prost::alloc::vec::Vec<UseCardStatistics>,
    #[prost(int32, repeated, packed = "false", tag = "6")]
    pub get_buffs: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightRecord {
    #[prost(int64, optional, tag = "1")]
    pub fight_id: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub fight_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "3")]
    pub fight_time: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "4")]
    pub fight_result: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "5")]
    pub attack_statistics: ::prost::alloc::vec::Vec<FightStatistics>,
    #[prost(message, repeated, tag = "6")]
    pub defense_statistics: ::prost::alloc::vec::Vec<FightStatistics>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightRound {
    #[prost(message, repeated, tag = "1")]
    pub fight_step: ::prost::alloc::vec::Vec<FightStep>,
    #[prost(int32, optional, tag = "2")]
    pub act_point: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub is_finish: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "4")]
    pub movint32: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "5")]
    pub ex_point_info: ::prost::alloc::vec::Vec<FightExPointInfo>,
    #[prost(message, repeated, tag = "6")]
    pub ai_use_cards: ::prost::alloc::vec::Vec<CardInfo>,
    #[prost(int32, optional, tag = "7")]
    pub power: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "8")]
    pub skill_infos: ::prost::alloc::vec::Vec<PlayerSkillInfo>,
    #[prost(message, repeated, tag = "9")]
    pub before_cards1: ::prost::alloc::vec::Vec<CardInfo>,
    #[prost(message, repeated, tag = "10")]
    pub team_a_cards1: ::prost::alloc::vec::Vec<CardInfo>,
    #[prost(message, repeated, tag = "11")]
    pub before_cards2: ::prost::alloc::vec::Vec<CardInfo>,
    #[prost(message, repeated, tag = "12")]
    pub team_a_cards2: ::prost::alloc::vec::Vec<CardInfo>,
    #[prost(message, repeated, tag = "13")]
    pub next_round_begin_step: ::prost::alloc::vec::Vec<FightStep>,
    #[prost(int32, repeated, packed = "false", tag = "14")]
    pub use_card_list: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "15")]
    pub cur_round: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "16")]
    pub hero_sp_attributes: ::prost::alloc::vec::Vec<FightHeroSpAttributeInfo>,
    #[prost(int64, optional, tag = "17")]
    pub last_change_hero_uid: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseClothSkillRound {
    #[prost(int32, optional, tag = "1")]
    pub skill_id: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "2")]
    pub from_id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "3")]
    pub to_id: ::core::option::Option<i64>,
    #[prost(message, optional, tag = "4")]
    pub round: ::core::option::Option<FightRound>,
    #[prost(int32, optional, tag = "5")]
    pub r#type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightRoundRecord {
    #[prost(message, repeated, tag = "1")]
    pub cloth_skills: ::prost::alloc::vec::Vec<UseClothSkillRound>,
    #[prost(message, repeated, tag = "2")]
    pub opers: ::prost::alloc::vec::Vec<BeginRoundOper>,
    #[prost(message, optional, tag = "3")]
    pub round: ::core::option::Option<FightRound>,
    #[prost(bool, optional, tag = "4")]
    pub new_wave: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveCardOper {
    #[prost(int32, optional, tag = "1")]
    pub from_position: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub to_position: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetInfo {
    #[prost(int32, optional, tag = "1")]
    pub card_index: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "2")]
    pub to_id: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct M2qEntry {
    #[prost(uint32, optional, tag = "1")]
    pub material_id: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2")]
    pub quantity: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaterialData {
    #[prost(uint32, optional, tag = "1")]
    pub materil_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub materil_id: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "3")]
    pub quantity: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaterialChangePush {
    #[prost(message, repeated, tag = "1")]
    pub data_list: ::prost::alloc::vec::Vec<MaterialData>,
    #[prost(uint32, optional, tag = "2")]
    pub get_approach: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssistHeroCareerNo {
    #[prost(int32, optional, tag = "1")]
    pub career: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub assist_hero_infos: ::prost::alloc::vec::Vec<AssistHeroInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChapterMapElementUpdatePush {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub elements: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChapterMapUpdatePush {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub map_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoverDungeonRecordReply {
    #[prost(bool, optional, tag = "1")]
    pub is_cover: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoverDungeonRecordRequest {
    #[prost(bool, optional, tag = "1")]
    pub is_cover: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DungeonBonusInfo {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub bonus: ::prost::alloc::vec::Vec<MaterialData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DungeonInfosPush {
    #[prost(message, repeated, tag = "1")]
    pub dungeon_infos: ::prost::alloc::vec::Vec<UserDungeon>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DungeonLastHeroGroup {
    #[prost(int32, optional, tag = "1")]
    pub chapter_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub hero_group_snapshot: ::core::option::Option<HeroGroupInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DungeonUpdatePush {
    #[prost(message, optional, tag = "1")]
    pub dungeon_info: ::core::option::Option<UserDungeon>,
    #[prost(message, repeated, tag = "2")]
    pub chapter_typint32s: ::prost::alloc::vec::Vec<UserChapterTypint32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndDungeonPush {
    #[prost(int32, optional, tag = "1")]
    pub chapter_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub episode_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub player_exp: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "4")]
    pub first_bonus: ::prost::alloc::vec::Vec<MaterialData>,
    #[prost(message, repeated, tag = "5")]
    pub normal_bonus: ::prost::alloc::vec::Vec<MaterialData>,
    #[prost(int32, optional, tag = "6")]
    pub star: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "7")]
    pub advenced_bonus: ::prost::alloc::vec::Vec<MaterialData>,
    #[prost(bool, optional, tag = "8")]
    pub update_dungeon_record: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "9")]
    pub can_update_dungeon_record: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "10")]
    pub old_record_round: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "11")]
    pub new_record_round: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "12")]
    pub first_pass: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "13")]
    pub addition_bonus: ::prost::alloc::vec::Vec<MaterialData>,
    #[prost(message, repeated, tag = "14")]
    pub time_first_bonus: ::prost::alloc::vec::Vec<MaterialData>,
    #[prost(string, optional, tag = "15")]
    pub extra_str: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "16")]
    pub drop_bonus: ::prost::alloc::vec::Vec<DungeonBonusInfo>,
    #[prost(int64, optional, tag = "17")]
    pub assist_user_id: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "18")]
    pub assist_nickname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "19")]
    pub total_round: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndDungeonReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndDungeonRequest {
    #[prost(bool, optional, tag = "1")]
    pub is_abort: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroRecommendInfo {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub hero_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub sub_hero_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "3")]
    pub cloth: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "4")]
    pub rate: ::core::option::Option<f32>,
    #[prost(int32, repeated, packed = "false", tag = "5")]
    pub levels: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "6")]
    pub assist_boss_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpisodeHeroRecommendInfo {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub infos: ::prost::alloc::vec::Vec<HeroRecommendInfo>,
    #[prost(float, optional, tag = "3")]
    pub rate: ::core::option::Option<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipSpDungeonUpdatePush {
    #[prost(bool, optional, tag = "1")]
    pub is_delete: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "2")]
    pub chapter_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardPointInfo {
    #[prost(int32, optional, tag = "1")]
    pub chapter_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub reward_point: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub has_get_point_reward_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDungeonReply {
    #[prost(message, repeated, tag = "1")]
    pub dungeon_info_list: ::prost::alloc::vec::Vec<UserDungeon>,
    #[prost(message, repeated, tag = "2")]
    pub last_hero_group: ::prost::alloc::vec::Vec<DungeonLastHeroGroup>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub map_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "4")]
    pub elements: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "5")]
    pub reward_point_info: ::prost::alloc::vec::Vec<RewardPointInfo>,
    #[prost(int32, repeated, packed = "false", tag = "6")]
    pub equip_sp_chapters: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "7")]
    pub chapter_typint32s: ::prost::alloc::vec::Vec<UserChapterTypint32>,
    #[prost(int32, repeated, packed = "false", tag = "8")]
    pub finish_elements: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "9")]
    pub finish_puzzles: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "10")]
    pub dungeon_info_size: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDungeonRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEpisodeHeroRecommendReply {
    #[prost(message, repeated, tag = "1")]
    pub racommends: ::prost::alloc::vec::Vec<EpisodeHeroRecommendInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEpisodeHeroRecommendRequest {
    #[prost(int32, optional, tag = "1")]
    pub episode_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMainDramaRewardReply {
    #[prost(message, repeated, tag = "1")]
    pub bonus: ::prost::alloc::vec::Vec<MaterialData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMainDramaRewardRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPointRewardReply {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPointRewardRequest {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPuzzleProgressReply {
    #[prost(int32, optional, tag = "1")]
    pub element_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub progress: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPuzzleProgressRequest {
    #[prost(int32, optional, tag = "1")]
    pub element_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstructionDungeonFinalRewardReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstructionDungeonFinalRewardRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstructionDungeonInfoPush {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub unlock_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub get_reward_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(bool, optional, tag = "3")]
    pub get_final_reward: ::core::option::Option<bool>,
    #[prost(int32, repeated, packed = "false", tag = "4")]
    pub open_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstructionDungeonInfoReply {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub unlock_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub get_reward_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(bool, optional, tag = "3")]
    pub get_final_reward: ::core::option::Option<bool>,
    #[prost(int32, repeated, packed = "false", tag = "4")]
    pub open_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstructionDungeonInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstructionDungeonOpenReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstructionDungeonOpenRequest {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub open_id: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstructionDungeonRewardReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstructionDungeonRewardRequest {
    #[prost(int32, optional, tag = "1")]
    pub topic_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MainDramaRewardInfo {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapElementReply {
    #[prost(int32, optional, tag = "1")]
    pub element_id: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub dialog_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapElementRequest {
    #[prost(int32, optional, tag = "1")]
    pub element_id: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub dialog_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PuzzleFinishReply {
    #[prost(int32, optional, tag = "1")]
    pub element_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PuzzleFinishRequest {
    #[prost(int32, optional, tag = "1")]
    pub element_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshAssistReply {
    #[prost(int32, optional, tag = "1")]
    pub assist_type: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub assist_hero_careers: ::prost::alloc::vec::Vec<AssistHeroCareerNo>,
    #[prost(string, optional, tag = "3")]
    pub ext: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshAssistRequest {
    #[prost(int32, optional, tag = "1")]
    pub assist_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub ext: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardPointUpdatePush {
    #[prost(int32, optional, tag = "1")]
    pub chapter_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub value: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SavePuzzleProgressReply {
    #[prost(int32, optional, tag = "1")]
    pub element_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SavePuzzleProgressRequest {
    #[prost(int32, optional, tag = "1")]
    pub element_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub progress: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartDungeonReply {
    #[prost(message, optional, tag = "1")]
    pub fight: ::core::option::Option<Fight>,
    #[prost(message, optional, tag = "2")]
    pub round: ::core::option::Option<FightRound>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartDungeonRequest {
    #[prost(int32, optional, tag = "1")]
    pub chapter_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub episode_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub fight_group: ::core::option::Option<FightGroup>,
    #[prost(int32, optional, tag = "4")]
    pub multiplication: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "5")]
    pub use_record: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub is_restart: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "7")]
    pub is_balance: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "8")]
    pub params: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTaskPush {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub task_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishAllTaskReply {
    #[prost(int32, optional, tag = "1")]
    pub type_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub min_type_id: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub task_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "4")]
    pub activity_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishAllTaskRequest {
    #[prost(int32, optional, tag = "1")]
    pub type_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub min_type_id: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub task_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "4")]
    pub activity_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishReadTaskReply {
    #[prost(int32, optional, tag = "1")]
    pub task_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishReadTaskRequest {
    #[prost(int32, optional, tag = "1")]
    pub task_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishTaskReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub finish_count: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishTaskRequest {
    #[prost(int32, required, tag = "1")]
    pub id: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTaskActivityBonusReply {
    #[prost(int32, optional, tag = "1")]
    pub type_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub define_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTaskActivityBonusRequest {
    #[prost(int32, optional, tag = "1")]
    pub type_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub define_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    #[prost(int32, required, tag = "1")]
    pub id: i32,
    #[prost(int32, required, tag = "2")]
    pub progress: i32,
    #[prost(bool, required, tag = "3")]
    pub has_finished: bool,
    #[prost(int32, optional, tag = "4")]
    pub finish_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub expiry_time: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskActivityInfo {
    #[prost(int32, required, tag = "1")]
    pub type_id: i32,
    #[prost(int32, required, tag = "2")]
    pub define_id: i32,
    #[prost(int32, required, tag = "3")]
    pub value: i32,
    #[prost(int32, optional, tag = "4")]
    pub gain_value: ::core::option::Option<i32>,
    #[prost(int32, required, tag = "5")]
    pub expiry_time: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTaskInfoReply {
    #[prost(message, repeated, tag = "1")]
    pub task_info: ::prost::alloc::vec::Vec<Task>,
    #[prost(message, repeated, tag = "2")]
    pub activity_info: ::prost::alloc::vec::Vec<TaskActivityInfo>,
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub type_ids: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTaskInfoRequest {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub type_ids: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTaskPush {
    #[prost(message, repeated, tag = "1")]
    pub task_info: ::prost::alloc::vec::Vec<Task>,
    #[prost(message, repeated, tag = "2")]
    pub activity_info: ::prost::alloc::vec::Vec<TaskActivityInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BpBuyLevelReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub score: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BpBuyLevelRequset {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub num: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BpMarkFirstShowReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub is_sp: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BpMarkFirstShowRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub is_sp: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BpOpenPush {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BpPayPush {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub pay_status: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BpScoreBonusInfo {
    #[prost(int32, optional, tag = "1")]
    pub level: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub has_getfree_bonus: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub has_get_pay_bonus: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub has_get_spfree_bonus: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "5")]
    pub has_get_sp_pay_bonus: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BpScoreUpdatePush {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub score: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub weekly_score: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BpSelfSelectBonus {
    #[prost(int32, optional, tag = "1")]
    pub level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub index: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBpBonusReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub score_bonus_info: ::prost::alloc::vec::Vec<BpScoreBonusInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBpBonusRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub level: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub pay_bonus: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub is_sp: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBpInfoReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub score: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub pay_status: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub start_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub end_time: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "6")]
    pub task_info: ::prost::alloc::vec::Vec<Task>,
    #[prost(message, repeated, tag = "7")]
    pub score_bonus_info: ::prost::alloc::vec::Vec<BpScoreBonusInfo>,
    #[prost(int32, optional, tag = "8")]
    pub weekly_score: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "9")]
    pub first_show: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "10")]
    pub has_get_self_select_bonus: ::prost::alloc::vec::Vec<BpSelfSelectBonus>,
    #[prost(bool, optional, tag = "11")]
    pub sp_first_show: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBpInfoRequest {
    #[prost(bool, optional, tag = "1")]
    pub get_task: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSelfSelectBonusReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub index: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSelfSelectBonusRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub index: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyDoubleBonusReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub has_get_double_task_bonus: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "3")]
    pub double_bonus: ::prost::alloc::vec::Vec<MaterialData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyDoubleBonusRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropInfo {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub total_num: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub current_num: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TurnbackSignInInfo {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub state: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TurnbackInfo {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub tasks: ::prost::alloc::vec::Vec<Task>,
    #[prost(int32, optional, tag = "3")]
    pub bonus_point: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "4")]
    pub first_show: ::core::option::Option<bool>,
    #[prost(int32, repeated, packed = "false", tag = "5")]
    pub has_get_task_bonus: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "6")]
    pub sign_in_day: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "7")]
    pub sign_in_infos: ::prost::alloc::vec::Vec<TurnbackSignInInfo>,
    #[prost(bool, optional, tag = "8")]
    pub once_bonus: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "9")]
    pub end_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "10")]
    pub start_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "11")]
    pub remain_addition_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "12")]
    pub leave_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "13")]
    pub month_card_added_buy_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "14")]
    pub version: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "15")]
    pub buy_double_bonus: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "16")]
    pub drop_infos: ::prost::alloc::vec::Vec<DropInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTurnbackInfoReply {
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<TurnbackInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTurnbackInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshOnlineTaskReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshOnlineTaskRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TurnbackAdditionPush {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub remain_addition_count: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TurnbackBonusPointReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub bonus_point_id: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub has_get_task_bonus: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TurnbackBonusPointRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub bonus_point_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TurnbackFirstShowReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TurnbackFirstShowRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TurnbackOnceBonusReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TurnbackOnceBonusRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TurnbackSignInReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub day: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TurnbackSignInRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub day: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SettingInfo {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub param: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSettingInfosReply {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<SettingInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSettingInfosRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSettingInfoReply {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub param: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSettingInfoRequest {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub param: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Act160MissionInfo {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub state: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Act160FinishMissionReply {
    #[prost(int32, optional, tag = "1")]
    pub activity_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub act160_info: ::core::option::Option<Act160MissionInfo>,
    #[prost(bool, optional, tag = "3")]
    pub is_read_mail: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Act160FinishMissionRequest {
    #[prost(int32, optional, tag = "1")]
    pub activity_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Act160GetInfoReply {
    #[prost(int32, optional, tag = "1")]
    pub activity_id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub act160_infos: ::prost::alloc::vec::Vec<Act160MissionInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Act160GetInfoRequest {
    #[prost(int32, optional, tag = "1")]
    pub activity_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Act160UpdatePush {
    #[prost(int32, optional, tag = "1")]
    pub activity_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub act160_info: ::core::option::Option<Act160MissionInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Act172Info {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub use_item_task_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Act172UseItemTaskIdsUpdatePush {
    #[prost(int32, optional, tag = "1")]
    pub activity_id: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub use_item_task_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAct172InfoReply {
    #[prost(int32, optional, tag = "1")]
    pub activity_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub info: ::core::option::Option<Act172Info>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAct172InfoRequest {
    #[prost(int32, optional, tag = "1")]
    pub activity_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceptAct186SpBonusReply {
    #[prost(int32, optional, tag = "1")]
    pub activity_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub act186_activity_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceptAct186SpBonusRequest {
    #[prost(int32, optional, tag = "1")]
    pub activity_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub act186_activity_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Act101Info {
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub state: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Act101SpInfo {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub state: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Get101BonusReply {
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2")]
    pub activity_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Get101BonusRequest {
    #[prost(int32, optional, tag = "1")]
    pub activity_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub id: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Get101InfosReply {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<Act101Info>,
    #[prost(uint32, optional, tag = "2")]
    pub login_count: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "3")]
    pub activity_id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "4")]
    pub sp_infos: ::prost::alloc::vec::Vec<Act101SpInfo>,
    #[prost(bool, optional, tag = "5")]
    pub got_once_bonus: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Get101InfosRequest {
    #[prost(int32, optional, tag = "1")]
    pub activity_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Get101OnceBonusReply {
    #[prost(int32, optional, tag = "1")]
    pub activity_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Get101OnceBonusRequest {
    #[prost(int32, optional, tag = "1")]
    pub activity_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Get101SpBonusReply {
    #[prost(int32, optional, tag = "1")]
    pub activity_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Get101SpBonusRequest {
    #[prost(int32, optional, tag = "1")]
    pub activity_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAct186SpBonusInfoReply {
    #[prost(int32, optional, tag = "1")]
    pub activity_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub act186_activity_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub sp_bonus_stage: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAct186SpBonusInfoRequest {
    #[prost(int32, optional, tag = "1")]
    pub activity_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub act186_activity_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinyLevelUpReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub level: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinyLevelUpRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub level: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinyRankUpReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinyRankUpRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinyStoneUnlockReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub stone_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinyStoneUnlockRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub stone_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinyStoneUseReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub stone_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinyStoneUseRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub stone_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroBirthdayInfo {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub birthday_count: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroDefaultEquipReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "2")]
    pub default_equip_uid: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroDefaultEquipRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "2")]
    pub default_equip_uid: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroGainPush {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub is_first: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "3")]
    pub duplicate_count: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroInfoListReply {
    #[prost(message, repeated, tag = "1")]
    pub heros: ::prost::alloc::vec::Vec<HeroInfo>,
    #[prost(int32, optional, tag = "2")]
    pub touch_count_left: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub all_hero_skin: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "4")]
    pub birthday_infos: ::prost::alloc::vec::Vec<HeroBirthdayInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroInfoListRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroLevelUpReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub new_level: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroLevelUpRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub expect_level: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroLevelUpUpdatePush {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub new_level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub new_rank: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroRankUpReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub new_rank: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroRankUpRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroRedDotReadReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub red_dot: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroRedDotReadRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub red_dot_type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroSkinGainPush {
    #[prost(int32, optional, tag = "1")]
    pub skin_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub first_gain: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "3")]
    pub get_approach: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroTalentStylePercent {
    #[prost(int32, optional, tag = "1")]
    pub style: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub percent: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroTalentStyleStatReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub style_percent_list: ::prost::alloc::vec::Vec<HeroTalentStylePercent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroTalentStyleStatRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroTalentUpReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub talent_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroTalentUpRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroTouchReply {
    #[prost(int32, optional, tag = "1")]
    pub touch_count_left: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub success: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroTouchRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroUpdatePush {
    #[prost(message, repeated, tag = "1")]
    pub hero_updates: ::prost::alloc::vec::Vec<HeroInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroUpgradeSkillReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroUpgradeSkillRequest {
    #[prost(int32, required, tag = "1")]
    pub hero_id: i32,
    #[prost(int32, required, tag = "2")]
    pub r#type: i32,
    #[prost(int32, optional, tag = "3")]
    pub consume: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemUnlockReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub item_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemUnlockRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub item_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkHeroFavorReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub is_favor: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkHeroFavorRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub is_favor: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutTalentCubeBatchReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub template_info: ::core::option::Option<TalentTemplateInfo>,
    #[prost(int32, optional, tag = "3")]
    pub style: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutTalentCubeBatchRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub put_cube_info: ::prost::alloc::vec::Vec<TalentCubeInfo>,
    #[prost(int32, optional, tag = "3")]
    pub template_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub style: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutTalentCubeReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub template_info: ::core::option::Option<TalentTemplateInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutTalentCubeRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub get_cube_info: ::core::option::Option<TalentCubeInfo>,
    #[prost(message, optional, tag = "3")]
    pub put_cube_info: ::core::option::Option<TalentCubeInfo>,
    #[prost(int32, optional, tag = "4")]
    pub template_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutTalentSchemeReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub template_info: ::core::option::Option<TalentTemplateInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutTalentSchemeRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub talent_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub talent_mould: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub star_mould: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub template_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameTalentTemplateReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub template_info: ::core::option::Option<TalentTemplateInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameTalentTemplateRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub template_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakeoffAllTalentCubeReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub template_info: ::core::option::Option<TalentTemplateInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakeoffAllTalentCubeRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub template_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TalentStyleReadReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TalentStyleReadRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnMarkIsNewReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnMarkIsNewRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlockTalentStyleReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub style: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlockTalentStyleRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub style: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlockVoiceReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub voice_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlockVoiceRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub voice_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseSkinReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub skin_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseSkinRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub skin_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseTalentStyleReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub template_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub style: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseTalentStyleRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub template_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub style: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseTalentTemplateReply {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub template_info: ::core::option::Option<TalentTemplateInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseTalentTemplateRequest {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub template_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeHeroGroupSelectReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub current_select: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeHeroGroupSelectRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub current_select: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroGourpType {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub current_select: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub group_info: ::core::option::Option<HeroGroupInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHeroGroupCommonListReply {
    #[prost(message, repeated, tag = "1")]
    pub hero_group_commons: ::prost::alloc::vec::Vec<HeroGroupInfo>,
    #[prost(message, repeated, tag = "2")]
    pub hero_gourp_types: ::prost::alloc::vec::Vec<HeroGourpType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHeroGroupCommonListRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHeroGroupListReply {
    #[prost(message, repeated, tag = "1")]
    pub group_info_list: ::prost::alloc::vec::Vec<HeroGroupInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHeroGroupListRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeroGroupSnapshotNo {
    #[prost(int32, optional, tag = "1")]
    pub snapshot_id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub hero_group_snapshots: ::prost::alloc::vec::Vec<HeroGroupInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHeroGroupSnapshotListReply {
    #[prost(message, repeated, tag = "1")]
    pub hero_group_snapshots: ::prost::alloc::vec::Vec<HeroGroupSnapshotNo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHeroGroupSnapshotListRequest {
    #[prost(int32, optional, tag = "1")]
    pub snapshot_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetHeroGroupEquipReply {
    #[prost(int32, optional, tag = "1")]
    pub group_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub equip: ::core::option::Option<HeroGroupEquip>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetHeroGroupEquipRequest {
    #[prost(int32, optional, tag = "1")]
    pub group_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub equip: ::core::option::Option<HeroGroupEquip>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetHeroGroupSnapshotReply {
    #[prost(int32, optional, tag = "1")]
    pub snapshot_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub snapshot_sub_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub fight_group: ::core::option::Option<FightGroup>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetHeroGroupSnapshotRequest {
    #[prost(int32, optional, tag = "1")]
    pub snapshot_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub snapshot_sub_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub fight_group: ::core::option::Option<FightGroup>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateHeroGroupNameReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub current_select: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateHeroGroupNameRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub current_select: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateHeroGroupPush {
    #[prost(message, optional, tag = "1")]
    pub group_info: ::core::option::Option<HeroGroupInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateHeroGroupReply {
    #[prost(message, optional, tag = "1")]
    pub group_info: ::core::option::Option<HeroGroupInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateHeroGroupRequest {
    #[prost(message, required, tag = "1")]
    pub group_info: HeroGroupInfo,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoUseExpirePowerItemReply {
    #[prost(bool, optional, tag = "1")]
    pub used: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoUseExpirePowerItemRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Item {
    #[prost(uint32, optional, tag = "1")]
    pub item_id: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2")]
    pub quantity: ::core::option::Option<i32>,
    #[prost(uint64, optional, tag = "3")]
    pub last_use_time: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "4")]
    pub last_update_time: ::core::option::Option<u64>,
    #[prost(int64, optional, tag = "5")]
    pub total_gain_count: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PowerItem {
    #[prost(int64, optional, tag = "1")]
    pub uid: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub item_id: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "3")]
    pub quantity: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub expire_time: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsightItem {
    #[prost(int64, optional, tag = "1")]
    pub uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub item_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub quantity: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub expire_time: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetItemListReply {
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<Item>,
    #[prost(message, repeated, tag = "2")]
    pub power_items: ::prost::alloc::vec::Vec<PowerItem>,
    #[prost(message, repeated, tag = "3")]
    pub insight_items: ::prost::alloc::vec::Vec<InsightItem>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetItemListRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemChangePush {
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<Item>,
    #[prost(message, repeated, tag = "2")]
    pub power_items: ::prost::alloc::vec::Vec<PowerItem>,
    #[prost(message, repeated, tag = "3")]
    pub insight_items: ::prost::alloc::vec::Vec<InsightItem>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkReadSubType21Reply {
    #[prost(int32, optional, tag = "1")]
    pub item_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkReadSubType21Request {
    #[prost(int32, optional, tag = "1")]
    pub item_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseInsightItemReply {
    #[prost(int64, optional, tag = "1")]
    pub uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub hero_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseInsightItemRequest {
    #[prost(int64, optional, tag = "1")]
    pub uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub hero_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseItemReply {
    #[prost(message, repeated, tag = "1")]
    pub entry: ::prost::alloc::vec::Vec<M2qEntry>,
    #[prost(uint64, optional, tag = "2")]
    pub target_id: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseItemRequest {
    #[prost(message, repeated, tag = "1")]
    pub entry: ::prost::alloc::vec::Vec<M2qEntry>,
    #[prost(uint64, optional, tag = "2")]
    pub target_id: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsePowerItemInfo {
    #[prost(int64, optional, tag = "1")]
    pub uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub num: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsePowerItemListReply {
    #[prost(message, repeated, tag = "1")]
    pub use_power_item_info: ::prost::alloc::vec::Vec<UsePowerItemInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsePowerItemListRequest {
    #[prost(message, repeated, tag = "1")]
    pub use_power_item_info: ::prost::alloc::vec::Vec<UsePowerItemInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsePowerItemReply {
    #[prost(int64, optional, tag = "1")]
    pub uid: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsePowerItemRequest {
    #[prost(int64, optional, tag = "1")]
    pub uid: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoRoundReply {
    #[prost(message, repeated, tag = "1")]
    pub opers: ::prost::alloc::vec::Vec<BeginRoundOper>,
    #[prost(int64, optional, tag = "2")]
    pub to_id: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoRoundRequest {
    #[prost(message, repeated, tag = "1")]
    pub opers: ::prost::alloc::vec::Vec<BeginRoundOper>,
    #[prost(int64, optional, tag = "2")]
    pub to_id: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginFightReply {
    #[prost(message, optional, tag = "1")]
    pub fight: ::core::option::Option<Fight>,
    #[prost(message, optional, tag = "2")]
    pub round: ::core::option::Option<FightRound>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginFightRequest {
    #[prost(message, optional, tag = "1")]
    pub fight_group: ::core::option::Option<FightGroup>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginRoundReply {
    #[prost(message, optional, tag = "1")]
    pub round: ::core::option::Option<FightRound>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginRoundRequest {
    #[prost(message, repeated, tag = "1")]
    pub opers: ::prost::alloc::vec::Vec<BeginRoundOper>,
    #[prost(bool, optional, tag = "2")]
    pub auto_oper: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardInfoPush {
    #[prost(message, repeated, tag = "1")]
    pub card_group: ::prost::alloc::vec::Vec<CardInfo>,
    #[prost(int32, optional, tag = "2")]
    pub act_point: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub movint32: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "4")]
    pub before_cards: ::prost::alloc::vec::Vec<CardInfo>,
    #[prost(message, repeated, tag = "5")]
    pub deal_card_group: ::prost::alloc::vec::Vec<CardInfo>,
    #[prost(int32, optional, tag = "6")]
    pub extra_move_act: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "7")]
    pub is_gm: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeSubHeroExSkillReply {
    #[prost(message, optional, tag = "1")]
    pub round: ::core::option::Option<FightRound>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeSubHeroExSkillRequest {
    #[prost(int64, optional, tag = "1")]
    pub ex_skill_target: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeSubHeroReply {
    #[prost(message, optional, tag = "1")]
    pub round: ::core::option::Option<FightRound>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeSubHeroRequest {
    #[prost(int64, optional, tag = "1")]
    pub sub_hero_id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub change_hero_id: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndFightPush {
    #[prost(message, optional, tag = "1")]
    pub record: ::core::option::Option<FightRecord>,
    #[prost(message, optional, tag = "2")]
    pub fight_group_a: ::core::option::Option<FightGroup>,
    #[prost(bool, optional, tag = "3")]
    pub is_record: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndFightReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndFightRequest {
    #[prost(bool, optional, tag = "1")]
    pub is_abort: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndRoundReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndRoundRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityInfoReply {
    #[prost(message, optional, tag = "1")]
    pub entity_info: ::core::option::Option<FightEntityInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityInfoRequest {
    #[prost(int64, optional, tag = "1")]
    pub uid: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseClothSkillOperRecord {
    #[prost(int32, optional, tag = "1")]
    pub skill_id: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "2")]
    pub from_id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "3")]
    pub to_id: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "4")]
    pub r#type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightRoundOperRecord {
    #[prost(message, repeated, tag = "1")]
    pub cloth_skill_opers: ::prost::alloc::vec::Vec<UseClothSkillOperRecord>,
    #[prost(message, repeated, tag = "2")]
    pub opers: ::prost::alloc::vec::Vec<BeginRoundOper>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedealCardInfoPush {
    #[prost(message, repeated, tag = "1")]
    pub card_group: ::prost::alloc::vec::Vec<CardInfo>,
    #[prost(message, repeated, tag = "2")]
    pub deal_card_group: ::prost::alloc::vec::Vec<CardInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightRoundRecordAll {
    #[prost(message, optional, tag = "1")]
    pub fight: ::core::option::Option<Fight>,
    #[prost(message, optional, tag = "2")]
    pub round: ::core::option::Option<FightRound>,
    #[prost(message, repeated, tag = "3")]
    pub records: ::prost::alloc::vec::Vec<FightRoundRecord>,
    #[prost(message, optional, tag = "4")]
    pub fight_record: ::core::option::Option<FightRecord>,
    #[prost(int32, optional, tag = "5")]
    pub result: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub result_cause: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub total_round: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub kill_total: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "9")]
    pub push_info: ::core::option::Option<CardInfoPush>,
    #[prost(message, repeated, tag = "10")]
    pub wave_push_fight: ::prost::alloc::vec::Vec<Fight>,
    #[prost(message, repeated, tag = "12")]
    pub redeal_infos: ::prost::alloc::vec::Vec<RedealCardInfoPush>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightWavePush {
    #[prost(message, optional, tag = "1")]
    pub fight: ::core::option::Option<Fight>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightWithRecordAllReply {
    #[prost(message, optional, tag = "1")]
    pub fight: ::core::option::Option<Fight>,
    #[prost(message, optional, tag = "2")]
    pub round: ::core::option::Option<FightRound>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightWithRecordAllRequest {
    #[prost(message, optional, tag = "1")]
    pub record_all: ::core::option::Option<FightRoundRecordAll>,
    #[prost(message, optional, tag = "2")]
    pub group: ::core::option::Option<FightGroupRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntityDetailInfosReply {
    #[prost(message, repeated, tag = "1")]
    pub team_a_infos: ::prost::alloc::vec::Vec<FightEntityDetailInfo>,
    #[prost(message, repeated, tag = "2")]
    pub team_b_infos: ::prost::alloc::vec::Vec<FightEntityDetailInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntityDetailInfosRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFightCardDeckDetailInfoReply {
    #[prost(message, repeated, tag = "1")]
    pub deck_infos: ::prost::alloc::vec::Vec<CardInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFightCardDeckDetailInfoRequest {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFightCardDeckInfoReply {
    #[prost(message, repeated, tag = "1")]
    pub deck_infos: ::prost::alloc::vec::Vec<CardInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFightCardDeckInfoRequest {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFightOperReply {
    #[prost(message, repeated, tag = "1")]
    pub oper_records: ::prost::alloc::vec::Vec<FightRoundOperRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFightOperRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFightRecordAllReply {
    #[prost(message, optional, tag = "1")]
    pub record_all: ::core::option::Option<FightRoundRecordAll>,
    #[prost(message, optional, tag = "2")]
    pub group: ::core::option::Option<FightGroupRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFightRecordAllRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFightRecordGroupReply {
    #[prost(message, optional, tag = "1")]
    pub fight_group: ::core::option::Option<FightGroupRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFightRecordGroupRequest {
    #[prost(int32, optional, tag = "1")]
    pub episode_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveCardReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveCardRequest {
    #[prost(int32, optional, tag = "1")]
    pub from_position: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub to_position: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReconnectFightReply {
    #[prost(message, optional, tag = "1")]
    pub fight: ::core::option::Option<Fight>,
    #[prost(message, optional, tag = "2")]
    pub last_round: ::core::option::Option<FightRound>,
    #[prost(message, optional, tag = "3")]
    pub fight_reason: ::core::option::Option<FightReason>,
    #[prost(message, optional, tag = "4")]
    pub fight_group: ::core::option::Option<FightGroup>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReconnectFightRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetRoundReply {
    #[prost(message, optional, tag = "1")]
    pub fight: ::core::option::Option<Fight>,
    #[prost(message, optional, tag = "2")]
    pub round: ::core::option::Option<FightRound>,
    #[prost(message, repeated, tag = "3")]
    pub cards: ::prost::alloc::vec::Vec<CardInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetRoundRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TeamInfoPush {
    #[prost(message, optional, tag = "1")]
    pub fight: ::core::option::Option<Fight>,
    #[prost(bool, optional, tag = "2")]
    pub is_gm: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestFightIdReply {
    #[prost(message, optional, tag = "1")]
    pub fight: ::core::option::Option<Fight>,
    #[prost(message, optional, tag = "2")]
    pub round: ::core::option::Option<FightRound>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestFightIdRequest {
    #[prost(int32, optional, tag = "1")]
    pub fight_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub fight_group: ::core::option::Option<FightGroup>,
    #[prost(int32, optional, tag = "3")]
    pub fight_act_type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestFightReply {
    #[prost(message, optional, tag = "1")]
    pub fight: ::core::option::Option<Fight>,
    #[prost(message, optional, tag = "2")]
    pub round: ::core::option::Option<FightRound>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestFightRequest {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub group_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, optional, tag = "2")]
    pub fight_group: ::core::option::Option<FightGroup>,
    #[prost(int32, optional, tag = "3")]
    pub fight_act_type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseClothSkillReply {
    #[prost(message, optional, tag = "1")]
    pub round: ::core::option::Option<FightRound>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseClothSkillRequest {
    #[prost(int32, optional, tag = "1")]
    pub skill_id: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "2")]
    pub from_id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "3")]
    pub to_id: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "4")]
    pub r#type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EatEquip {
    #[prost(int64, optional, tag = "1")]
    pub eat_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub count: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Equip {
    #[prost(int32, optional, tag = "1")]
    pub equip_id: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "2")]
    pub uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "3")]
    pub level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub exp: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub break_lv: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub count: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "8")]
    pub is_lock: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "9")]
    pub refine_lv: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipBreakReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipBreakRequest {
    #[prost(int64, optional, tag = "1")]
    pub target_uid: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipComposeReply {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub equip_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipComposeRequest {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub equip_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipDecomposeReply {
    #[prost(int64, repeated, packed = "false", tag = "1")]
    pub equip_uids: ::prost::alloc::vec::Vec<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipDecomposeRequest {
    #[prost(int64, repeated, packed = "false", tag = "1")]
    pub equip_uids: ::prost::alloc::vec::Vec<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipDeletePush {
    #[prost(int64, repeated, packed = "false", tag = "1")]
    pub uids: ::prost::alloc::vec::Vec<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipLockReply {
    #[prost(int64, optional, tag = "1")]
    pub target_uid: ::core::option::Option<i64>,
    #[prost(bool, optional, tag = "2")]
    pub lock: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipLockRequest {
    #[prost(int64, optional, tag = "1")]
    pub target_uid: ::core::option::Option<i64>,
    #[prost(bool, optional, tag = "2")]
    pub lock: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipRefineReply {
    #[prost(int64, optional, tag = "1")]
    pub target_uid: ::core::option::Option<i64>,
    #[prost(int64, repeated, packed = "false", tag = "2")]
    pub eat_uids: ::prost::alloc::vec::Vec<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipRefineRequest {
    #[prost(int64, optional, tag = "1")]
    pub target_uid: ::core::option::Option<i64>,
    #[prost(int64, repeated, packed = "false", tag = "2")]
    pub eat_uids: ::prost::alloc::vec::Vec<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipStrengthenReply {
    #[prost(int64, optional, tag = "1")]
    pub target_uid: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "2")]
    pub eat_equips: ::prost::alloc::vec::Vec<EatEquip>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipStrengthenRequest {
    #[prost(int64, optional, tag = "1")]
    pub target_uid: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "2")]
    pub eat_equips: ::prost::alloc::vec::Vec<EatEquip>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipUpdatePush {
    #[prost(message, repeated, tag = "1")]
    pub equips: ::prost::alloc::vec::Vec<Equip>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEquipInfoReply {
    #[prost(message, repeated, tag = "1")]
    pub equips: ::prost::alloc::vec::Vec<Equip>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEquipInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoryFinishReply {
    #[prost(bool, optional, tag = "1")]
    pub is_finish: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoryFinishRequest {
    #[prost(int32, optional, tag = "1")]
    pub story_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessingStoryInfo {
    #[prost(int32, optional, tag = "1")]
    pub story_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub step_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub favor: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoryReply {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub finish_list: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "2")]
    pub processing_list: ::prost::alloc::vec::Vec<ProcessingStoryInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoryRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoryFinishPush {
    #[prost(int32, optional, tag = "1")]
    pub story_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateStoryReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateStoryRequest {
    #[prost(int32, optional, tag = "1")]
    pub story_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub step_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub favor: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChargeInfo {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub buy_count: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub first_charge: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChargeInfoReply {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<ChargeInfo>,
    #[prost(bool, optional, tag = "2")]
    pub sandbox_enable: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "3")]
    pub sandbox_balance: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChargeInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMonthCardBonusReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMonthCardBonusRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonthCardInfo {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub expire_time: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub has_get_bonus: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMonthCardInfoReply {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<MonthCardInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMonthCardInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewOrderReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub pass_back_param: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub notify_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "4")]
    pub game_order_id: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "5")]
    pub timestamp: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "6")]
    pub sign: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "7")]
    pub server_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "8")]
    pub currency: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelectionInfo {
    #[prost(int32, optional, tag = "1")]
    pub region_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub selection_pos: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewOrderRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub origin_currency: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub origin_amount: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "4")]
    pub selection_infos: ::prost::alloc::vec::Vec<SelectionInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderCompletePush {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "2")]
    pub game_order_id: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadChargeNewReply {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub goods_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadChargeNewRequest {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub goods_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SandboxChargeReply {
    #[prost(int64, optional, tag = "1")]
    pub game_order_id: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub sandbox_balance: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SandboxChargeRequset {
    #[prost(int64, optional, tag = "1")]
    pub game_order_id: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccelerateGuidePlanReply {
    #[prost(int32, optional, tag = "1")]
    pub guide_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub step: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccelerateGuidePlanRequest {
    #[prost(int32, optional, tag = "1")]
    pub guide_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub step: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllotCritterReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "2")]
    pub critter_uid: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllotCritterRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "2")]
    pub critter_uid: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllotVehicleReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "2")]
    pub building_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "3")]
    pub skin_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub building_define_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllotVehicleRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "2")]
    pub building_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "3")]
    pub skin_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchAddProctionsReply {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FreeChooseInfo {
    #[prost(int32, optional, tag = "1")]
    pub building_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub building_define_id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "3")]
    pub item2_count: ::prost::alloc::vec::Vec<M2qEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchAddProctionsRequest {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub free_info: ::core::option::Option<FreeChooseInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDispatchInfo {
    #[prost(int64, optional, tag = "1")]
    pub building_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub road_id: ::core::option::Option<i32>,
    #[prost(int64, repeated, packed = "false", tag = "3")]
    pub critter_uids: ::prost::alloc::vec::Vec<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDispatchCrittersReply {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub infos: ::prost::alloc::vec::Vec<BatchDispatchInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDispatchCrittersRequest {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockInfo {
    #[prost(int32, optional, tag = "1")]
    pub block_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub x: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub y: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub rotate: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub water_type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockPackageInfo {
    #[prost(int32, optional, tag = "1")]
    pub block_package_id: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub un_use_block_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub use_block_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockPackageGainPush {
    #[prost(message, repeated, tag = "1")]
    pub block_packages: ::prost::alloc::vec::Vec<BlockPackageInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildingInfo {
    #[prost(int64, optional, tag = "1")]
    pub uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub define_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub r#use: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "4")]
    pub x: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub y: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub rotate: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub level: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildingGainPush {
    #[prost(message, repeated, tag = "1")]
    pub building_infos: ::prost::alloc::vec::Vec<BuildingInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildingLevelUpPush {
    #[prost(message, repeated, tag = "1")]
    pub building_infos: ::prost::alloc::vec::Vec<BuildingInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyManufactureBuildingInfoReply {
    #[prost(int32, optional, tag = "1")]
    pub building_id: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "2")]
    pub building_uid: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyManufactureBuildingRequest {
    #[prost(int32, optional, tag = "1")]
    pub building_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyRestSlotReply {
    #[prost(int64, optional, tag = "1")]
    pub building_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub buy_slot_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyRestSlotRequest {
    #[prost(int64, optional, tag = "1")]
    pub building_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub buy_slot_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangePurchaseOrderTraceStateReply {
    #[prost(int32, optional, tag = "1")]
    pub order_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub is_trace: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangePurchaseOrderTraceStateRequest {
    #[prost(int32, optional, tag = "1")]
    pub order_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub is_trace: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeRestCritterReply {
    #[prost(int64, optional, tag = "1")]
    pub building_uid: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeRestCritterRequest {
    #[prost(int64, optional, tag = "1")]
    pub building_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub operation: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub slot_id1: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "4")]
    pub critter_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "5")]
    pub slot_id2: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CharacterInteractionInfo {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub finish: ::core::option::Option<bool>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub select_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CopyOtherRoomPlanReply {
    #[prost(int64, optional, tag = "1")]
    pub target_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub cover_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CopyOtherRoomPlanRequest {
    #[prost(int64, optional, tag = "1")]
    pub target_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub cover_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRoadReply {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRoadRequest {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRoomPlanReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRoomPlanRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRoomShareReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRoomShareRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DispatchCritterInfo {
    #[prost(int32, optional, tag = "1")]
    pub critter_slot_id: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "2")]
    pub critter_uid: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoadCritterInfo {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "5")]
    pub critter_uid: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DispatchCritterReply {
    #[prost(int64, optional, tag = "1")]
    pub critter_uid: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub building_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "3")]
    pub put_slot_id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "4")]
    pub infos: ::prost::alloc::vec::Vec<RoadCritterInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DispatchCritterRequest {
    #[prost(int64, optional, tag = "1")]
    pub critter_uid: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub building_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "3")]
    pub critter_slot_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedCritterReply {
    #[prost(int64, optional, tag = "1")]
    pub critter_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub current_mood: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedCritterRequest {
    #[prost(int64, optional, tag = "1")]
    pub critter_uid: ::core::option::Option<i64>,
    #[prost(message, optional, tag = "2")]
    pub use_food_data: ::core::option::Option<MaterialData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductionData {
    #[prost(uint32, optional, tag = "1")]
    pub production_id: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2")]
    pub quantity: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurchaseOrderInfo {
    #[prost(int32, optional, tag = "1")]
    pub order_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub last_refresh_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub buyer_id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "4")]
    pub goods_info: ::prost::alloc::vec::Vec<ProductionData>,
    #[prost(bool, optional, tag = "5")]
    pub is_advanced: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub is_traced: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "7")]
    pub refresh_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub quality: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "9")]
    pub is_locked: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishOrderReply {
    #[prost(int32, optional, tag = "1")]
    pub order_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub order_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub new_purchase_order_info: ::core::option::Option<PurchaseOrderInfo>,
    #[prost(int64, optional, tag = "4")]
    pub sold_count: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "5")]
    pub remain_refresh_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub weekly_wholesale_revenue: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishOrderRequest {
    #[prost(int32, optional, tag = "1")]
    pub order_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub order_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub sell_count: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FormulaInfo {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FormulaGainPush {
    #[prost(message, repeated, tag = "1")]
    pub formula_infos: ::prost::alloc::vec::Vec<FormulaInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FormulaProduce {
    #[prost(int32, optional, tag = "1")]
    pub formula_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub count: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GainGuideBuildingReply {
    #[prost(int32, optional, tag = "1")]
    pub guide_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub step: ::core::option::Option<i32>,
    #[prost(int64, repeated, packed = "false", tag = "3")]
    pub building_uids: ::prost::alloc::vec::Vec<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GainGuideBuildingRequest {
    #[prost(int32, optional, tag = "1")]
    pub guide_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub step: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductionLineInfo {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub formula_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub finish_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub next_finish_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub pause_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub level: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GainProductionLineReply {
    #[prost(message, repeated, tag = "1")]
    pub production_lines: ::prost::alloc::vec::Vec<ProductionLineInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GainProductionLineRequest {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomHeroData {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub current_faith: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub next_refresh_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub skin: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub current_minute: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GainRoomHeroFaithReply {
    #[prost(message, repeated, tag = "1")]
    pub room_hero_datas: ::prost::alloc::vec::Vec<RoomHeroData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GainRoomHeroFaithRequest {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub hero_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GainSpecialBlockPush {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub special_blocks: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoadPoint {
    #[prost(int32, optional, tag = "1")]
    pub x: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub y: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoadInfo {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub from_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub to_type: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "4")]
    pub road_points: ::prost::alloc::vec::Vec<RoadPoint>,
    #[prost(int64, optional, tag = "5")]
    pub critter_uid: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "6")]
    pub building_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "7")]
    pub building_define_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub skin_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "9")]
    pub block_clean_type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateRoadReply {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "2")]
    pub valid_road_infos: ::prost::alloc::vec::Vec<RoadInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateRoadRequest {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "2")]
    pub road_infos: ::prost::alloc::vec::Vec<RoadInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpecialBlockInfo {
    #[prost(int32, optional, tag = "1")]
    pub block_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub create_time: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockPackageInfoReply {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub block_package_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "2")]
    pub special_blocks: ::prost::alloc::vec::Vec<SpecialBlockInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockPackageInfoRequset {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBuildingInfoReply {
    #[prost(message, repeated, tag = "1")]
    pub building_infos: ::prost::alloc::vec::Vec<BuildingInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBuildingInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCharacterInteractionBonusReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub select_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCharacterInteractionBonusRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub select_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCharacterInteractionInfoReply {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<CharacterInteractionInfo>,
    #[prost(int32, optional, tag = "2")]
    pub interaction_count: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCharacterInteractionInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFrozenItemInfoReply {
    #[prost(message, repeated, tag = "1")]
    pub frozen_items2_count: ::prost::alloc::vec::Vec<M2qEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFrozenItemInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlotInfo {
    #[prost(int32, optional, tag = "1")]
    pub slot_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub priority: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub production_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub slot_status: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub inventory_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub begin_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub next_finish_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub pause_time: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManuBuildingInfo {
    #[prost(int64, optional, tag = "1")]
    pub building_uid: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "2")]
    pub slot_infos: ::prost::alloc::vec::Vec<SlotInfo>,
    #[prost(message, repeated, tag = "3")]
    pub critter_infos: ::prost::alloc::vec::Vec<DispatchCritterInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestBuildingInfo {
    #[prost(int64, optional, tag = "1")]
    pub building_uid: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "2")]
    pub unlock_slot_infos: ::prost::alloc::vec::Vec<DispatchCritterInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManufactureInfoReply {
    #[prost(int32, optional, tag = "1")]
    pub trade_level: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub manu_building_infos: ::prost::alloc::vec::Vec<ManuBuildingInfo>,
    #[prost(message, repeated, tag = "3")]
    pub rest_building_infos: ::prost::alloc::vec::Vec<RestBuildingInfo>,
    #[prost(message, repeated, tag = "4")]
    pub frozen_items2_count: ::prost::alloc::vec::Vec<M2qEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManufactureInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WholesaleOrderInfo {
    #[prost(int32, optional, tag = "1")]
    pub order_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub good_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub today_sold_count: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderInfoReply {
    #[prost(int32, optional, tag = "1")]
    pub purchase_order_finish_count: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub purchase_order_infos: ::prost::alloc::vec::Vec<PurchaseOrderInfo>,
    #[prost(message, repeated, tag = "3")]
    pub wholesale_order_infos: ::prost::alloc::vec::Vec<WholesaleOrderInfo>,
    #[prost(int32, optional, tag = "4")]
    pub remain_refresh_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub weekly_wholesale_revenue: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomSkinInfo {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub skin_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOtherRoomObInfoReply {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<BlockInfo>,
    #[prost(message, repeated, tag = "2")]
    pub building_infos: ::prost::alloc::vec::Vec<BuildingInfo>,
    #[prost(int64, optional, tag = "3")]
    pub target_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "4")]
    pub room_level: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "5")]
    pub room_hero_datas: ::prost::alloc::vec::Vec<RoomHeroData>,
    #[prost(message, repeated, tag = "6")]
    pub production_lines: ::prost::alloc::vec::Vec<ProductionLineInfo>,
    #[prost(string, optional, tag = "7")]
    pub share_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "8")]
    pub skins: ::prost::alloc::vec::Vec<RoomSkinInfo>,
    #[prost(message, repeated, tag = "9")]
    pub road_infos: ::prost::alloc::vec::Vec<RoadInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOtherRoomObInfoRequest {
    #[prost(int64, optional, tag = "1")]
    pub target_uid: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoomInfoReply {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<BlockInfo>,
    #[prost(bool, optional, tag = "2")]
    pub is_reset: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "3")]
    pub building_infos: ::prost::alloc::vec::Vec<BuildingInfo>,
    #[prost(message, repeated, tag = "4")]
    pub block_packages: ::prost::alloc::vec::Vec<BlockPackageInfo>,
    #[prost(message, repeated, tag = "5")]
    pub road_infos: ::prost::alloc::vec::Vec<RoadInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoomInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomLogInfo {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "5")]
    pub is_new: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoomLogReply {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<RoomLogInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoomLogRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoomObInfoReply {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<BlockInfo>,
    #[prost(message, repeated, tag = "2")]
    pub building_infos: ::prost::alloc::vec::Vec<BuildingInfo>,
    #[prost(message, repeated, tag = "3")]
    pub formula_infos: ::prost::alloc::vec::Vec<FormulaInfo>,
    #[prost(int32, optional, tag = "4")]
    pub room_level: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "5")]
    pub room_hero_datas: ::prost::alloc::vec::Vec<RoomHeroData>,
    #[prost(message, repeated, tag = "6")]
    pub production_lines: ::prost::alloc::vec::Vec<ProductionLineInfo>,
    #[prost(int32, repeated, packed = "false", tag = "7")]
    pub has_get_room_themes: ::prost::alloc::vec::Vec<i32>,
    #[prost(bool, optional, tag = "8")]
    pub need_block_data: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "9")]
    pub skins: ::prost::alloc::vec::Vec<RoomSkinInfo>,
    #[prost(message, repeated, tag = "10")]
    pub road_infos: ::prost::alloc::vec::Vec<RoadInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoomObInfoRequest {
    #[prost(bool, optional, tag = "1")]
    pub need_block_data: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomPlanInfo {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub infos: ::prost::alloc::vec::Vec<BlockInfo>,
    #[prost(message, repeated, tag = "3")]
    pub building_infos: ::prost::alloc::vec::Vec<BuildingInfo>,
    #[prost(int32, optional, tag = "4")]
    pub cover_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "5")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "6")]
    pub building_degree: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub block_count: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "8")]
    pub share_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "9")]
    pub use_count: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "10")]
    pub skins: ::prost::alloc::vec::Vec<RoomSkinInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoomPlanDetailsReply {
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<RoomPlanInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoomPlanDetailsRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoomPlanInfoReply {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<RoomPlanInfo>,
    #[prost(int32, optional, tag = "2")]
    pub can_share_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub can_use_share_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub total_use_count: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoomPlanInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoomShareReply {
    #[prost(int32, optional, tag = "1")]
    pub zone_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub share_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub room_plan_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub nick_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "5")]
    pub share_user_id: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "6")]
    pub infos: ::prost::alloc::vec::Vec<BlockInfo>,
    #[prost(message, repeated, tag = "7")]
    pub building_infos: ::prost::alloc::vec::Vec<BuildingInfo>,
    #[prost(int32, optional, tag = "8")]
    pub building_degree: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "9")]
    pub block_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "10")]
    pub use_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "11")]
    pub portrait: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "12")]
    pub skins: ::prost::alloc::vec::Vec<RoomSkinInfo>,
    #[prost(message, repeated, tag = "13")]
    pub road_infos: ::prost::alloc::vec::Vec<RoadInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoomShareRequest {
    #[prost(string, optional, tag = "1")]
    pub share_code: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoomThemeCollectionBonusReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoomThemeCollectionBonusRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTradeSupportBonusReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTradeSupportBonusRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTradeTaskExtraBonusReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTradeTaskExtraBonusRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeTaskInfo {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub progress: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub has_finish: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub new: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "5")]
    pub finish_time: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTradeTaskInfoReply {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<TradeTaskInfo>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub has_get_support_bonus: ::prost::alloc::vec::Vec<i32>,
    #[prost(bool, optional, tag = "3")]
    pub can_get_extra_bonus: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTradeTaskInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HideBlockPackageReddotReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HideBlockPackageReddotRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HideBuildingReddotReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HideBuildingReddotRequset {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockOrderReply {
    #[prost(int32, optional, tag = "1")]
    pub order_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub is_locked: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockOrderRequest {
    #[prost(int32, optional, tag = "1")]
    pub order_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub operation: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManuBuildingInfoPush {
    #[prost(message, repeated, tag = "1")]
    pub manu_building_infos: ::prost::alloc::vec::Vec<ManuBuildingInfo>,
    #[prost(message, repeated, tag = "2")]
    pub frozen_items2_count: ::prost::alloc::vec::Vec<M2qEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManuBuildingUpgradeReply {
    #[prost(message, optional, tag = "1")]
    pub manu_building_info: ::core::option::Option<ManuBuildingInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManuBuildingUpgradeRequest {
    #[prost(int64, optional, tag = "1")]
    pub uid: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManufactureAccelerateReply {
    #[prost(message, repeated, tag = "1")]
    pub manu_building_infos: ::prost::alloc::vec::Vec<ManuBuildingInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManufactureAccelerateRequest {
    #[prost(int64, optional, tag = "1")]
    pub uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub slot_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub use_item_data: ::core::option::Option<MaterialData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationInfo {
    #[prost(int32, optional, tag = "1")]
    pub slot_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub operation: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub production_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub priority: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductionLineAccelerateReply {
    #[prost(message, optional, tag = "1")]
    pub production_line: ::core::option::Option<ProductionLineInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductionLineAccelerateRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub use_item_count: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductionLineInfoReply {
    #[prost(message, repeated, tag = "1")]
    pub production_lines: ::prost::alloc::vec::Vec<ProductionLineInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductionLineInfoRequest {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductionLineLvUpReply {
    #[prost(message, optional, tag = "1")]
    pub production_line: ::core::option::Option<ProductionLineInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductionLineLvUpRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub new_level: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadNewTradeTaskReply {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadNewTradeTaskRequest {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRoomLogNewReply {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub index: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRoomLogNewRequest {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub index: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRoomSkinReply {
    #[prost(int32, optional, tag = "1")]
    pub skin_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRoomSkinRequest {
    #[prost(int32, optional, tag = "1")]
    pub skin_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReapFinishSlotReply {
    #[prost(int64, optional, tag = "1")]
    pub building_uid: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "2")]
    pub manu_building_infos: ::prost::alloc::vec::Vec<ManuBuildingInfo>,
    #[prost(message, repeated, tag = "3")]
    pub normal_reap_item: ::prost::alloc::vec::Vec<MaterialData>,
    #[prost(message, repeated, tag = "4")]
    pub cri_reap_item: ::prost::alloc::vec::Vec<MaterialData>,
    #[prost(message, repeated, tag = "5")]
    pub occupied_cri_item: ::prost::alloc::vec::Vec<MaterialData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReapFinishSlotRequest {
    #[prost(int64, optional, tag = "1")]
    pub building_uid: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshPurchaseOrderReply {
    #[prost(message, optional, tag = "1")]
    pub order_info: ::core::option::Option<PurchaseOrderInfo>,
    #[prost(int32, optional, tag = "2")]
    pub remain_refresh_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub guide_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub step: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshPurchaseOrderRequest {
    #[prost(int32, optional, tag = "1")]
    pub order_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub guide_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub step: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplaceRestBuildingCrittersReply {
    #[prost(int64, optional, tag = "1")]
    pub building_uid: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplaceRestBuildingCrittersRequest {
    #[prost(int64, optional, tag = "1")]
    pub building_uid: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportRoomReply {
    #[prost(string, optional, tag = "1")]
    pub token: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportRoomRequest {
    #[prost(int64, optional, tag = "1")]
    pub reported_user_id: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub report_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub content: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub share_code: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetRoomReply {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<BlockInfo>,
    #[prost(message, repeated, tag = "2")]
    pub block_packages: ::prost::alloc::vec::Vec<BlockPackageInfo>,
    #[prost(message, repeated, tag = "3")]
    pub road_infos: ::prost::alloc::vec::Vec<RoadInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetRoomRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestBuildingInfoPush {
    #[prost(message, repeated, tag = "1")]
    pub rest_building_infos: ::prost::alloc::vec::Vec<RestBuildingInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoadInfoPush {
    #[prost(message, repeated, tag = "1")]
    pub road_infos: ::prost::alloc::vec::Vec<RoadInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomConfirmReply {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<BlockInfo>,
    #[prost(message, repeated, tag = "2")]
    pub building_infos: ::prost::alloc::vec::Vec<BuildingInfo>,
    #[prost(message, repeated, tag = "3")]
    pub formula_infos: ::prost::alloc::vec::Vec<FormulaInfo>,
    #[prost(int32, optional, tag = "4")]
    pub room_level: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "5")]
    pub room_hero_datas: ::prost::alloc::vec::Vec<RoomHeroData>,
    #[prost(message, repeated, tag = "6")]
    pub production_lines: ::prost::alloc::vec::Vec<ProductionLineInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomConfirmRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomLevelUpReply {
    #[prost(int32, optional, tag = "1")]
    pub room_level: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub production_lines: ::prost::alloc::vec::Vec<ProductionLineInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomLevelUpRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomRevertReply {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<BlockInfo>,
    #[prost(message, repeated, tag = "2")]
    pub block_packages: ::prost::alloc::vec::Vec<BlockPackageInfo>,
    #[prost(message, repeated, tag = "3")]
    pub building_infos: ::prost::alloc::vec::Vec<BuildingInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomRevertRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidDispatchCritterInfo {
    #[prost(int64, optional, tag = "1")]
    pub critter_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub slot_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidDispatchInfo {
    #[prost(int64, optional, tag = "1")]
    pub building_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub road_id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "3")]
    pub infos: ::prost::alloc::vec::Vec<ValidDispatchCritterInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouseCrittersReply {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub road_infos: ::prost::alloc::vec::Vec<RoadInfo>,
    #[prost(message, repeated, tag = "3")]
    pub valid_infos: ::prost::alloc::vec::Vec<ValidDispatchInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouseCrittersRequest {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub infos: ::prost::alloc::vec::Vec<BatchDispatchInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelectSlotProductionPlanReply {
    #[prost(message, repeated, tag = "1")]
    pub manu_building_infos: ::prost::alloc::vec::Vec<ManuBuildingInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelectSlotProductionPlanRequest {
    #[prost(int64, optional, tag = "1")]
    pub uid: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "2")]
    pub operation_infos: ::prost::alloc::vec::Vec<OperationInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRoomPlanCoverReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub cover_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRoomPlanCoverRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub cover_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRoomPlanNameReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRoomPlanNameRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRoomPlanReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub cover_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRoomPlanRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub cover_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRoomSkinReply {
    #[prost(message, optional, tag = "1")]
    pub skin: ::core::option::Option<RoomSkinInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRoomSkinRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub skin_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetWaterTypeReply {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub block_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "2")]
    pub infos: ::prost::alloc::vec::Vec<BlockInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaterInfo {
    #[prost(int32, optional, tag = "1")]
    pub block_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub water_type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetWaterTypeRequest {
    #[prost(message, repeated, tag = "1")]
    pub water_infos: ::prost::alloc::vec::Vec<WaterInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareRoomPlanReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub share_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub can_share_count: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareRoomPlanRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartCharacterInteractionReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartCharacterInteractionRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartProductionLineReply {
    #[prost(message, optional, tag = "1")]
    pub production_line: ::core::option::Option<ProductionLineInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartProductionLineRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub formula_produce: ::prost::alloc::vec::Vec<FormulaProduce>,
    #[prost(message, repeated, tag = "3")]
    pub cost_check: ::prost::alloc::vec::Vec<MaterialData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchRoomPlanReply {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<RoomPlanInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchRoomPlanRequest {
    #[prost(int32, optional, tag = "1")]
    pub id_a: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub id_b: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeLevelUpReply {
    #[prost(int32, optional, tag = "1")]
    pub level: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeLevelUpRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeTaskPush {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<TradeTaskInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnUseBlockReply {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub block_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "2")]
    pub building_infos: ::prost::alloc::vec::Vec<BuildingInfo>,
    #[prost(message, repeated, tag = "3")]
    pub road_infos: ::prost::alloc::vec::Vec<RoadInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnUseBlockRequest {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub block_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnUseBuildingReply {
    #[prost(message, repeated, tag = "1")]
    pub building_infos: ::prost::alloc::vec::Vec<BuildingInfo>,
    #[prost(message, repeated, tag = "2")]
    pub road_infos: ::prost::alloc::vec::Vec<RoadInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnUseBuildingRequest {
    #[prost(int64, optional, tag = "1")]
    pub uid: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnloadRestBuildingCrittersReply {
    #[prost(int64, optional, tag = "1")]
    pub building_uid: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnloadRestBuildingCrittersRequest {
    #[prost(int64, optional, tag = "1")]
    pub building_uid: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRoomHeroDataReply {
    #[prost(message, repeated, tag = "1")]
    pub room_hero_datas: ::prost::alloc::vec::Vec<RoomHeroData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRoomHeroDataRequest {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub room_hero_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseBlockReply {
    #[prost(int32, optional, tag = "1")]
    pub block_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub rotate: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub x: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub y: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseBlockRequest {
    #[prost(int32, optional, tag = "1")]
    pub block_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub block_package_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub rotate: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub x: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub y: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseBuildingReply {
    #[prost(message, optional, tag = "1")]
    pub building_info: ::core::option::Option<BuildingInfo>,
    #[prost(message, repeated, tag = "2")]
    pub delete_building_infos: ::prost::alloc::vec::Vec<BuildingInfo>,
    #[prost(message, repeated, tag = "3")]
    pub delete_road_infos: ::prost::alloc::vec::Vec<RoadInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseBuildingRequest {
    #[prost(int64, optional, tag = "1")]
    pub uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "2")]
    pub rotate: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub x: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub y: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseRoomPlanReply {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseRoomPlanRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseRoomShareReply {
    #[prost(string, optional, tag = "1")]
    pub share_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub cover_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "5")]
    pub can_use_share_count: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseRoomShareRequest {
    #[prost(string, optional, tag = "1")]
    pub share_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub cover_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChooseEnhancedPoolHeroReply {
    #[prost(int32, optional, tag = "1")]
    pub pool_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub hero_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChooseEnhancedPoolHeroRequest {
    #[prost(int32, optional, tag = "1")]
    pub pool_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub hero_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChooseMultiUpHeroReply {
    #[prost(int32, optional, tag = "1")]
    pub pool_id: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub hero_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChooseMultiUpHeroRequest {
    #[prost(int32, optional, tag = "1")]
    pub pool_id: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub hero_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LuckyBagInfo {
    #[prost(int32, optional, tag = "1")]
    pub count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub lucky_bag_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub open_lb_times: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpPoolInfo {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub up_hero_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "3")]
    pub limited_ticket_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub limited_ticket_num: ::core::option::Option<i32>,
    #[prost(uint64, optional, tag = "5")]
    pub open_time: ::core::option::Option<u64>,
    #[prost(bool, optional, tag = "6")]
    pub used_first_ssr_guarantee: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SummonPoolInfo {
    #[prost(int32, optional, tag = "1")]
    pub pool_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub online_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub offline_time: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "4")]
    pub have_free: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "5")]
    pub used_free_count: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "6")]
    pub lucky_bag_info: ::core::option::Option<LuckyBagInfo>,
    #[prost(message, optional, tag = "7")]
    pub sp_pool_info: ::core::option::Option<SpPoolInfo>,
    #[prost(int32, optional, tag = "8")]
    pub discount_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "9")]
    pub can_get_guarantee_sr_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "10")]
    pub guarantee_sr_count_down: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSummonInfoReply {
    #[prost(bool, optional, tag = "1")]
    pub free_equip_summon: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "2")]
    pub is_show_new_summon: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "3")]
    pub new_summon_count: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "4")]
    pub pool_infos: ::prost::alloc::vec::Vec<SummonPoolInfo>,
    #[prost(int32, optional, tag = "5")]
    pub total_summon_count: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSummonInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LuckyBagResult {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub is_new: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "3")]
    pub cur_count: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "4")]
    pub return_materials: ::prost::alloc::vec::Vec<MaterialData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenLuckyBagReply {
    #[prost(message, repeated, tag = "1")]
    pub lucky_bag_results: ::prost::alloc::vec::Vec<LuckyBagResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenLuckyBagRequest {
    #[prost(int32, optional, tag = "1")]
    pub lucky_bag_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub hero_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SummonQueryTokenReply {
    #[prost(string, optional, tag = "1")]
    pub token: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SummonQueryTokenRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SummonResult {
    #[prost(int32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub is_new: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "3")]
    pub duplicate_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub equip_id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "5")]
    pub return_materials: ::prost::alloc::vec::Vec<MaterialData>,
    #[prost(int32, optional, tag = "6")]
    pub lucky_bag_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub limited_ticket_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SummonReply {
    #[prost(message, repeated, tag = "1")]
    pub summon_result: ::prost::alloc::vec::Vec<SummonResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SummonRequest {
    #[prost(int32, optional, tag = "1")]
    pub pool_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub guide_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub step_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub count: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AchievementTaskInfo {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub progress: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub has_finish: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub new: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "5")]
    pub finish_time: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAchievementInfoReply {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<AchievementTaskInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAchievementInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadNewAchievementReply {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadNewAchievementRequest {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShowAchievementReply {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "2")]
    pub group_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShowAchievementRequest {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "2")]
    pub group_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAchievementPush {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<AchievementTaskInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDialogInfoReply {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub dialog_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDialogInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordDialogInfoReplay {
    #[prost(int32, optional, tag = "1")]
    pub dialog_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordDialogInfoRequest {
    #[prost(int32, optional, tag = "1")]
    pub dialog_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AntiqueInfo {
    #[prost(int32, optional, tag = "1")]
    pub antique_id: ::core::option::Option<i32>,
    #[prost(uint64, optional, tag = "2")]
    pub get_time: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AntiqueUpdatePush {
    #[prost(message, repeated, tag = "1")]
    pub antiques: ::prost::alloc::vec::Vec<AntiqueInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAntiqueInfoReply {
    #[prost(message, repeated, tag = "1")]
    pub antiques: ::prost::alloc::vec::Vec<AntiqueInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAntiqueInfoRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAct189InfoReply {
    #[prost(int32, optional, tag = "1")]
    pub activity_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub has_get_once_bonus: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAct189InfoRequest {
    #[prost(int32, optional, tag = "1")]
    pub activity_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAct189OnceBonusReply {
    #[prost(int32, optional, tag = "1")]
    pub activity_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAct189OnceBonusRequest {
    #[prost(int32, optional, tag = "1")]
    pub activity_id: ::core::option::Option<i32>,
}
