//! # MineStats
//!
//! `mine_stats` is a collection of web service:
//!
//! 0. show request ip like <https://ifconfig.io/ip>
//! 0. get user github stats like <https://github.com/anuraghazra/github-readme-stats>
//! 0. get server service status using `systemctl status service_name`

pub mod api;
mod cache;
mod cards;
pub mod config;
mod error;
mod github;
mod status;
