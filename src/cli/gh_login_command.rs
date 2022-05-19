/// Check if user is logged in into GitHub CLI as
/// root user, and if not, log in.
pub fn gh_login_command() -> String {
    let set_env_variables = "source /etc/profile.d/codespaces.sh";
    let do_login = format!(
        "{set_env_variables} && (echo \"$GITHUB_TOKEN\" | sudo gh auth login --with-token)",
        set_env_variables = set_env_variables,
    );

    return format!(
        "if [[ \"$(sudo gh auth status 2>&1)\" != *\"Token: ***\"* ]]; then {do_login}; fi",
        do_login = do_login,
    );
}

// #[cfg(test)]
// mod tests {
//     use crate::utils::with_end_new_line;

//     use super::gh_login_command;
//     use k9::assert_matches_snapshot;

//     #[test]
//     fn correct_gh_login_command() {
//         assert_matches_snapshot!(
//             with_end_new_line(gh_login_command()),
//             "Must create correct GH CLI login command."
//         );
//     }
// }
