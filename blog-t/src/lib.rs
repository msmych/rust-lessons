pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approvals_count: 0,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
    approvals_count: u32,
}

impl PendingReviewPost {
    pub fn approve(&mut self) {
        self.approvals_count += 1;
    }

    pub fn approved(self) -> Option<Post> {
        Some(Post {
            content: self.content,
        })
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}
