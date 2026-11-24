# Deep dive: Akiflow

Research for [#66](https://github.com/piemkay/roji/issues/66), part of the MVP spec map [#43](https://github.com/piemkay/roji/issues/43). Sources were read on 2026-11-24.

## Question

How does Akiflow pull tasks from email, Slack and other tools into one inbox, how does its MCP connector work, and how does daily planning work? What do users love and dislike, and which ideas fit Roji's capture and tasks-from-email?

## Short answer

Akiflow is a paid, desktop-first time-blocking planner ($34/month, or $19/month billed yearly; 7-day trial with a card; no free plan). Every new task, typed or imported, lands in one **Inbox** that is kept apart from the schedule. Imports are explicit signals, not detection: you star (or label) an email in Gmail, save a message in Slack, or a task is assigned to you in a project tool, and it shows up as a task that links back to the source. Undoing the signal (unstar, unsave) completes the task. As of November 2026 Akiflow still **does not detect requests in email**: a Gmail "inbox monitor" is in beta for meeting requests and travel bookings only, and staff point users to starring or to Claude over MCP as the workaround.

The MCP connector is a hosted remote server (`https://mcp.akiflow.com/mcp`) with OAuth 2.1, included in every paid plan, covering tasks, projects, calendar events, time slots and meeting transcripts. Daily planning is a four-step ritual (recap, check to-dos, choose focus, set a shutdown), plus an optional shutdown and weekly rituals; the AI assistant Aki can propose a schedule for approval. Users love the consolidation and the time blocking; they dislike the price, billing surprises, mobile bugs and an AI push some did not ask for.

For Roji: take the single inbox with source links, "signal in the source tool = capture, undo the signal = done", work/personal tagging by email domain, the hosted OAuth MCP server with a published capability and limitation list, and draft-for-approval planning. Akiflow's gap (no detection of requests in email) is exactly what Roji plans to fill.

## How it works

### The Inbox

All new, unplanned tasks land in the Inbox, separate from the schedule ([Your Inbox](https://product.akiflow.com/articles/5284502-your-inbox)). They arrive from the "Add new task" field, the Command Bar (`Cmd/Ctrl+E`, natural language such as "Call John"), integrations, Aki (text, voice, WhatsApp) and the Meeting Assistant, whose action items appear under "Suggested from your meetings" with `+` to accept and `X` to dismiss.

Processing is GTD-style and manual: do it now if it takes under 2 minutes (`E` marks done), plan it (`P`, or drag to a calendar slot, a date, or a range like "This Week"), or send it to Someday (`Cmd/Ctrl+S`). The Inbox can be sorted (newest, oldest, priority, deadline, integration, project) and grouped by source or project, with counts and total duration per group. The help article describes no automatic sorting of the Inbox by AI.

### Email

- **Gmail:** starring an email imports it as a task; optionally a dedicated "Akiflow" label (auto-created) is used instead of the star. Old starred mail is not imported. Starring a thread imports only its latest email; later replies come in as separate tasks. `O` on a task opens the email in Gmail ([Gmail](https://product.akiflow.com/articles/0294053-gmail)).
- **Two-way completion:** per account, "On mark as done" can unstar/unlabel, open Gmail, do nothing, or ask every time. Removing the star or label in Gmail marks the task done. Deleting the task does not touch the email.
- **Work and Personal:** each account is tagged from its domain ("If your account is something like gmail.com or yahoo.com, we mark it as Personal"; company domains become Work), editable in Settings, and used by Aki to understand context.
- **Outlook email** converts emails to tasks as well ([integrations](https://akiflow.com/integrations)).
- **Forward to Aki (beta since Aug 2025):** send an email to `aki@akiflow.com` from an enabled account, with Aki in the To field (CC/BCC is ignored); Aki creates a task or event from things like flight confirmations and invitations ([changelog](https://product.akiflow.com/changelog/loveletter-send-aki-an-email-to-create-tasks-beta)).
- **No detection of requests yet.** A feature request for an assistant that reads email and suggests tasks (25 votes, "Building Now") has staff replies saying a Gmail inbox monitor is in beta for meeting requests and travel bookings, but "doesn't yet spot requests in your emails and turn them into to-dos" (Nov 2); they suggest starring or connecting Gmail and Akiflow to Claude via MCP ([feature board](https://akiflow.featurebase.app/p/akiflow-assistant-that-reads-emails-and-whatsapp-messages-to-suggest)).

### Slack and other tools

- **Slack:** messages you save ("Add to saved") or add to Slack's To-dos become Inbox tasks, from connection onwards only; a message shortcut "Create a task" works too. Unsaving in Slack marks the task done; completing in Akiflow does not unsave. Akiflow says it holds only the content of messages you save or send to it ([Slack](https://product.akiflow.com/en/help/articles/5772525-slack)).
- **Others:** Microsoft Teams, Notion, Asana, ClickUp, Trello, Todoist, Microsoft To Do, Jira, Linear, GitHub; calendars Google, Outlook, iCloud (explicitly two-way); Zoom; Zapier and IFTTT for everything else ([integrations](https://akiflow.com/integrations)). The site does not mention a browser extension or public API.

### MCP connector

From the official setup article ([Akiflow MCP](https://product.akiflow.com/help/articles/4302815-akiflow-mcp); [landing page](https://akiflow.com/mcp)):

- **Endpoint and auth:** hosted remote server at `https://mcp.akiflow.com/mcp`, OAuth 2.1; the client "can only reach Akiflow data your account can reach". Included in all paid plans and the trial.
- **Clients:** Claude (connector directory or custom connector), Claude Code (`claude mcp add --transport http akiflow https://mcp.akiflow.com/mcp`), ChatGPT and Codex (plugin), Cursor, Devin Desktop; stdio-only clients via `mcp-remote`.
- **Scope:** the landing page says 19 actions. Tasks (create into Inbox/Someday/planned, list inbox/someday/overdue/high-priority, update, plan, reschedule, complete, recurring, subtasks), organisation (folders, projects, sections, tags), files on tasks (10 per task, 25 MB each), calendar events (create, edit, cancel, find time with up to 10 people, RSVP), time slots, read-only meeting transcripts, and opt-in sharing of "Akiflow Memory" (off by default).
- **Documented limitations:** cannot list all tasks of a project, merge or reorder, edit a whole recurring series, or read PDF/Word content; schedule reads are capped at about 90 days per request.
- Marketing examples: pull action items from meeting notes into the calendar, review priorities against free time, block focus time.

### Daily planning

- **Rituals** ([Rituals](https://akiflow.featurebase.app/en/help/articles/0805246-rituals)): *Daily Planning* is recap yesterday, check Inbox / This Week / Month, choose today's focus and defer the rest, add a shutdown reminder to the calendar. *Daily Shutdown* reviews completed tasks, rates the day and optionally plans tomorrow. *Weekly Planning* and *Weekly Shutdown* add goals, notes, stats and time slots. A skipped shutdown becomes the first step of the next planning. Daily Planning can open automatically (Settings > Rituals).
- **Time blocking** is the core: tasks are dragged onto the calendar or into reusable *Time Slots* (blocks tied to a project).
- **Overdue:** tasks are not flagged overdue right after their end time, to avoid a wall of overdue items; overdue tasks sit at the top of Today; "Replan Undone Tasks" suggests the next slot of the same series or project ([search summary of the Today Page and Overdue help articles](https://product.akiflow.com/articles/0741055-today-page)). A changelog notes the automation that moved overdue tasks to the next day at shutdown was removed.
- **Aki** (beta since March 2025) is a chat assistant in the app, by voice, WhatsApp and email. "What's my plan for today?" shows the schedule, flags unplanned tasks and offers to place them. The AI calendar page promises "Akiflow proposes a complete schedule that fits your real calendar. You review it, approve it" and "Nothing happens without your permission" ([AI calendar](https://akiflow.com/ai-calendar-copilot); [Aki](https://akiflow.com/aki)). Users can create automations such as an 8am day overview.

## What users say

First-party signals: App Store 4.3 from 206 ratings (US, iPhone; [App Store](https://apps.apple.com/us/app/akiflow-time-blocking-calendar/id1621279084)). A Carly blog post (which sells a competing scheduling assistant, so treat with care) reports Trustpilot 4.4, Google Play 3.7 from 339 reviews, and quotes r/Akiflow ([usecarly](https://www.usecarly.com/blog/akiflow-review/)). YouTube reviews exist, for example GBRLIFE's "I gave Akiflow a 3.5 out of 5" ([5j2_eQoZ-1Q](https://www.youtube.com/watch?v=5j2_eQoZ-1Q)) and Healthnews' 2025 review ([9XrY4mFzxfA](https://www.youtube.com/watch?v=9XrY4mFzxfA), affiliate-coded); transcripts could not be fetched, so only their descriptions are used. GBRLIFE's summary: good for integrations, a clean interface and rituals, but it "doesn't integrate with absolutely everything", automation "isn't as deep as some people want", and "there is a learning curve".

**What they love**

- **Everything in one place.** "AkiFlow is the first app that has every feature I need to task manage my life, all in one place" (App Store, ADHD reviewer); tasks from "Asana, Gmail, and Todoist" together (Trustpilot via Carly).
- **Time blocking**, called the best available in several Capterra reviews.
- **Easy to learn for ADHD**, and good support ("customer service is exceptionally good", App Store).
- **Aki** feels like "talking to a person" (App Store via Carly).

**What they dislike**

- **Price** ("very pricey. Way more than any other calendar app") and **billing surprises** (an annual charge "without receiving any clear reminder email", Trustpilot via Carly).
- **Reliability and mobile:** features "stop working" until a force quit, the day view blanks after swiping (App Store); mobile "laggy and glitchy", bugs "with email syncing and default reminders duplicating", drag-to-calendar removed on mobile (via Carly).
- **AI push and caps:** an r/Akiflow user complains the team is "prioritizing 'AKI' the AI that many users do not want AT ALL"; another hit an unpublished message limit (via Carly). An App Store reviewer says AI assigns tasks to old projects.
- **Fit:** it suits people who time-block; reviewers flag it as wrong for anyone who doesn't plan that way. Big tasks can't be split across days.

## Fit for Roji

**Take**

- **One Inbox, kept apart from Today**, fed by every capture channel, each task linking back to its source (`O` opens the email or message). Group by source with counts.
- **Capture by a signal in the source tool**, and complete by undoing it (unstar, unsave), with a per-account "on done" setting. For Roji's email this is a cheap, deterministic fallback next to detection: a star or label always becomes a task.
- **Thread handling:** a thread produces one task, and new replies need a rule (Akiflow makes a new task per reply; Roji may prefer to update the existing task).
- **Work vs personal from the email domain**, editable. This gives Roji's areas and model-privacy rules (work tasks may go to a cloud LLM) a sensible default.
- **Suggestions with accept or dismiss** (Meeting Assistant's "Suggested from your meetings"): the right shape for tasks detected in email, kept out of the real list until accepted.
- **Hosted MCP with OAuth and a written list of what agents can and cannot do.** Inbox/Someday/planned targets for task creation, and opt-in sharing of preferences ("memory"), are good defaults for Roji's MCP from day one.
- **Lenient overdue:** don't flag overdue immediately; replan undone tasks into the next suitable slot.
- **Proposal, then approval** for any AI-made plan.

**Avoid**

- **Manual time blocking and rituals** as the core. Akiflow's planning is a daily and weekly ritual plus dragging blocks; Roji wants a short suggested Today with no review rituals and no calendar writes.
- **Gmail-centric capture with no real detection.** Users have asked for it for a year; Roji should treat detection as a first-class feature.
- **Pushing AI chat as the product** and hidden AI usage caps; users notice and resent it.
- **A weak mobile app.** Roji's installed web app must be first-class.
- **Opaque billing** (less relevant for a self-hosted solo trial).

**Open questions for later tickets**

- Should a starred or labelled email become a task directly, or a suggestion like detected ones?
- When an email thread gets a reply, does Roji update the task, add a note, or create a new task?
- Should "done" in Roji write back to the mailbox (unstar, archive)? That needs write access to mail, which a read-only design avoids.
