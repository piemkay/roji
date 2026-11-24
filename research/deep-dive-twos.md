# Deep dive: Twos

Research for [#67](https://github.com/piemkay/roji/issues/67), part of the MVP spec map [#43](https://github.com/piemkay/roji/issues/43). Sources were read on 2026-11-24.

## Question

How does Twos capture with low effort (suggesting whether a line is a note or a to-do), how does it carry unfinished items forward, and how does it organise lists? What do users love and dislike, and which ideas fit Roji's capture and backlog?

## Short answer

Twos is a free notes-and-to-dos app built around days. Everything you write lands on **Today** unless you say otherwise. A line starts as a plain "thing", and Twos may offer to turn it into a to-do ("Sounds like a to-do. Add checkbox / Leave as note"). Unfinished to-dos move themselves to the next day when you open the app. It is the same item moving, not a copy, and it keeps a "Carried over N times" counter. Nothing turns red and nothing is counted as overdue. Lists are flat with optional sublists, there are no folders, and tags drive "dynamic lists". Users praise how fast it is to write things down, the rollover and the no-guilt tone, and many describe it as ADHD-friendly. Their complaints are the number of settings and paid tiers, old notes resurfacing, and difficulty narrowing a long captured list down to a few priorities.

For Roji, the ideas worth taking are capture to Today by default with type decided afterwards, a one-tap note-or-to-do suggestion that the user can dismiss, rollover as a move with a carried-over counter, no overdue styling, and per-list auto-reset for routines. Twos has no cap on how far the backlog can grow and nothing that suggests a short Today. A to-do can roll forward forever, so the pile-up Roji wants to avoid is left to the user.

## How it works

### The model: things, lists, days

- "Twos has two nouns. You write *things*, and *things* live in lists." A thing is usually one line. Its type is not fixed ([Things and lists](https://twosapp.com/docs/start/things-and-lists)).
- Every date gets its own list, which Twos creates and names. Days are real lists: you can reorder, share and search them ([Today](https://twosapp.com/docs/days/today)).
- Types are dash (the default), to-do, note, bullet, number and divider. "A to-do is the only *thing* that carries forward to the next day." The docs frame typing as something you do after writing: "You write, and then, if it turns out to be a to-do, you give it a checkbox" ([Thing types](https://twosapp.com/docs/things/types)).
- The docs' stated intent: "Not a place you have to tend. If you never make a single list and only ever write onto your days, it still works. That is the intended path, not the degraded one" ([What is Twos](https://twosapp.com/docs/start/what-is-twos)).

### Low-effort capture

- **Capture bar.** ⌘K / Ctrl+K on desktop, pinned to the bottom on phones. Press Enter after each item. The bar stays open, so items stack up. A system-wide Quick Capture window is ⌘⌃Space. Pasting several lines offers to split them into separate things. An "Add to" chip shows and changes the destination ([Capture bar](https://twosapp.com/docs/write/capture-bar)).
- **Where things land.** From the capture bar, things go to Today. From Siri, the share sheet, email and other integrations, they go to Today unless a default list is set. From an open list or day, they go to that list or day ([Things and lists](https://twosapp.com/docs/start/things-and-lists)).
- **Dates in plain language.** "Call the dentist Thursday at 3" is recognised as a reminder while you type ([Capture bar](https://twosapp.com/docs/write/capture-bar)).
- **Note-or-to-do suggestion.** The homepage demo shows a typed line "Call mom" and a prompt: "Sounds like a to-do. Want to add a checkbox to 'Call mom'? [Add checkbox] [Leave as note]" ([twosapp.com](https://www.twosapp.com/)). The settings list has "Detect to-dos", "Auto-prompt for to-dos" and "Auto-apply verb detection" under Lists & things, and a separate "To-dos" toggle under Smart features ([Settings A-Z](https://www.twosapp.com/docs/reference/settings)). The docs don't explain these settings. My reading of the names: detection looks for a leading verb, by default Twos asks before changing the line, and auto-apply skips the question. Product Hunt material calls this "detect tasks" as part of the PALs AI feature, and the founders would not say how it works ([Product Hunt, Twos PALs](https://www.producthunt.com/p/twos/twos-pals)). The default type stays dash and can be changed under Settings → Default thing type.
- **Voice, Ramble (Plus).** You talk freely and Ramble splits what you said into separate things, "with to-dos detected where you clearly stated one". The docs call the result "an interpretation, not a transcript" and advise checking it. It needs AI turned on, and you can use your own OpenAI key ([Ramble](https://twosapp.com/docs/write/ramble)).
- **Channels.** Email, text, WhatsApp, Slack, Discord, the browser extension, Apple Watch and Wear OS, Raycast, Zapier, an API and **MCP** (ChatGPT, Claude). Apple Reminders sync both ways and calendars sync in ([twosapp.com](https://www.twosapp.com/)).
- **Suggested actions (AI).** For some tasks Twos adds links or drafted text, for example shopping links for "buy napkins", or a message or gift idea for a birthday. TechCrunch notes the suggestions are US-centric ([TechCrunch, 2024-12-05](https://techcrunch.com/2024/12/05/twos-to-do-app-adds-a-feature-that-automatically-adds-a-suggested-actions-against-a-task)).

### Carrying unfinished items forward

All of this is from [How unfinished to-dos roll forward](https://twosapp.com/docs/days/carry-over) unless noted:

- **When.** "Carry-over runs when you open the app, not on a server at midnight." It is on by default.
- **What moves.** Unchecked to-dos move, along with anything indented under them (notes, sub-to-dos). Checked and cancelled to-dos stay on their day. Top-level notes, photos and links stay too, as a record of that day. The rule depends on where an item sits, not on its type.
- **Move, not copy.** "A to-do that carries over is the same thing, not a copy." Tags, notes, reminders and history come with it.
- **Counter.** A "Carried over N times" counter tracks how many days a to-do has rolled forward.
- **Skipped days.** Only the last day you used Twos is carried automatically. For older days there is a manual carry-over button. A Product Hunt user reported the same gap after skipping a day.
- **Opting out.** There is a global switch. For a single item there is no opt-out: you delete it, move it to a list, or un-indent a note.
- **Reminders.** The to-do moves but the alert doesn't fire again. "Carry over reminders" is off by default, "because it changes when you get notified, which is not a decision to make on someone's behalf." The tone is deliberate: "A wall of red is a punishment… No red, and no counting. There is no overdue badge." The docs' advice for a reminder that keeps going overdue: "Cancel it rather than rescheduling it a fourth time" ([Overdue reminders](https://twosapp.com/docs/reminders/overdue)).
- **Routines are the exception.** "Routines are the one case where carry-over is wrong." Auto-reset (Plus) unchecks a list daily, weekly, monthly or yearly. It runs on the next open, and a missed reset doesn't stack ([Auto reset](https://twosapp.com/docs/lists/auto-reset)).
- **Manual moves.** ⌘⇧O moves an item to Today and ⌘⇧P to Tomorrow. You can drag onto the week bar, or bind a swipe to "move". "Move to Tomorrow is the whole evening routine" ([Move and copy](https://twosapp.com/docs/things/move-and-copy)). Week view (Plus) shows seven days so you can rebalance them ([Week view](https://twosapp.com/docs/days/week-view)).
- **Positioning.** Coming from Todoist or Things: "Carry-over replaces due-date triage: unfinished to-dos follow you rather than accumulating as overdue" ([Switching](https://twosapp.com/docs/start/switching)). The homepage says: "No re-planning, no guilt-scrolling."

### Organising lists

- **No folders.** Lists are flat. A **sublist** is a thing promoted into a list (⌘⇧K), and items indented under it come along. Its contents can show inline so it doesn't feel like a folder. A list can sit in several parent lists. The docs suggest two levels of nesting and say four is too many ([Sublists](https://twosapp.com/docs/lists/sublists)).
- **Tags and dynamic lists.** "Tags are not the filing system." A dynamic list is a tag filter: "It has no contents of its own, and it is always current." Things stay where they were written ([Dynamic lists](https://twosapp.com/docs/lists/dynamic-lists)).
- **Links.** Typing `@` links a thing to a list, and the linked list shows a "Linked from" section. Backlinks are a Plus feature ([Linking](https://twosapp.com/docs/lists/linking)).
- **Auto-sort.** Auto-sort is free. It groups a list by what each item is: to-dos, plain things, reminders, starred, completed and cancelled. It does not sort by topic or with AI. A parent moves with its children ([Auto-sort](https://twosapp.com/docs/lists/auto-sort)). Sorting is "a view, not an edit" ([Sorting](https://twosapp.com/docs/lists/sorting)).
- **Upkeep.** Each list either keeps completed items where they are or collects them into their own section ([Completed](https://twosapp.com/docs/lists/completed)). Archive (Plus) hides a list but keeps it searchable ([Archive](https://twosapp.com/docs/lists/archive)). A list can have a review date, and due lists collect in a Review hub instead of sending a notification for each one ([Review a list](https://twosapp.com/docs/lists/review-a-list)). Blank past days are cleaned up automatically ([Blank days](https://twosapp.com/docs/days/day-housekeeping)).
- **Focus (Plus).** Pins one item to the top with a countdown, a count-up timer or no limit. Completing the item ends the session, and confetti fires when the time runs out ([Focus timer](https://twosapp.com/docs/things/focus-timer)).

### Business facts

Twos is free for writing, with no limit. Twos Plus costs $5/month or $40/year. Individual add-ons cost $1.99 each, and there are "Believer" and "Champion" supporter tiers ([App Store](https://apps.apple.com/us/app/twos/id1097350934)). The free/Plus split covers 78 features, many marked "currently free" or grandfathered ([Free vs Plus](https://www.twosapp.com/docs/reference/plus-matrix)). It runs on iOS, Android, the web, Mac, Windows, Linux, Apple Watch and Wear OS. Twos was founded in 2021 by Parker Klein and Joe Steilberg and had 25k+ active users in Dec 2024 ([TechCrunch](https://techcrunch.com/2024/12/05/twos-to-do-app-adds-a-feature-that-automatically-adds-a-suggested-actions-against-a-task)). The homepage claims "11K+ writing daily".

## What users say

Ratings are 4.8 from 398 ratings on the App Store and 4.9 from 117 reviews on Product Hunt.

**Love**
- **Fast capture in one place.** "It replaced 5 apps for me" (homepage testimonial). MakerStack: the daily-list model "removes organizing friction" ([MakerStack, 2026-07-20](https://makerstack.co/reviews/twos-review/)).
- **ADHD fit.** "With me having ADHD, I need something to keep track of things" and "the absolute best digital implementation of Bullet Journaling" ([Product Hunt](https://www.producthunt.com/products/twos/reviews)). Zapier picked it as the best ADHD to-do app for daily tasks and notes ([Zapier](https://zapier.com/blog/adhd-to-do-list/)).
- **Rollover and moving tasks between dates**, sync across devices, and developers who respond quickly ([App Store reviews](https://apps.apple.com/us/app/twos-quick-notes-lists/id1097350934?see-all=reviews&platform=iphone)).

**Dislike**
- **Too many settings and tiers.** It is "confusing to find what a subscription offers over the free version", and the settings are "a bit daunting" (App Store, Oct and Nov 2026). The settings reference does list well over a hundred options. "Learning curve" is the only con tag on Product Hunt.
- **No help narrowing down.** One reviewer captured a long list and struggled to reduce it to a few focus items. The developer's answer was to make a separate "top priorities" list by hand (App Store, Oct 2026).
- **Unwanted resurfacing.** Reminders about old, minor notes that were never to-dos. These come from the daily "remember" nudge, which can be turned off (App Store, Nov 2026).
- **Rollover gaps.** The rollover prompt sometimes needed a reload and didn't run after a skipped day (older Product Hunt comment; the docs confirm the skipped-day behaviour).
- **Limits.** No real collaboration, databases or rich formatting, and the AI (PALs) is experimental (MakerStack).

**YouTube.** Reviews exist from Be Productive ("Twos App Review | Smartest Note Taking App?", [video](https://www.youtube.com/watch?v=OBKWXKYjltM)), Knowledge Work Nexus ([video](https://www.youtube.com/watch?v=iOupaVnZEsw)) and Keep Productive, which the founders reacted to ([video](https://www.youtube.com/watch?v=iGzYc3bKnzw)). I couldn't get transcripts, so only titles and channels were checked and nothing in this note relies on what the videos say.

## Ideas for Roji

### Worth taking

1. **Capture to Today by default, decide the type later.** No choice is needed at capture. A line is a plain thing until it is clearly a to-do. This fits quick-add, the share target and MCP.
2. **Note-or-to-do suggestion as a one-tap question** ("Add checkbox / Leave as note"), with an opt-in to apply it automatically. This matches Roji's "deterministic rules plus model" direction: a verb rule first, a model second. It is cheap, and the user stays in control.
3. **Rollover as a move, with a counter.** The item stays the same object, with the same ID and history, and gets `carriedOverCount`. Roji can use that counter where Twos doesn't: as a signal for the backlog limit, for example to suggest "park, cancel or break down" after N carries, or to archive automatically as Sunsama does.
4. **No red, no overdue badge.** Keep the alert separate from the task. A missed alert doesn't fire again unless the user asked for that.
5. **Carry over by position, not by type.** Notes indented under a to-do travel with it, and day-level notes stay as the record of that day.
6. **Routines reset rather than roll over.** A recurring checklist resets on a schedule, runs on the next open, and missed resets don't stack.
7. **Cleanup as a view, not an edit.** Sorting and grouping never move data. Use tag-like filters instead of folders.

### Avoid or improve on

- **Unbounded rollover.** Twos lets a to-do carry over forever, and Today grows. Roji needs the short suggested Today and the backlog limit that Twos leaves out.
- **Rollover only when the app opens, and only from the last day used.** Roji has a server, so it can roll over on a schedule and catch up on skipped days.
- **A very long settings surface and a confusing set of paid tiers.** Roji should have few settings and sensible defaults.
- **Nudges about notes the user never meant as tasks.** Anything that resurfaces should be opt-in and limited to items the user marked as tasks.

## Sources

- [twosapp.com](https://www.twosapp.com/) (homepage with the to-do suggestion demo)
- Twos docs: [Things and lists](https://twosapp.com/docs/start/things-and-lists), [What is Twos](https://twosapp.com/docs/start/what-is-twos), [Today](https://twosapp.com/docs/days/today), [Carry-over](https://twosapp.com/docs/days/carry-over), [Thing types](https://twosapp.com/docs/things/types), [Capture bar](https://twosapp.com/docs/write/capture-bar), [Ramble](https://twosapp.com/docs/write/ramble), [Overdue reminders](https://twosapp.com/docs/reminders/overdue), [Auto reset](https://twosapp.com/docs/lists/auto-reset), [Move and copy](https://twosapp.com/docs/things/move-and-copy), [Sublists](https://twosapp.com/docs/lists/sublists), [Dynamic lists](https://twosapp.com/docs/lists/dynamic-lists), [Linking](https://twosapp.com/docs/lists/linking), [Sorting](https://twosapp.com/docs/lists/sorting), [Auto-sort](https://twosapp.com/docs/lists/auto-sort), [Completed](https://twosapp.com/docs/lists/completed), [Archive](https://twosapp.com/docs/lists/archive), [Review a list](https://twosapp.com/docs/lists/review-a-list), [Focus timer](https://twosapp.com/docs/things/focus-timer), [Switching](https://twosapp.com/docs/start/switching), [Settings A-Z](https://www.twosapp.com/docs/reference/settings), [Free vs Plus](https://www.twosapp.com/docs/reference/plus-matrix), [Glossary](https://www.twosapp.com/docs/reference/glossary)
- [App Store listing](https://apps.apple.com/us/app/twos/id1097350934) and [reviews](https://apps.apple.com/us/app/twos-quick-notes-lists/id1097350934?see-all=reviews&platform=iphone)
- [Product Hunt reviews](https://www.producthunt.com/products/twos/reviews), [Product Hunt PALs](https://www.producthunt.com/p/twos/twos-pals)
- [TechCrunch, 2024-12-05](https://techcrunch.com/2024/12/05/twos-to-do-app-adds-a-feature-that-automatically-adds-a-suggested-actions-against-a-task)
- [MakerStack review, 2026-07-20](https://makerstack.co/reviews/twos-review/), [Zapier ADHD to-do apps](https://zapier.com/blog/adhd-to-do-list/) (secondary sources)
- The Google Play listing couldn't be read (the page was truncated), and Reddit had no relevant threads.
