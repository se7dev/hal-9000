//! # Controller
//! This module holds the main functionality of the twitch bot
//! # Submodules
//! ## Filter:
//! Contains a word filter to detect bad language
//! ## Bot:
//! Combining the functionality from the other modules into one app
//! ## Vote:
//! Contains voting functionality
//! ## Database:
//! Connects the app to a mongo database
//! ## Giveaway:
//! Contains giveaway functionality

pub mod filter;
pub mod bot;
pub mod database;
pub mod filter;
pub mod giveaway;
pub mod vote;
