use crate::ui::SectionTitle;
use dioxus::prelude::*;
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

fn get_activity_type_symbol(activity_type: &str) -> &'static str {
    match activity_type {
        "PushEvent" => "→",
        "WatchEvent" => "★",
        "CreateEvent" => "+",
        "PullRequestEvent" => "⇄",
        "IssuesEvent" => "●",
        _ => "·",
    }
}

fn format_activity_log(activity: &Activity) -> String {
    let repo_name = activity.repo.split('/').last().unwrap_or(&activity.repo);

    match activity.activity_type.as_str() {
        "PushEvent" => {
            if let Some(msg) = &activity.message {
                let first_line = msg.lines().next().unwrap_or(msg);
                format!("[PUSH] {} :: {}", repo_name, first_line)
            } else {
                format!("[PUSH] {} :: pushed commits", repo_name)
            }
        }
        "WatchEvent" => format!("[STAR] {} :: starred repository", activity.repo),
        "CreateEvent" => format!(
            "[CREATE] {} :: created {}",
            repo_name,
            activity
                .message
                .as_ref()
                .unwrap_or(&"repository".to_string())
        ),
        "PullRequestEvent" => format!(
            "[PR] {} :: {} pull request",
            repo_name,
            activity.action.as_ref().unwrap_or(&"updated".to_string())
        ),
        "IssuesEvent" => format!(
            "[ISSUE] {} :: {} issue",
            repo_name,
            activity.action.as_ref().unwrap_or(&"updated".to_string())
        ),
        _ => format!(
            "[{}] {} :: activity",
            activity.activity_type.to_uppercase(),
            repo_name
        ),
    }
}

fn get_contribution_color_from_github(color: &str) -> &'static str {
    match color {
        "#ebedf0" => "bg-white/10",
        "#9be9a8" => "bg-green-400/30",
        "#40c463" => "bg-green-400/50",
        "#30a14e" => "bg-green-400/70",
        "#216e39" => "bg-green-400/90",
        _ => "bg-white/5",
    }
}

fn format_date(date_str: &str) -> String {
    use chrono::{Datelike, NaiveDate};

    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        let months = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];
        let month = months[(date.month() - 1) as usize];
        format!("{} {} {}", date.day(), month, date.format("%y"))
    } else {
        date_str.to_string()
    }
}

