# Deep dive: Quest (quest.app)

Research for [#45](https://github.com/piemkay/roji/issues/45) on the map [#43](https://github.com/piemkay/roji/issues/43). Researched 2026-11-24.

## Question

How does Quest work in detail: capture, the focus view and timer, the retroactive time log of when you worked on what, day planning, motivation, AI features, platforms? What do users love and dislike, and which ideas fit Roji's destination?

## Short answer

Quest is a one-person, opinionated daily planner from German YouTuber Niklas Steenfatt, in free public beta since November 2026. You plan tasks by **day, not hour**, in an order. You work through them in a **Focus view** that shows one task, its notes and a stopwatch. Finishing a task moves you to the next one. Every timer run becomes a block in a **Calendar that logs work after the fact**: it shows what you did, not a plan. Blocks can be dragged, resized, reassigned and added by hand, but only in the past. A **Weekly Review** adds up "quest hours" (time on tasks tied to a long-term goal). Quest has no AI features, no MCP or API, no calendar integration yet (Google Calendar is the top-voted request), no inbox or undated tasks, and no Pomodoro. Its core loop (an ordered day, one task in focus, a stopwatch, a log calendar) matches the focus view and retroactive time log in Roji's scope. Its goal-hours framing, and its lack of capture and email, do not.

## Sources

Primary sources, most-trusted first:

- **[S1] Release notes**: signed releases on GitHub, `steenfatt/quest-releases`, v0.1.4 (2026-09-30) to v0.8.5 (2026-11-18). Read via `gh api repos/steenfatt/quest-releases/releases`. <https://github.com/steenfatt/quest-releases/releases>
- **[S2] Official site and FAQ**: <https://quest.app/>
- **[S3] Download page** (platforms, requirements): <https://quest.app/download/>
- **[S4] Public roadmap** (Featurebase, with vote counts): <https://feedback.quest.app/roadmap>
- **[S5] Feature-request board**: <https://feedback.quest.app>
- **[S6] Launch video by the maker**: "Der beste Task Manager aller Zeiten" (German, 12:48, 2026-11-17, about 161k views and 142 comments when read). <https://www.youtube.com/watch?v=aV1qdgYYYis>. Read from the auto-generated German transcript; timestamps are given below.
- **[S7] Comments on S6**: the top 150 comments, fetched with yt-dlp. This is a user-voice sample, not a primary source for how the app behaves.
- **[S8] Privacy policy** (hosting, processors): <https://quest.app/privacy/>

No independent YouTube reviews of Quest turned up; the app is about a week into public beta. It has no docs site; the release notes serve as its documentation. I did not use Quest hands-on.

## What Quest is and why it exists

- Its tagline is "Make time for what matters". A **quest** is a long-term goal; you split it into daily tasks and track time against it [S2].
- In S6 the maker says years of **time-blocking** led to burnout. A day packed with 30-minute blocks fell apart whenever something came up, and AI rescheduling tools (he names Reclaim and Motion) only repeated the "micro-failures" [S6 1:20-2:48]. Quest is his replacement: **plan by day, order the tasks, focus on one at a time, measure the time** [S6 2:57, 9:53].
- Before Quest he used Sunsama (closest to his vision, but "too many features"), a Notion goal template, Toggl for time tracking, with the "billable" flag misused to mark quest hours, and a weekly Grow dashboard [S6 10:16].
- He calls the app "software as a medium": his philosophy built into a product. He turns down many feature requests on purpose, citing a request for several timers at once as an example [S6 11:01-12:02].

The six principles he gives, which he says work in any tool [S6 description]:

1. Only one task visible
2. Measure time
3. A done list instead of a to-do list
4. Reflection as a ritual (a weekly review)
5. Input goals (hours put in) instead of output goals
6. Vertical coherence (life goal, then year, month, week, day and hour line up)

## Capture

There is little of it, and it all happens inside the app:

- `N` or the command menu (`Cmd/Ctrl+K`) creates a task. **Quick Add** assigns a quest when you type `#quest` at either end of the title, with suggestions [S1 v0.3.1, v0.4.3, v0.8.3]. A Top/Bottom setting decides where new tasks land [S1 v0.1.7, v0.1.12].
- Every task has a date. **Undated tasks (an inbox or backlog)**, a **braindump section** and a **global shortcut to add a task from anywhere** are all only *planned* on the roadmap (47, 20 and 19 votes) [S4]. The braindump item names AI as a possible "someday" feature [S4].
- There is no email capture, share sheet, MCP or API. One commenter criticised the missing MCP server, and another asked for one [S7].
- A commenter asked how he handles things that come up during the day; no answer was captured [S7].

## Day planning

- **Home** shows today. **Plan** is a horizontal board with one column per day; you drag tasks between days and order them inside a day. You can choose from five column widths, and there is a "jump to Today" button [S1 v0.8.3, S6 3:11].
- Each day is split into **Quest work** (tasks tied to a quest, colour-coded) and **Other** (everything else). The maker keeps "Other" collapsed, out of sight, and does it in the evening [S1 v0.1.13, v0.4.3; S6 8:28-8:54].
- **No time slots and no time blocks.** "Plan your tasks by day, not by hour, put them in an order, and only ever focus on one" [S6 9:53]. Tasks you don't finish roll over to the next day automatically; the release notes mention a "daily rollover" [S1 v0.4.0].
- The site's advice: put your most important quest task into the day "before the small things fill the day" [S2].
- Recurring tasks have daily, weekly, monthly and yearly rules, custom intervals, end dates and timezones. You can edit one occurrence or the series, and since 0.8.4 you can pin them [S1 v0.2.0, v0.8.4].
- Rich-text notes on tasks and quests [S1 v0.1.13, v0.8.3]; filters by one or several quests [S1 v0.1.13, v0.8.3].
- **No deadlines or priorities.** Users ask how hard deadlines (a tax filing, a sponsor date) avoid being buried in "Other" [S7, a top comment with 14 likes; S5 "Frist/Priorisierung"].

## The focus view and timer (in detail)

What it shows [S6 3:24-4:30, S2, S1]:

- **One task only**: the title, its notes and a timer. The maker calls this the first principle: seeing what else is waiting drains the energy to start the current task [S6 3:31-3:48].
- The timer is a **stopwatch that counts up, not a countdown**. You can pause and resume it [S6 3:51].
- **The focus view is a queue.** When you press complete (`Cmd/Ctrl+Enter`), the next task in the day's order appears: "it takes as long as it takes" [S6 4:12-4:31, S2].
- `F` opens the focus view; the back button sits top-left; a later release made leaving focus easier [S1 v0.3.1, v0.4.3; S4 "Easier way to leave Focus", completed].
- You can edit the title and formatted notes inside the focus view, with an expandable editor [S1 v0.8.3].
- **One timer at a time**, by design: the maker turned down parallel timers [S6 11:51].
- Starting a task moves it to the top of its list. It stays there when paused; switching tasks moves the next one to the top [S1 v0.1.7, v0.1.14].
- Since 0.8.3 you can start a timer from a Plan card without opening the focus view. Running timers show their quest's colour, and the timer and pause controls sit together [S1 v0.8.3; S4].
- On desktop, a setting decides whether closing the window keeps the timer running, stops it or cancels the close [S1 v0.8.3].
- Moving a running task to a future day stops its timer and keeps the time already worked on the day it happened [S1 v0.4.3].
- Not there yet: **Pomodoro** work and break intervals (planned, 67 votes, second-highest), break alerts, iOS Live Activity, a lock-screen timer [S4, S5]. One commenter wants an optional countdown because it keeps them from dawdling [S7]; another raises Parkinson's law as their main problem with the open-ended stopwatch [S7].

## The retroactive calendar: when you worked on what (in detail)

The maker: "the calendar becomes a little **logbook**. On the left I have my tasks; on the right, no time blocks, but the time actually worked" [S6 4:33-4:41]. It doubles as the **done list**: you look back at "the mountain you already climbed: reward, reward, reward", and it gives you data to analyse [S6 4:43-5:04].

How it behaves, from the release notes:

- **Every timer session becomes a block**, coloured by quest, in a week Calendar (a "This week" button is shared with Review). Home also shows today's blocks [S1 v0.1.13, S6 5:06].
- **Pauses are merged.** Pauses shorter than ten minutes in the same task join its sessions into one block, and the pause is not counted as tracked time [S1 v0.1.13, v0.2.0].
- **Correct after the fact:** drag to move or resize, but only within the past. Moving keeps the exact duration, and clicking a resize handle does not change the time [S1 v0.1.12].
- **Log work you forgot to time**, straight from the Calendar, either on a new completed task (dated to that work day) or on an existing task (whose schedule and completion stay unchanged). Date, time and quest are editable [S1 v0.4.0]. "Easier manual time entry" was the top completed roadmap item, with 18 votes [S4]. One user still asks for entering a time range without starting and stopping a timer [S5].
- **Reassign a block** to another task without changing its time, with Undo. If another device has changed the same work meanwhile, the newer data wins [S1 v0.4.0, v0.8.3].
- Same-day entries are edited with a single date and a time range; entries that span midnight get separate start and end dates. Times are handled correctly across daylight-saving changes [S1 v0.4.0, v0.4.3]. You can choose a 12- or 24-hour clock [S1 v0.3.1].
- Running time is counted the same way everywhere while the timer runs: task history, calendar totals, quest cards, and daily and weekly summaries [S1 v0.2.0].
- **The calendar is internal only.** It shows no outside appointments and writes nothing out. Google Calendar integration, to show appointments next to tasks and sync tracked time back, is the **top planned item (86 votes)** [S4]. CalDAV, Outlook and Apple calendar requests sit on the board [S5].

## Review and motivation

- **Weekly Review** (`R`) shows bars of tracked time per day, split into Quest work and Other, with a four-week moving average and the current week's daily average [S1 v0.1.9, v0.1.13, v0.2.0].
- **Quest hours**: tracked time on a task linked to a quest. Quest totals are lifetime hours [S2, S1 v0.3.1]. The idea is **input goals**: "this week I'll put N hours into this project" leaves few excuses, while an output goal you miss teaches you that missing is normal [S6 6:12-7:00].
- Quests can be **completed** and browsed later in a Completed view [S1 v0.3.1].
- The maker reviews the dashboard every week with an outside coach [S6 5:49-6:02].
- There are **no points, streaks or badges**. The rewards are the "satisfying tick" on completion, the done-list calendar and the growing hours [S2, S6]. Users asked for a completion sound [S5].
- Calm touches: a **Reduce motion** setting, which also respects the operating system's setting [S1 v0.3.1]; warm light and dark themes [S1 v0.1.12, v0.8.3]; interface scaling from 100% to 150% [S1 v0.1.14]; a keyboard-first design with a command menu [S2].

## AI features

**None in the product** [S2, S1 across all releases]. The only AI mentions are the braindump idea on the roadmap ("someday") [S4] and third-party processors: Featurebase summarises feedback with AI, and Sentry and PostHog may use AI to investigate errors [S8]. Users asked for MCP, a Claude connection and voice control with an on-device LLM [S5, S7]. The maker's stance is anti-automation for planning: AI rescheduling tools still produce constant re-planning failures [S6 2:20-2:36].

## Platforms, data and price

- Desktop apps for **macOS** (Apple Silicon, macOS 12+) and **Linux** (one-command installer for Ubuntu, Debian, Fedora, Arch and Omarchy; also DEB, RPM and AppImage). The **web app** is at web.quest.app. **iPhone** (TestFlight, iOS 16.4+) and **Android** (Google Play beta) are "companion" apps. **Windows** is browser-only for now [S3, S1 v0.8.4, v0.8.5].
- The full workspace is designed for large screens [S2].
- **Local-first**, with sync through PowerSync (EU) and Neon (Frankfurt); accounts run on Clerk (US). It works offline once offline files are cached; sign-in, sync and updates need a network [S2, S8].
- Session recording with PostHog (US) on desktop, kept 30 days; Sentry for errors [S8].
- Free during the beta, with pricing after the beta not announced [S2]. Closed source; "open source" is the top feature-board request (5 votes) [S5], and several commenters asked too [S7].
- English and German [S1 v0.4.0].

## What users love and dislike

Evidence is thin: one launch-video comment section [S7] and the vote board [S4, S5]. There are no store reviews or independent reviews yet.

**Loved**

- **Dropping time-blocking** for an ordered day and one task. "Much closer to how you work anyway, without the to-do mountains that cause stress"; "first feeling: I got more done than usual and am happy with myself" [S7].
- A **done list and logged time** instead of a growing list; useful for logging client time at work [S7].
- **Sunsama, Toggl and a goal framework in one place**, free [S7].
- One commenter found the hard-coded philosophy refreshing compared with yet another task manager [S7].

**Disliked or missing**

- **No calendar integration.** "Completely impractical for people with calls or fixed appointments"; people whose calendar others book can't plan by day alone [S7]. Google Calendar is the top roadmap item (86 votes) [S4].
- **No Pomodoro or countdown**; worries about Parkinson's law with an open-ended stopwatch [S4, S7].
- **No inbox or undated tasks** (47 votes), and no quick capture from outside the app [S4].
- **No deadlines or priorities**; non-quest obligations can get buried [S7, S5].
- **Mobile** came late and still has rough edges: sign-up and focus bugs reported at launch [S7, S1 v0.8.5].
- **No MCP, API or AI**; easy to copy ("vibe-coded it in 5 minutes"); closed source [S7].
- Sequential single-tasking doesn't suit people who run several coding agents at once [S7].

## Which ideas fit Roji's destination

Roji's scope (map #43) already includes "a focus view with a timer inside the web app, plus a retroactive time log (a calendar view of when you worked on what)", a read-only calendar and no time blocks. Quest is close to a reference implementation of that slice.

**Worth borrowing**

1. **A focus view that is also a queue**: one task, its notes, a count-up timer with pause, and *complete moves to the next task in Today's order*. That removes "what next?" without time slots.
2. **The log calendar as a done list**: blocks come from timer sessions, not plans, and are coloured by area. It rewards by looking back, which suits an ADHD brain better than staring at the mountain ahead.
3. **Tolerant logging rules**: merge pauses under ten minutes, edit only the past, keep durations when moving a block, reassign a block without changing its time, add a block by hand for work you forgot to time, and stop the timer when its task moves to another day. These rules belong in the domain model, and in `GLOSSARY.md` as something like *time entry* or *work block*.
4. **Plan by day, not by hour**, with automatic rollover. This fits "no weekly review rituals" if Roji does the rollover quietly.
5. **Hiding the non-essential**: Quest collapses "Other" so it isn't in your face. That is close to Roji's "short suggested Today list".
6. **Calm settings**: Reduce motion, one running timer, a keyboard-first design.
7. **A timer that survives closing**: Roji is a web app, so the timer has to be server-side or timestamp-based, so that closing a tab doesn't lose time. Quest had to add an explicit choice about this.

**Not a fit, or a trap**

- **Quest hours and input goals** assume a goal-setting, self-tracking user with a weekly review ritual and a coach. Roji explicitly avoids review rituals. A light "where did my time go" view could come later; a goal system should not drive the MVP.
- **Manual capture only, with every task dated**: Roji needs capture from anywhere, email detection, MCP and an inbox that sorts itself. These are Quest's biggest gaps, and Roji's difference.
- **No outside calendar**: Quest's top complaint. Roji's read-only calendar should show fixed appointments next to the log, and let free time shape Today (still unspecified on the map).
- **Stopwatch only**: users ask for Pomodoro or a countdown. Count-up is a reasonable MVP default; an optional soft time box could come later, but alarms and pressure work against calm.
- **Deadlines**: Quest has none. Roji's sorting will need some due-date signal (for example from email) so obligations don't sink.
