pub const GIT_VERSION: &str = "git -v";
pub const GIT_CURRENT_BRANCH: &str = "git rev-parse --abbrev-ref HEAD";

pub const GIT_PUSH_REMOTE: &str = "git push -u origin [BRANCH_NAME]";
pub const GIT_PUSH_REMOTE_REGEX: &str = r"\[BRANCH_NAME\]$";