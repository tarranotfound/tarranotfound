# Terminal UI plan

The current program is line-oriented. A full-screen terminal interface is a separate milestone, not an existing feature.

## Proposed interaction

- A clear expression/result panel.
- A keyboard-navigable keypad and expression editor.
- A small recent-history panel.
- Visible help for evaluate, delete, clear, and quit actions.
- A compact dark theme with restrained borders and no decorative animation.

## Implementation sequence

1. Extract a reusable calculation engine.
2. Add parser unit tests and predictable error results.
3. Choose a terminal UI library such as FTXUI.
4. Keep terminal rendering separate from calculation logic.
5. Test keyboard navigation and resize behavior in a real terminal.
