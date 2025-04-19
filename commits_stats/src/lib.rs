use chrono::prelude::*;
use json::JsonValue;
use std::collections::HashMap;

/// Computes the number of commits per GitHub author (by login).
pub fn commits_per_author(data: &JsonValue) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    for commit in data.members() {
        if let Some(login) = commit["author"]["login"].as_str() {
            *counts.entry(login.to_string()).or_insert(0) += 1;
        }
    }
    counts
}

/// Computes the number of commits per ISO week (formatted as "YYYY-Www").
pub fn commits_per_week(data: &JsonValue) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    for commit in data.members() {
        if let Some(date_str) = commit["commit"]["author"]["date"].as_str() {
            if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
                let iso_week = dt.iso_week();
                let week_key = format!("{}-W{}", iso_week.year(), iso_week.week());
                *counts.entry(week_key).or_insert(0) += 1;
            }
        }
    }
    counts
}
