use crate::command::diff::gh_mutation::gh_mutation;

pub struct Comment {
    pub file_path: String,
    pub line_no: usize,
    pub body: String,
    pub diff_side: DiffSide,
}

pub enum DiffSide {
    Left,
    Right,
}

pub fn post_gh_pr_comments(
    node_id: &str,
    body: &str,
    comments: Vec<Comment>,
) -> Result<(), String> {
    let comments_concat = comments
        .iter()
        .map(|c| {
            let diff_side = match c.diff_side {
                DiffSide::Left => "LEFT",
                _ => "RIGHT",
            };
            format!(
                "{{ body: \"{}\", line: {}, path: \"{}\", side: {} }}",
                c.body, c.line_no, c.file_path, diff_side
            )
        })
        .fold(String::new(), |a, b| a + b.as_str() + ", ");

    let event = "REVIEW";

    let mutation = &format!(
        r#"mutation {{ addPullRequestReview(input: {{ event: {}, pullRequestId: "{}", body: "{}", threads: [{}] }}) {{ clientMutationId }} }}"#,
        event, node_id, body, comments_concat
        ).to_string();

    gh_mutation(mutation)
}
