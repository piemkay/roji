# Deep dive: Lunatask

Research for [#65](https://github.com/piemkay/roji/issues/65), part of the MVP spec map [#43](https://github.com/piemkay/roji/issues/43). Sources were read on 2026-11-24.

## Question

How does Lunatask keep its backlog small without manual review (task aging, hiding not-yet-relevant tasks, prioritisation), how does it capture (including forwarded email), and how do areas and goals work? What do users love and dislike, and which ideas fit Roji's backlog and Today suggestion?

## Short answer

Lunatask is an end-to-end encrypted desktop and mobile app (Windows, macOS, Linux, iOS, Android; no web app yet) that combines tasks, habits, journal, mood tracking, notes and a contacts tracker. It is built by a team of two in Prague and Berlin, and is free for 2 areas, with Premium at $6/month billed yearly or $8 monthly, or $300 lifetime ([pricing](https://lunatask.app/pricing); [FAQ](https://lunatask.app/docs/common-questions)). A rewrite, "Lunatask 3", is announced and is meant to bring a web app.

**It does not keep the backlog small. It keeps the visible list small.** Tasks are split into a short "Now" (current focus) and a "Later" pile that is sorted automatically by status, priority and **age**, and can be collapsed. Older tasks rise, not sink: age is a reason to do a task, not to drop it. Future tasks are hidden until a date, then pop up in a "just came up" area to be re-sorted by hand. The docs still tell you to review months-old tasks and delete them ([long lists](https://lunatask.app/docs/how-to/handle-long-lists)). Nothing is archived or decayed automatically, and there is no AI.

Capture is plain: a quick-add, one forwarding email address per area (subject becomes the title, body the note), a bookmarklet, Siri/Shortcuts, Zapier and a create-only API with de-duplication by `source`/`source_id`. Areas are separate contexts (Work, Personal) that never mix; goals are progress-tracking folders inside an area.

For Roji, the strongest ideas are Now/Later instead of dated Today lists (nothing breaks at midnight), a visible age in days, a deterministic sort key (status, priority, age), defer-until-date with a "just came up" moment, a work-in-progress warning and the API's idempotent `source_id`. Roji has to go further on the backlog itself: Lunatask shows that sorting alone still leaves a long pile that the user must prune.

## How it works

### Now / Later and the automatic sort

- **Now and Later.** Tasks sit in two buckets: "Now" is your current focus, not tied to a date, and "Later" holds candidates, "automatically sorted by how long it's been waiting" and other details. New tasks land in Later by default; an area setting can change that ([tasks overview](https://lunatask.app/docs/features/tasks/overview); [workflows](https://lunatask.app/docs/features/tasks/workflows)).
- **Sort key.** In every workflow, "automatic prioritization … based on age, status, priority, and other properties … always works behind the scenes". The Eisenhower workflow sorts by quadrant, then priority, then age. The top task is shown in bold as the next one ([workflows](https://lunatask.app/docs/features/tasks/workflows)).
- **Priority** is an integer from -2 to 2 (default 0), raised with Shift+Arrow Up ([task entity](https://lunatask.app/api/tasks-api/entity)).
- **Age is visible.** Since v1.6 every task shows its age in days next to its name, so the sort "shine[s] through" ([1.6.0](https://lunatask.app/releases/1.6.0)). The FAQ says the number "serve[s] an important psychological role" ([FAQ](https://lunatask.app/docs/common-questions)).
- **Age can be reset** by hand, for example when an old task is moved to Now; the release note says it is "not meant to be used all the time" ([1.6.5](https://lunatask.app/releases/1.6.5)). Scheduling a task also resets its age, because a reminder only becomes a real to-do on its date ([scheduling](https://lunatask.app/docs/features/tasks/scheduling/date-based-scheduling)).
- **Nothing happens at midnight.** The scheduling guide compares the list to a Kanban board: "nothing changes … at midnight", so there is no rescheduling and "no feeling of failure when you don't finish everything in 'Today'". It tells users not to schedule tasks for today and not to use dates for planning ([scheduling](https://lunatask.app/docs/features/tasks/scheduling/date-based-scheduling)).

### Workflows (per area, switchable at any time)

| Workflow | Sections | Notes |
| --- | --- | --- |
| Priority List | one list | "Next to work on" shows only its top 10 |
| Now / Later | Now, Later | default; WIP limit counts Now |
| Kanban | In progress, Next, Waiting, Later | WIP limit counts In progress; nudges finishing before starting |
| Must / Should / Want (Premium) | Must, Should, Want, Later | a MoSCoW variant on the Now part |
| Eisenhower Matrix (Premium) | Do now, Delegate, Do later, Avoid | sorted by quadrant, priority, age |
| Plan Your Days | Today, Tomorrow, days…, Later | the only date-based workflow; discouraged |

Switching a workflow loses nothing, so you can triage in one and work in another ([workflows](https://lunatask.app/docs/features/tasks/workflows)). A "Plan Ahead" workflow (Today, This Week, This Month, This Year, Later) is announced. One workflow applies to a whole area; goals can't have their own ([FAQ](https://lunatask.app/docs/common-questions)).

### Hiding what isn't relevant yet

- **Defer to a date.** A future task is set to Later and drops to the bottom. On its date it appears in a **"just came up"** area above the list with a red badge, and you move it into your focus, back to Later, or defer it again ([scheduling](https://lunatask.app/docs/features/tasks/scheduling/date-based-scheduling)). Dates before tomorrow are disabled for deferring because they break the sort ([2.1.28](https://lunatask.app/releases/2.1.28)).
- **Collapse sections.** Click "Later" or "Waiting" to hide them ([long lists](https://lunatask.app/docs/how-to/handle-long-lists)).
- **Goals that show only actionable tasks.** A goal such as "Backlog" or "Roadmap" can be set to show only its non-Later tasks in the area list, so its Later tasks appear nowhere else ([long lists](https://lunatask.app/docs/how-to/handle-long-lists)). Archived goals hide their open tasks ([2.1.2](https://lunatask.app/releases/2.1.2)).
- **"Next to work on" overview.** One cross-area screen of current tasks with a "done today" section; areas such as a reading list can be excluded ([1.7.6](https://lunatask.app/releases/1.7.6), [2.0.19](https://lunatask.app/releases/2.0.19), [2.1.7](https://lunatask.app/releases/2.1.7)).
- **Work-in-progress limit.** A message appears when Now (or In progress) holds too many tasks; you can still add more ([WIP limit](https://lunatask.app/docs/features/tasks/work-in-progress-limit)).

### What is still manual

The "How to handle long lists" guide says: if a task has sat untouched for months, "consider deleting it … if something is important, it will come back one way or another", or move it to an "Icebox" area or goal. It also accepts that "if you still have 80 relevant tasks in your Later section, then you have 80 tasks in your backlog … and that's okay" ([long lists](https://lunatask.app/docs/how-to/handle-long-lists)). Moving tasks from Later to Now, and re-sorting "just came up" tasks, is also by hand. The app's own developer has 948 open tasks across four areas ([structure areas](https://lunatask.app/docs/how-to/structure-areas)).

### Capture

- **Quick add:** the A key, Cmd/Ctrl+N, or a button per section ([workflows](https://lunatask.app/docs/features/tasks/workflows)).
- **Email forwarding:** each area can generate a unique address. Sending, forwarding, CC or BCC to it creates a task: subject as name, body (HTML stripped to supported formatting) as the note, and the first link extracted. The suggested pattern for routing is a separate "Inbox" area. Mail passes through SendGrid, where it is stored temporarily, and is encrypted on arrival ([email integration](https://lunatask.app/docs/features/integrations/email-integration); [2.0.16](https://lunatask.app/releases/2.0.16)). There is no parsing of dates, priority or area from the email, and no detection of tasks inside mail.
- **Browser bookmarklet** sends page title and URL to the API (blocked on sites with strict CSP), plus community extensions ([browser](https://lunatask.app/docs/features/integrations/browser-extension)).
- **Others:** Siri, Apple Shortcuts, Tasker, a widget, Zapier, community Alfred/Raycast tools, CLIs and MCP servers ([tasks overview](https://lunatask.app/docs/features/tasks/overview)).
- **API:** `POST /v1/tasks` accepts area, goal, name, note, status, motivation, Eisenhower quadrant, estimate, priority and date. If an open task in the same area already has the same `source`/`source_id`, it returns 204 and creates no duplicate ([create task](https://lunatask.app/api/tasks-api/create)). Because data is end-to-end encrypted, integrations can only bring tasks in, never sync them out.
- **Quick Note:** a synced Markdown scratchpad beside the list ([quick note](https://lunatask.app/docs/features/quick-note)).

### Areas and goals

- **Areas of life** are contexts, not categories: "If two tasks compete for the same portion of your time or attention, they likely belong in the same area." The guide warns against making Chores, Finance and Health separate areas, because the sort can't compare tasks across areas. The motivating story: in apps that mix work and personal tasks in one Today view, they paid the phone bill first thing at the office ([structure areas](https://lunatask.app/docs/how-to/structure-areas)).
- **Goals** live inside an area. They are "basically just nested lists, folders, or categories" with a progress bar computed from done and open tasks, an optional due date (highlighted when within 7 days) and an archive. The progress bar can be hidden for folder-like goals such as "Chores" or "Backlog" ([tasks overview](https://lunatask.app/docs/features/tasks/overview); release notes).

### Time

- **Estimates** show a sum per section; very large estimates are highlighted in red as a nudge to split the task ([estimation](https://lunatask.app/docs/features/tasks/estimation)).
- **Timers** are per task, 25 minutes by default (F key), to get started; starting one marks the task In progress. They sync across devices, and on mobile the timer can go full-screen ([timers](https://lunatask.app/docs/features/tasks/timers)).
- **Time blocking** is optional: drag tasks onto a day calendar that shows synced external calendars (Premium). Completing a task removes its future blocks ([time blocking](https://lunatask.app/docs/features/tasks/scheduling/time-blocking)).

## What users say

Independent reviews are thin. Product Hunt has no reviews, Reddit couldn't be read, and YouTube reviews exist ([CkyJJ2V_7AM](https://www.youtube.com/watch?v=CkyJJ2V_7AM), [ppYoG2dgIIM](https://www.youtube.com/watch?v=ppYoG2dgIIM), [JbkRGof7wtM](https://www.youtube.com/watch?v=JbkRGof7wtM)) but their descriptions and transcripts couldn't be fetched, so they are not used as evidence. The App Store listing is the main first-party signal: 4.4 stars from 103 US ratings ([App Store](https://apps.apple.com/us/app/lunatask-a-better-to-do-list/id1583719331?see-all=reviews&platform=iphone)).

**What they love**

- **Prioritisation done for them.** "The task prioritization options are phenomenal" (App Store). A testimonial on the vendor's site: "the only task and time management tool that keeps me on track and requires almost none of my time to manage" ([ADHD page](https://lunatask.app/adhd); curated, so weak evidence).
- **ADHD fit and all-in-one.** "Life changing for ADHD … replacing Things and Notion in my life almost entirely"; "It works like I think, in Areas and Goals" (App Store).
- **Privacy:** end-to-end encryption is a recurring reason to choose it.

**What they dislike**

- **No cross-area view of priorities.** "I'd give it 5 stars if it offered a way to pin or somehow prioritize tasks across areas"; weak filtering and search (App Store).
- **Feels disorganised / too much.** "A little bit too disorganized for an app that I would rely on" (App Store). Journal and habits can't be turned off by product decision ([FAQ](https://lunatask.app/docs/common-questions)).
- **Platform gaps, admitted by the vendor:** no web app, a 1024×720 minimum window, no scrollbars on Windows and Linux, Linux freezes and Wayland issues, notifications need Google Play Services, no sharing with a partner because of encryption ([FAQ](https://lunatask.app/docs/common-questions)). Secondary reviews name the mobile app as the most common complaint ([toolguide](https://toolguide.io/en/tool/lunatask/), secondary).
- **Hidden features.** "The best button is no button" makes collapsing sections and similar actions hard to discover; the vendor concedes this ([FAQ](https://lunatask.app/docs/common-questions)).
- **Learning the method.** The docs warn that using dates as in other apps leads to "problems and confusion", and say understanding scheduling is "absolutely essential". The model needs reading before it works.

## Fit for Roji

**Take**

- **Now instead of Today.** Roji's suggested Today can behave like Now: it survives midnight unchanged, so unfinished tasks are neither failures nor rescheduled. The suggestion is refreshed, not rebuilt.
- **A deterministic sort key** for the backlog and as input to the Today suggestion: status, then priority (-2..2), then age. It works without a model and is easy to explain.
- **Age in days, visible.** Cheap, honest and explains the order. Roji can use age both ways: rising for tasks that matter, decaying for tasks that keep being skipped.
- **Defer until a date, then "just came up".** Hides not-yet-relevant tasks without a review; age starts when the task becomes real. Roji could let the suggestion place a resurfaced task automatically instead of asking the user to re-sort it.
- **Areas as contexts, not categories,** with the "competes for the same time" rule, and work and personal never mixed in one view. Matches the map's work and private split and the per-area model opt-outs.
- **Goals as folders with optional progress and due date,** including "only actionable tasks shown" for backlog-type goals.
- **Work-in-progress warning** on Now, as a calm message rather than a block.
- **Idempotent capture:** `source` plus `source_id` with "no duplicate while an open task exists" suits tasks from email and MCP.
- **Large estimates flagged** as a hint to split the task, and the one-tap 25-minute starter timer for the focus view.

**Avoid**

- **Sort-only backlog control.** Lunatask's answer to an ever-growing Later is "review and delete" or "80 tasks is okay". Roji needs an automatic step on top: archive or fade tasks that stay at the bottom, or that are skipped again and again, with undo and search.
- **Six workflows to choose from** and a method that must be learned from long docs. Roji should have one way that works without reading.
- **Email forwarding as the email path.** It is dumb (subject to title) and is out of scope for Roji anyway; Roji plans detection in mail.
- **Manual re-sorting of resurfaced tasks** and manual moves from Later to Now; these are the review rituals Roji wants to remove.
- **Bundled journal, habits and mood** that can't be turned off.
- **Desktop-first with platform gaps;** Roji's web app must work at phone width.

**Open questions for later tickets**

- Should age raise a task (Lunatask) or eventually retire it (Sunsama's auto-archive), and how do the two combine?
- Is Roji's Today a dated list or a Now set that carries over, with the suggestion only topping it up?
- Does Roji need explicit priority at all, or only "pin" plus signals from email, deadlines and age?
- How are tasks compared across areas when the user asks for one list (the top App Store complaint)?
