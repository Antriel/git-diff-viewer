A simple app for viewing and searching the results of `git diff`.
- Compare CWD vs HEAD.
- Compare against commits.
- Search for a string within the added/removed lines.
- Quickly navigate to the change in your editor via a button.

<img width="1101" height="655" alt="250822_154320_485_git-diff-viewer" src="https://github.com/user-attachments/assets/c5ecce69-163a-4e3d-a710-968eb1b7da78" />

My main motivation for this was that I sometimes make a mess of a commit with scattered `TODO` comments that I want to finish before pushing, but searching for `TODO` also shows ones that are not new changes, and are meant to stay (potentially forever).

This was a 2 day project that took a week, as my first test of LLM enhanced development (using Claude Code).
