# Deep dive: Sunsama

Research for [#46](https://github.com/piemkay/roji/issues/46), part of the MVP spec map [#43](https://github.com/piemkay/roji/issues/43). Sources were read on 2026-11-24.

## Question

How does Sunsama work in detail: the daily planning ritual, task intake from email and other tools, workload limits, time tracking, shutdown and review, and AI features? What do users love and dislike, including what the daily ritual costs them? Which ideas fit Roji, which wants no manual weekly ritual?

## Short answer

Sunsama is a paid desktop-first daily planner ($17/month billed yearly, $22 billed monthly, 14-day trial, no free tier). It is built around two guided rituals: plan the morning, shut down in the evening. You pull tasks from your inbox and other tools into today by hand, estimate each one, and a workload counter warns you when the day is too full. Unfinished tasks roll over at midnight and are archived automatically after about four days. Since 2026 an opt-in "agentic" planner (Autoplan) drafts the day for you to review and undo, which shortens the ritual without removing it. Users praise the calm, the overcommitment warning and the single view across all their tools. Their complaints are the price, a mobile app that is only a companion, and a ritual that becomes a chore. That chore is the main reason people give for quitting.

For Roji, the parts worth taking are the workload warning, automatic rollover with auto-archive, the shutdown time as the day's capacity, a personal context that doesn't count toward workload, and Autoplan's "draft, review, undo" model. The manual pulling and estimating ritual, and the weekly planning and review, are what Roji wants to avoid.

## How it works

### Daily planning ritual (legacy flow)

The guided flow has five steps ([Legacy Daily Planning](https://help.sunsama.com/docs/usage-guides/daily-planning/legacy); [feature page](https://www.sunsama.com/features/daily-planning-and-shutdown)):

1. **Reflect on yesterday.** If you skipped the shutdown, mark yesterday's leftover tasks done or edit them.
2. **Add tasks.** Import calendar events as tasks, then pull tasks from integrations, the backlog and weekly objectives, or create new ones.
3. **Check predicted workload.** Planned times for work tasks are summed and compared with your workload threshold. A timeline shows when you would finish compared with your shutdown time. Overflow is deferred (`D`) or sent to the backlog (`Z`).
4. **Finalize.** Order the tasks, optionally timebox them on the calendar, and adjust the shutdown time.
5. **Share.** Note obstacles and optionally post the plan to Slack or Teams, then click "Get started".

Sunsama says the ritual "usually" takes 10 to 15 minutes ([llms.txt](https://www.sunsama.com/llms.txt)). The legacy help page suggests that new users plan about 5.5 hours of work, well under their working hours.

### Agentic daily planning (Autoplan, beta)

Autoplan is on by default in new workspaces and opt-in for existing ones ([Daily Planning](https://help.sunsama.com/docs/usage-guides/daily-planning)).

- **Setup:** connect the tools the agent may read (GitHub, Jira, Linear, Asana, Gmail and others), describe your work, and give each weekday a theme (for example "Deep work" on Wednesday).
- **Each day:** write a rough braindump of your focus and obstacles, then click "Plan my day". Planning takes 2 to 4 minutes.
- **Review:** a change list shows each created or moved task. "Judgement calls" list the decisions the agent was unsure of, each of which you keep or discard. You can undo one action, undo the whole run, or re-plan. Sunny, the assistant, writes a briefing that explains the order and the items it left out.
- **Rules:** standing instructions such as "Never move my 1:1s". Phrases like "from now on" in a braindump become learned rules.
- **Late in the day:** within two hours of shutdown it asks whether to extend today or plan the next workday.
- **Calendar:** solo focus blocks are opened up for tasks, while meetings stay blocked.

### Task intake

- **Email (Gmail, Outlook):** you browse an inbox or label in a side panel. An email becomes a task when you drag it onto a day or the calendar, or choose "Add to today", "Pick a day" or "Add to backlog". The task takes the subject line as its title. Nothing detects tasks in email automatically: Sunny can answer "what still needs a reply?" when asked, and Autoplan reads Gmail while drafting. Archiving, starring and labelling from Sunsama are applied in Gmail. Completion automations, such as archiving the thread when the task is done, must be set up by hand ([Gmail integration](https://help.sunsama.com/docs/integrations/gmail)).
- **Other tools:** more than 20 integrations, including Asana, ClickUp, GitHub, Linear, Jira, Todoist, Trello, Notion, Slack and Teams, plus Zapier. Calendars include Google, Outlook, Fastmail, Nextcloud and CalDAV ([homepage](https://www.sunsama.com/)). Completing a task syncs back to its project tool ([ADHD page](https://sunsama.com/for-adhd)).
- **MCP:** Sunsama is also an MCP server, so Claude, ChatGPT, Cursor and Claude Code can read and write it ([homepage](https://www.sunsama.com/)).
- **Mobile:** the iOS app is a companion for quick capture and works only after you have planned your first day on desktop ([App Store](https://apps.apple.com/us/app/id1475755747?l=en-us)).

### Workload limits, rollover, archive

- **Workload counter:** sums planned time on tasks that aren't in personal channels. It turns yellow as you approach the threshold set in account settings and red when you exceed it ([Planned and Actual Times](https://help.sunsama.com/docs/usage-guides/tasks/planned-and-actual-times)).
- **Personal context:** tasks in a personal context "do not count towards your workload". Contexts (#work, #personal) filter the view, and a channel can be linked to a calendar ([Channels and Contexts](https://help.sunsama.com/docs/usage-guides/channels-and-contexts)).
- **Rollover:** unfinished tasks "automatically roll over to the next day's task list if they are left incomplete at midnight" ([Task rollover](https://help.sunsama.com/docs/getting-started/basics/task-rollover-and-recurring-tasks-the-basics)).
- **Auto-archive:** a task that keeps rolling over for more than 4 days is archived. The threshold can be set from 1 to 14 days or turned off. Tracking time on a task or completing one of its subtasks resets the count. Archived tasks stay searchable, and a badge flags newly archived ones ([Archive](https://help.sunsama.com/docs/usage-guides/archive)).

### Time tracking and focus

- **Planned and actual time:** each task has planned time (shortcut `W`, or `~30m` while typing) and actual time (shortcut `E`, or the timer). Sunsama "can suggest a planned time based on similar tasks". A setting can count planned time as actual when you complete a task with no tracked time ([Planned and Actual Times](https://help.sunsama.com/docs/usage-guides/tasks/planned-and-actual-times)).
- **Timer:** starting it opens Focus Mode by default. Two parallel timers split time evenly while they overlap.
- **Focus Mode:** shows only the current task with a timer, a Pomodoro tab, and the calendar on hover. About 5 minutes before the next meeting or session it asks "Starts soon. Shift focus?" Completing a task moves focus to the next one ([Focus Mode](https://help.sunsama.com/docs/usage-guides/focus-mode)).
- **Focus Bar:** a small bar you can place anywhere on the desktop, showing the current task with play, pause and complete buttons. It needs the desktop app running ([Focus Bar](https://help.sunsama.com/docs/usage-guides/focus-bar)).

### Shutdown and review

- **Daily shutdown:** open it with `O`. It is not available in phone-sized browser windows. It has two steps ([Shutdown and highlights](https://help.sunsama.com/docs/usage-guides/daily-highlights)):
  1. "Today in review" shows total time, what you worked on and what you didn't finish.
  2. Sunsama drafts highlights with AI summaries for you to edit, then you click "Done for the day".

  You can add an emoji and a "Today was…" reflection, or custom prompts. The shutdown time defaults to 5:00 PM. Shutdown doesn't move unfinished tasks; they still roll over at midnight.
- **Weekly planning:** set weekly objectives, carry over unfinished ones, check whether the planned work fits the week ("What remains"), then edit an AI-drafted summary and share it ([Weekly Planning and Review](https://help.sunsama.com/docs/usage-guides/weekly-objectives/weekly-planning)).
- **Weekly review:** shows time per objective, a daily productivity chart and time by channel, then an AI-drafted summary of what you finished. The journaling and summary step can be switched off.

### AI features

| Feature | What it does | Source |
| --- | --- | --- |
| Autoplan (beta) | Drafts the day from your tools, calendar, rules and braindump. Every change is listed and can be undone. | [Daily Planning](https://help.sunsama.com/docs/usage-guides/daily-planning) |
| Sunny | Chat assistant that creates, moves and completes tasks, timeboxes, manages objectives, tracks time and summarizes work. It reads connected tools but can't change anything in them. There are no per-tool opt-outs; you have to disconnect the tool. | [Sunny](https://help.sunsama.com/docs/ai/sunny) |
| Channel recommendations | Suggests a channel for new tasks. Opt-in, and unavailable when a default channel is set. | [Channels and Contexts](https://help.sunsama.com/docs/usage-guides/channels-and-contexts) |
| Time suggestions | Suggests planned time from similar tasks. | [Planned and Actual Times](https://help.sunsama.com/docs/usage-guides/tasks/planned-and-actual-times) |
| Highlight summaries | AI summaries in the shutdown and the weekly review. Can be turned off. | [Shutdown](https://help.sunsama.com/docs/usage-guides/daily-highlights) |
| Agent Threads (beta, Mac) | Hand a task to Claude, ChatGPT, Cursor or Grok running on your Mac. The conversation stays attached to the task, and an amber dot shows when the agent needs you. | [Agent Threads](https://help.sunsama.com/docs/ai/agent-threads) |
| MCP server | Use Sunsama from other AI apps. | [MCP](https://help.sunsama.com/docs/ai/mcp) |

The docs don't say which model provider Sunsama uses.

## What users say

Most review sites are vendor-adjacent: ClickUp, Morgen, Saner and Lifestack sell competing products. The first-party signals are the App Store and Trustpilot. YouTube reviews exist (for example [R4OHJWPwsVM](https://www.youtube.com/watch?v=R4OHJWPwsVM), [GlkDGoHE2ek](https://www.youtube.com/watch?v=GlkDGoHE2ek), [wMdRF3cDEhk](https://www.youtube.com/watch?v=wMdRF3cDEhk)), but their transcripts could not be fetched, so they are not used as evidence here. One video description calls it the reviewer's most expensive productivity tool, at $192 a year.

**What they love**

- **The overcommitment warning.** Asian Efficiency calls it "the killer feature" ([review](https://www.asianefficiency.com/schedule-management/sunsama-review/)).
- **Calm and relief.** "Stress levels" felt "halved"; it is "a perfect fit" for ADHD; "undone tasks are bounced to the next day or a backlog" ([App Store](https://apps.apple.com/us/app/id1475755747?l=en-us), 4.5 stars from 530 ratings in the US).
- **One place for everything.** "This app pulls it ALL into one place that you look at right at the start of the day"; it works as a "universal inbox" ([Trustpilot](https://ca.trustpilot.com/review/sunsama.com), 4 reviews, so a weak signal).
- **Calm design.** No gamification, and the guided ritual makes it harder to skip planning on a busy day (Asian Efficiency).

**What they dislike**

- **Price.** Reviewers say "expensive, but very much worth it" and "the subscription price is high for me" (App Store, Trustpilot). There is no free tier.
- **The ritual becomes a chore.** Review roundups citing Reddit say users abandon it because "the ritual felt like a chore rather than a relief". Most people who quit say they stopped doing the morning ritual, not that the tool failed. Estimates put planning at 10 to 20 minutes plus 5 to 10 for the shutdown ([lifestack](https://lifestack.ai/blog/sunsama-review), [saner](https://blog.saner.ai/sunsama-reviews); both secondhand). One ADHD site warns: if you don't have 30 minutes a day for the ritual, Sunsama will frustrate you ([neurolaunch](https://neurolaunch.com/sunsama-review/)).
- **Mobile.** The app is "underfeatured", you "can't see more than one day at a time", and it is described as a companion rather than a standalone app (App Store).
- **Planning by hand.** Users asked for AI to "build and organize your weekly work plan" (App Store, 2025), and some reviewers found the rule-based guidance a drawback. Autoplan is Sunsama's answer to this.

**Cost of the ritual, in short:** the official figure is 10 to 15 minutes a day; reviewers report 5 to 30 minutes for planning plus up to 10 for the shutdown, every working day. The value depends on keeping the habit, and losing the habit is the most common reason for quitting. That cost lands hardest on an ADHD brain on a bad day, which is when the plan is needed most.

## Fit for Roji

**Take**

- **Workload warning against capacity.** Sum the estimates on the Today list, compare them with the time left before the end of the day minus meetings (read-only calendar), and say "too full" calmly. Roji can make the suggestion fit the capacity instead of asking the user to cut tasks.
- **Rollover plus auto-archive.** Tasks carry over automatically, and after N days of rolling over they leave Today on their own and stay searchable. Activity on a task (time logged, a subtask done) resets the count. This works against an ever-growing backlog without asking for a review.
- **The end of the day as a fixed boundary.** Use it as capacity for Today and as the moment to ask "plan today or tomorrow?" late in the day.
- **Personal context outside the work budget.** Roji covers both work and private life, so the budget should only count what belongs to the current context.
- **Draft, review, undo (from Autoplan).** The suggested Today is a draft with a visible change list, "judgement calls" for uncertain moves, undo for one action or all of them, and learned rules ("from now on…"). This fits deterministic rules plus a model.
- **Time suggestions from similar tasks.** Estimates appear without the user typing them, and actual time from the focus timer improves them.
- **Focus view details.** A "starts soon, shift focus?" nudge before the next meeting, and completing a task moves to the next one.
- **Auto-drafted highlights.** The day's log can fill itself in from the timer data (Roji's retroactive time log) with no ritual attached.
- **Agent Threads** show one way to attach an agent's work to a task, with a status dot when it needs you. This is relevant to agent preparation.

**Avoid**

- **The mandatory multi-step morning ritual**, and weekly planning and review built on objectives. This is exactly the friction Roji removes, and it is the main reason users quit.
- **Manual email import.** Sunsama doesn't detect tasks in email; Roji plans to.
- **A desktop-first app with a companion mobile app.** Roji's installed web app has to be first-class on the phone.
- **Writing timeboxes to the calendar.** Roji's calendar is read-only.
- **All-or-nothing AI privacy.** Sunny reads every connected tool, and the only opt-out is disconnecting the tool. Roji wants per-area opt-outs.

**Open questions for later tickets**

- Should Roji auto-archive at a fixed number of days, or let the suggestion decay stale tasks gradually?
- Who sets the capacity, the user (Sunsama's threshold) or Roji from the calendar and history?
- Does Roji need a one-tap "accept today's draft" moment at all, or does the Today list simply appear?