fn get_month_positions() -> Vec<(usize, &'static str)> {
    use chrono::{Datelike, Duration, Utc};

    let now = Utc::now();
    let one_year_ago = now - Duration::weeks(52);

    let months = [
        "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
    ];
    let mut result = Vec::new();
    let mut current_date = one_year_ago;
    let mut last_month = current_date.month();
    let mut week_index = 0;

    // Track when each month starts in the 52-week grid
    while current_date <= now && week_index < 52 {
        let month = current_date.month();
        if month != last_month {
            let month_name = months[(month - 1) as usize];
            result.push((week_index, month_name));
            last_month = month;
        }
        current_date = current_date + Duration::weeks(1);
        week_index += 1;
    }

    // Always include the first month at position 0
    if result.is_empty() || result[0].0 != 0 {
        let first_month = months[(one_year_ago.month() - 1) as usize];
        result.insert(0, (0, first_month));
    }

    result
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

        // Stats bar
        div {
            class: "flex justify-center gap-8 mb-20 pb-10 border-b border-white/20",

            div {
                class: "text-center",
                div {
                    class: "text-3xl font-thin text-white mb-2 tracking-wider",
                    "{data().user.followers}"
                }
                div {
                    class: "text-xs text-white/60 uppercase tracking-[0.2rem]",
                    "FOLLOWERS"
                }
            }

            div {
                class: "text-center border-l border-r border-white/20 px-8",
                div {
                    class: "text-3xl font-thin text-white mb-2 tracking-wider",
                    "{data().user.public_repos}"
                }
                div {
                    class: "text-xs text-white/60 uppercase tracking-[0.2rem]",
                    "REPOSITORIES"
                }
            }

            div {
                class: "text-center",
                div {
                    class: "text-3xl font-thin text-white mb-2 tracking-wider",
                    "{data().contributions.total_contributions}"
                }
                div {
                    class: "text-xs text-white/60 uppercase tracking-[0.2rem]",
                    "CONTRIBUTIONS"
                }
            }
        }

        // Single column layout
        div {
            class: "max-w-4xl mx-auto space-y-12",

            // Recent Activity Log
            div {
                div {
                    class: "border border-white/20 bg-black/50 backdrop-blur-sm p-8",

                    // Terminal header
                    div {
                        class: "mb-6 pb-4 border-b border-white/20",
                        h3 {
                            class: "text-sm font-light tracking-[0.2rem] text-white/80 uppercase",
                            "ACTIVITY LOG"
                        }
                    }

                    // Activity entries
                    div {
                        class: "space-y-3 font-mono text-xs",
                        for activity in data().recent_activity.iter().take(8) {
                            div {
                                class: "group",

                                // Timestamp and activity
                                div {
                                    class: "flex items-start gap-3",

                                    // Time
                                    span {
                                        class: "text-white/40 shrink-0",
                                        "{format_time_ago(&activity.created_at)}"
                                    }

                                    // Symbol
                                    span {
                                        class: "text-white/60",
                                        "{get_activity_type_symbol(&activity.activity_type)}"
                                    }

                                    // Log message
                                    span {
                                        class: "text-white/80 group-hover:text-white transition-colors duration-300",
                                        "{format_activity_log(activity)}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Contribution Graph
            div {
                div {
                    class: "border border-white/20 bg-black/50 backdrop-blur-sm p-8",

                    // Header
                    div {
                        class: "mb-6 pb-4 border-b border-white/20",
                        h3 {
                            class: "text-sm font-light tracking-[0.2rem] text-white/80 uppercase",
                            "{data().contributions.total_contributions} CONTRIBUTIONS THIS YEAR"
                        }
                    }

                    // Contribution heatmap with responsive scaling
                    div {
                        class: "w-full overflow-x-auto md:overflow-x-visible",

                        // Transform wrapper for desktop scaling
                        div {
                            class: "inline-block origin-top-left md:scale-110 lg:scale-125 xl:scale-150",

                            // Container
                            div {
                                class: "flex flex-col",

                                // Month labels
                                div {
                                    class: "flex mb-2 text-[10px] text-white/40 font-mono uppercase",

                                    // Spacer for day labels
                                    div { class: "w-10 mr-2 shrink-0" }

                                    // Month labels positioned over weeks
                                    div {
                                        class: "relative",
                                        style: "width: 689px;", // 52 weeks * 13px (10px + 3px gap)

                                        for (week_pos, month) in get_month_positions() {
                                            div {
                                                class: "absolute",
                                                style: format!("left: {}px;", week_pos * 13),
                                                "{month}"
                                            }
                                        }
                                    }
                                }

                                // Main grid container
                                div {
                                    class: "flex",

                                    // Day labels
                                    div {
                                        class: "flex flex-col gap-[3px] mr-2 text-[10px] text-white/40 font-mono shrink-0",
                                        div { class: "h-[10px]", "" }
                                        div { class: "h-[10px] leading-[10px]", "MON" }
                                        div { class: "h-[10px]", "" }
                                        div { class: "h-[10px] leading-[10px]", "WED" }
                                        div { class: "h-[10px]", "" }
                                        div { class: "h-[10px] leading-[10px]", "FRI" }
                                        div { class: "h-[10px]", "" }
                                    }

                                    // Weeks container
                                    div {
                                        class: "flex gap-[3px]",

                                        if data().contributions.calendar.weeks.is_empty() {
                                            // Fallback - 52 weeks
                                            for _week in 0..52 {
                                                div {
                                                    class: "flex flex-col gap-[3px]",
                                                    for _day in 0..7 {
                                                        div {
                                                            class: "w-[10px] h-[10px] bg-white/5",
                                                            title: "No data available",
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            // Real contribution data
                                            for week in data().contributions.calendar.weeks.iter() {
                                                div {
                                                    class: "flex flex-col gap-[3px]",
                                                    for day in week.contribution_days.iter() {
                                                        div {
                                                            class: format!("w-[10px] h-[10px] {} hover:ring-1 hover:ring-white/40 transition-all duration-200 cursor-pointer group relative",
                                                                get_contribution_color_from_github(&day.color)),
                                                            title: format!("{} contributions on {}", day.contribution_count, day.date),

                                                            // Enhanced hover tooltip
                                                            div {
                                                                class: "absolute bottom-full left-1/2 -translate-x-1/2 mb-2 px-3 py-1.5 bg-black/90 backdrop-blur-sm border border-white/30 text-white text-xs whitespace-nowrap opacity-0 group-hover:opacity-100 transition-all duration-200 pointer-events-none z-50 shadow-lg",
                                                                "{day.contribution_count} contributions on {format_date(&day.date)}"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Legend
                    div {
                        class: "flex items-center justify-end gap-2 mt-6 text-[10px] text-white/40 font-mono uppercase tracking-wider",
                        span { "LESS" }
                        div { class: "w-[10px] h-[10px] bg-white/10" }
                        div { class: "w-[10px] h-[10px] bg-green-400/30" }
                        div { class: "w-[10px] h-[10px] bg-green-400/50" }
                        div { class: "w-[10px] h-[10px] bg-green-400/70" }
                        div { class: "w-[10px] h-[10px] bg-green-400/90" }
                        span { "MORE" }
                    }
                }
            }
        }

        // View on GitHub link
        div {
            class: "text-center mt-20",
            a {
                href: "{data().user.profile_url}",
                target: "_blank",
                rel: "noopener noreferrer",
                class: "inline-block px-8 py-3 text-xs text-white/80 border border-white/30 uppercase tracking-[0.2rem] transition-all duration-300 hover:text-black hover:bg-white hover:border-white hover:shadow-[0_0_20px_rgba(255,255,255,0.3)] backdrop-blur-sm bg-black/10 cursor-none btn",
                "VIEW ON GITHUB"
            }
        }
    }
    }
}
