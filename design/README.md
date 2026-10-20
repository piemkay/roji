# Design tokens

`tokens.json` holds Roji's design tokens in the W3C Design Tokens format (2025.10 draft; the color `$value` objects carry `colorSpace`, `components` and a `hex` fallback). It is the intended source for colors, typography, spacing, radii, sizes and motion in the web app.

## What it defines

- **color.light:** warm paper, surface, soft ink in four strengths, lines and fills. Nothing is pure black or pure white.
- **color.accent:** four accents (Slate is the default, plus Sage, Clay and Dusk). Only the accent changes; paper, ink and the kind colors stay the same.
- **color.kind:** fixed colors for found words, the pieces of a task's text that Roji recognizes (dates, repeats, projects, people, places), the same in every accent.
- **typography, spacing, radius, size, shadow:** the type styles and scale, and the values used most.
- **motion:** durations and easings for ticking a task off, the path of stones in the header of the Now view that grows with each completed task, and the finish, where Ishi, the app's small companion character, hops along the path. With reduced motion nothing moves; the end state shows at once.

## Modes

Light only for now. Dark and high-contrast colors are added to this file when they are designed.

## Rules

- Components use tokens, never literal colors.
- A value changes here, in one pull request, with the screens it affects checked.
