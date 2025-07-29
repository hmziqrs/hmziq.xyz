use dioxus::prelude::*;
use crate::ui::SectionTitle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GitHubActivityData {
    user: UserData,
    #[serde(rename = "recentActivity")]
    recent_activity: Vec<Activity>,
    #[serde(rename = "topRepositories")]
    top_repositories: Vec<Repository>,
    contributions: ContributionData,
    #[serde(rename = "lastUpdated")]
    last_updated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserData {
    login: String,
    name: Option<String>,
    #[serde(rename = "avatarUrl")]
    avatar_url: String,
    bio: Option<String>,
    followers: u32,
    #[serde(rename = "publicRepos")]
    public_repos: u32,
    #[serde(rename = "profileUrl")]
    profile_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Activity {
    #[serde(rename = "type")]
    activity_type: String,
    repo: String,
    message: Option<String>,
    action: Option<String>,
    #[serde(rename = "createdAt")]
    created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Repository {
    name: String,
    description: Option<String>,
    language: Option<String>,
    stars: u32,
    url: String,
    topics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContributionData {
    #[serde(rename = "totalContributions")]
    total_contributions: u32,
    calendar: ContributionCalendar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContributionCalendar {
    #[serde(rename = "totalContributions")]
    total_contributions: u32,
    weeks: Vec<ContributionWeek>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContributionWeek {
    #[serde(rename = "contributionDays")]
    contribution_days: Vec<ContributionDay>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContributionDay {
    #[serde(rename = "contributionCount")]
    contribution_count: u32,
    date: String,
    color: String,
}

fn get_github_data() -> GitHubActivityData {
    let json_data = include_str!("../../../assets/github-activity.json");
    serde_json::from_str(json_data).expect("Failed to parse GitHub activity data")
}

fn format_time_ago(datetime: &str) -> String {
    if let Ok(parsed) = chrono::DateTime::parse_from_rfc3339(datetime) {
        let now = chrono::Utc::now();
        let duration = now.signed_duration_since(parsed);
        
        if duration.num_days() > 0 {
            format!("{} days ago", duration.num_days())
        } else if duration.num_hours() > 0 {
            format!("{} hours ago", duration.num_hours())
        } else {
            "Just now".to_string()
        }
    } else {
        "Recently".to_string()
    }
}

fn get_activity_icon(activity_type: &str) -> &'static str {
    match activity_type {
        "PushEvent" => "⟶",
        "WatchEvent" => "★",
        "CreateEvent" => "✱",
        "PullRequestEvent" => "⇅",
        "IssuesEvent" => "◉",
        _ => "•"
    }
}

fn get_activity_description(activity: &Activity) -> String {
    match activity.activity_type.as_str() {
        "PushEvent" => format!("Pushed to {}", activity.repo.split('/').last().unwrap_or(&activity.repo)),
        "WatchEvent" => format!("Starred {}", activity.repo),
        "CreateEvent" => format!("Created {}", activity.message.as_ref().unwrap_or(&"repository".to_string())),
        "PullRequestEvent" => format!("{} PR in {}", 
            activity.action.as_ref().unwrap_or(&"Updated".to_string()),
            activity.repo.split('/').last().unwrap_or(&activity.repo)
        ),
        "IssuesEvent" => format!("{} issue in {}", 
            activity.action.as_ref().unwrap_or(&"Updated".to_string()),
            activity.repo.split('/').last().unwrap_or(&activity.repo)
        ),
        _ => format!("Activity in {}", activity.repo.split('/').last().unwrap_or(&activity.repo))
    }
}

fn get_month_label(week_index: usize) -> Option<&'static str> {
    // Show month labels at the start of each month (approximate)
    match week_index {
        0 => Some("Aug"),
        4 => Some("Sep"),
        8 => Some("Oct"),
        13 => Some("Nov"),
        17 => Some("Dec"),
        22 => Some("Jan"),
        26 => Some("Feb"),
        30 => Some("Mar"),
        35 => Some("Apr"),
        39 => Some("May"),
        44 => Some("Jun"),
        48 => Some("Jul"),
        _ => None
    }
}

fn get_contribution_color_from_github(color: &str) -> &'static str {
    match color {
        "#216e39" => "bg-green-900/50",  // Level 4 (darkest)
        "#30a14e" => "bg-green-700/60",  // Level 3
        "#40c463" => "bg-green-500/70",  // Level 2
        "#9be9a8" => "bg-green-400/80",  // Level 1 (lightest)
        _ => "bg-white/5",                // Level 0 (no contributions)
    }
}

#[component]
pub fn GitHubActivitySection() -> Element {
    let data = use_signal(|| get_github_data());
    
    rsx! {
        section {
            id: "github-activity",
            class: "py-36 px-5 max-w-[1400px] mx-auto relative",
            
            // Section header
            div {
                class: "text-center mb-24 relative",
                SectionTitle { text: "LAB ACTIVITY" }
            }
            
            // Stats overview
            div {
                class: "flex justify-center gap-8 mb-20 flex-wrap",
                
                // Followers
                div {
                    class: "bg-black/50 backdrop-blur-sm border border-white/10 rounded-lg p-6 hover:bg-white/5 transition-all duration-300",
                    div {
                        class: "text-3xl font-thin text-white mb-2",
                        "{data().user.followers}"
                    }
                    div {
                        class: "text-sm text-white/60 uppercase tracking-widest",
                        "Followers"
                    }
                }
                
                // Repositories
                div {
                    class: "bg-black/50 backdrop-blur-sm border border-white/10 rounded-lg p-6 hover:bg-white/5 transition-all duration-300",
                    div {
                        class: "text-3xl font-thin text-white mb-2",
                        "{data().user.public_repos}"
                    }
                    div {
                        class: "text-sm text-white/60 uppercase tracking-widest",
                        "Repositories"
                    }
                }
                
                // Stars (from top repo)
                div {
                    class: "bg-black/50 backdrop-blur-sm border border-white/10 rounded-lg p-6 hover:bg-white/5 transition-all duration-300",
                    div {
                        class: "text-3xl font-thin text-white mb-2",
                        "{data().top_repositories.iter().map(|r| r.stars).sum::<u32>()}"
                    }
                    div {
                        class: "text-sm text-white/60 uppercase tracking-widest",
                        "Total Stars"
                    }
                }
            }
            
            // Single column layout
            div {
                class: "max-w-4xl mx-auto space-y-16",
                
                // Recent Activity
                div {
                    class: "space-y-6",
                    div {
                        class: "bg-black/50 backdrop-blur-sm border border-white/10 rounded-lg p-6",
                        h3 {
                            class: "text-2xl font-thin tracking-wider text-white mb-6 text-center",
                            "Recent Activity"
                        }
                        
                        div {
                            class: "space-y-4",
                            for activity in data().recent_activity.iter().take(5) {
                                div {
                                    class: "group relative p-4 rounded-lg bg-white/5 border border-white/10 hover:bg-white/10 transition-all duration-300",
                                    
                                    div {
                                        class: "flex items-start gap-4",
                                        
                                        // Icon
                                        div {
                                            class: "text-2xl text-white/80",
                                            "{get_activity_icon(&activity.activity_type)}"
                                        }
                                        
                                        // Content
                                        div {
                                            class: "flex-1",
                                            div {
                                                class: "text-white/90 mb-1",
                                                "{get_activity_description(activity)}"
                                            }
                                            if let Some(message) = &activity.message {
                                                div {
                                                    class: "text-sm text-white/60 line-clamp-2 mb-2",
                                                    "{message}"
                                                }
                                            }
                                            div {
                                                class: "text-xs text-white/40",
                                                "{format_time_ago(&activity.created_at)}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Contribution Graph
                div {
                    class: "space-y-6",
                    div {
                        class: "bg-black/50 backdrop-blur-sm border border-white/10 rounded-lg p-6",
                        h3 {
                            class: "text-2xl font-thin tracking-wider text-white mb-6 text-center",
                            "{data().contributions.total_contributions} contributions in the last year"
                        }
                        
                        // Contribution heatmap container
                        div {
                            class: "bg-white/5 rounded-lg p-4 md:p-6 border border-white/10 overflow-x-auto",
                            
                            // Inner container for horizontal scroll on mobile
                            div {
                                class: "min-w-fit",
                                
                                // Month labels
                                div {
                                    class: "flex gap-[3px] mb-2 ml-10",
                                    style: "font-size: 11px;",
                                    for week in 0..52 {
                                        if let Some(month) = get_month_label(week) {
                                            div {
                                                class: "text-white/60",
                                                style: format!("width: {}px;", if week == 0 { 30 } else { 50 }),
                                                "{month}"
                                            }
                                        }
                                    }
                                }
                                
                                // Grid container
                                div {
                                    class: "flex gap-1",
                                
                                // Day labels
                                div {
                                    class: "flex flex-col gap-[3px] mr-2 text-white/40",
                                    style: "font-size: 11px;",
                                    div { class: "h-[11px]", "" }
                                    div { class: "h-[11px] leading-[11px]", "Mon" }
                                    div { class: "h-[11px]", "" }
                                    div { class: "h-[11px] leading-[11px]", "Wed" }
                                    div { class: "h-[11px]", "" }
                                    div { class: "h-[11px] leading-[11px]", "Fri" }
                                    div { class: "h-[11px]", "" }
                                }
                                
                                // Contribution grid
                                div {
                                    class: "flex gap-[3px]",
                                    if data().contributions.calendar.weeks.is_empty() {
                                        // Fallback if no contribution data
                                        for _week in 0..52 {
                                            div {
                                                class: "flex flex-col gap-[3px]",
                                                for _day in 0..7 {
                                                    div {
                                                        class: "w-[11px] h-[11px] rounded-sm bg-white/5",
                                                        title: "No data available",
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        // Use real contribution data
                                        for week in data().contributions.calendar.weeks.iter() {
                                            div {
                                                class: "flex flex-col gap-[3px]",
                                                for day in week.contribution_days.iter() {
                                                    div {
                                                        class: format!("w-[11px] h-[11px] rounded-sm hover:ring-1 hover:ring-white/20 transition-all duration-200 cursor-pointer {}", 
                                                            get_contribution_color_from_github(&day.color)),
                                                        title: format!("{} contributions on {}", day.contribution_count, day.date),
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            
                                // Legend
                                div {
                                    class: "flex items-center justify-end gap-2 mt-4 text-white/40",
                                    style: "font-size: 11px;",
                                    span { "Less" }
                                    div { class: "w-[11px] h-[11px] bg-white/5 rounded-sm" }
                                    div { class: "w-[11px] h-[11px] bg-green-900/50 rounded-sm" }
                                    div { class: "w-[11px] h-[11px] bg-green-700/60 rounded-sm" }
                                    div { class: "w-[11px] h-[11px] bg-green-500/70 rounded-sm" }
                                    div { class: "w-[11px] h-[11px] bg-green-400/80 rounded-sm" }
                                    span { "More" }
                                }
                            }
                        }
                    }
                }
            }
            
            // View on GitHub link
            div {
                class: "text-center mt-12",
                a {
                    href: "{data().user.profile_url}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "inline-flex items-center gap-2 px-6 py-3 text-white/80 bg-black/50 backdrop-blur-sm border border-white/10 rounded-lg hover:bg-white/10 hover:text-white hover:border-white/20 transition-all duration-300",
                    "View on GitHub"
                    span { "→" }
                }
            }
        }
    }
}