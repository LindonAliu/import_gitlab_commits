# Import GitLab Commits

The tool to import commits from private GitLab to separate repo. Can be used to show your programming activity for another company in GitHub.

## Getting Started

1. Download and install [Rust](https://www.rust-lang.org/tools/install).
2. Clone the repository

    ```shell
    git clone git@github.com:LindonAliu/import_gitlab_commits.git
    cd import_gitlab_commits
    ```

3. Build and run `import-gitlab-commits`:

    ```shell
    cargo build --release
    mv target/release/import_gitlab_commits .
    ./import_gitlab_commits -gn GITLAB_NAME -n NAME -e EMAIL    
    ```

where

- `GITLAB_NAME` is your GitLab @ username, for exmaple in `gitlab.com/lindon2`, the username is `lindon2`
- `NAME` is your GitHub name
- `EMAIL` is your GitHub email

To show the changes on GitHub you need to:

- create a new repo `yourcompany-contributions` in GitHub;
- open folder `GITLAB_NAME-contributions` created by the executed program;
- add remote url `git remote add origin git@github.com:username/yourcompany-contributions.git`;
- push changes `git push --set-upstream origin master -f`;
