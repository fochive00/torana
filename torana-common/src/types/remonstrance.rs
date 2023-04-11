
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Remonstrance represents a code review session.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct RemonstranceData {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub comments: Vec<Comment>,
    pub source_view: Vec<CodeSession>,
}

// CodeSession represents a source code session.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CodeSession {
    lines: Vec<Line>,
}

// LineState indicates that a line was being added or removed or unchanged.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LineState {
    Added,
    Removed,
    Unchanged,
}

// Line represents a source code line.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Line {
    pub state: LineState,
    pub content: String,
}

// Comment represents a comment on a code review session.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub author: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    // A optional reference to the relative source code line number.
    pub line_number: Option<u32>,
}