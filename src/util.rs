//! # Controller
//! This module holds the main functionality of the twitch bot
//! # Submodules
//! ## Regex:
//! Contains regex expressions to differentiate between inc messages
//! ## Config:
//! Evaluates the variables provided to start the app
//! ## Get_Item:
//! Helper module to extract parameters given in twitch messages
//! ## Send:
//! Wrapper for sending messages to twitch and handling errors

pub mod regex;
pub mod config;
pub mod get_item;
pub mod regex;
pub mod send;
