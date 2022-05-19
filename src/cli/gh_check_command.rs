/// Command checks if `GitHub CLI` is installed on remote machine.
pub fn gh_check_command() -> String {
    let message = "⚠️ GitHub CLI is required to be installed on the remote machine.";
    let link = "  Please install it and try again: https://github.com/cli/cli#installation";

    let ask_to_install = format!(
        "echo '{message}\n{link}\n' >&2",
        message = message,
        link = link,
    );

    return format!(
        "if [[ \"$(command -v gh)\" == \"\" ]]; then {ask_to_install} && exit 1; fi",
        ask_to_install = ask_to_install,
    );
}

// #[cfg(test)]
// mod tests {
//     use crate::utils::with_end_new_line;

//     use super::gh_check_command;
//     use k9::assert_matches_snapshot;

//     #[test]
//     fn correct_gh_check_command() {
//         assert_matches_snapshot!(
//             with_end_new_line(gh_check_command()),
//             "Must create correct GH CLI check command."
//         );
//     }
// }
