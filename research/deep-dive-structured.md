# Deep dive: Structured

Research for issue #68 on the Roji MVP map (#43). Gathered 2026-11-24.

**Question:** How does Structured combine an inbox with a visual day timeline, how does Replan
handle missed tasks, and what makes it stick with ADHD users? What do users love and dislike,
and which ideas fit Roji's Today suggestion and motivation?

## Short answer

Structured (unorderly GmbH, Germany) is a **timeline-first day planner**. Each task has a start
time, a duration, an icon and a color. Tasks and imported calendar events sit together on one
vertical timeline. Anything without a date or time goes to a separate **Inbox**. You plan by
moving Inbox items onto a free slot, with drag and drop or by setting a date and time. Unfinished
tasks **stay where they were**: Structured never rolls them forward on its own.
**Replan** (Pro, Apple only, since May 2025) is a manual review. It shows each past unfinished
task as a card and offers four swipes: reschedule, Inbox, done, delete. It can prompt you in the
morning or the evening, and after three moves a badge shows how often a task has been pushed.

ADHD users say it sticks because of the **visible time**. The day becomes a picture of blocks
and gaps, which helps with time blindness. Other reasons they give: the calm, playful look
(icons, colors), the satisfaction of checking blocks off, an Inbox for "not now" thoughts, and a
free tier that is useful (for most of its history). Their main complaints: features locked
behind Pro (recurring tasks most of all), no automatic roll-over (the most hotly argued request),
an Inbox that gets hard to manage past about 20 items, no two-way calendar sync, sync and Mac
bugs, and having to pick a date and time before a task feels "real".

For Roji, worth taking: a **past-only, one-card-at-a-time review with four outcomes**, the
**"moved N times" signal**, an **energy budget** as a lighter alternative to a time budget, a
**draft that you accept or discard** for AI-created tasks, and Structured's choice to ship an
**MCP server**. Worth avoiding: making the timeline the place where planning happens, and an
Inbox that grows without anything keeping it down.

## How it works

### Timeline plus Inbox

- The site pitches it as "your all in one day planner" that puts tasks and to-dos "into one
  visual timeline". You can import your calendar and switch to week and month views
  ([structured.app][home]).
- The App Store describes the timeline as holding "tasks, calendar events, and routines" with
  drag-and-drop scheduling. The free tier includes the timeline, the Inbox, subtasks with notes,
  a Pomodoro focus timer, widgets, Live Activities and a dyslexia-friendly font. Pro adds
  calendar and Reminders connections, recurring tasks and routines, AI planning, Replan, custom
  notifications and timer intervals ([App Store][appstore]).
- The Inbox is for "any task without a specific time or date". On the phone it is a tab of its
  own; on Mac, iPad and the web it opens as a split view next to the timeline. Inbox tasks still
  have a title, an icon and an **expected duration** ([Help: Inbox][help-inbox]).
- Moving a task to the timeline: tap "+", set a date and time and confirm. On Apple devices you
  can also drag it onto the timeline. Moving it back works by dragging or "Move to Inbox". To keep
  a reusable template in the Inbox, you duplicate it and schedule the copy
  ([Help: Inbox][help-inbox], [Help: drag and drop][help-dnd]).
- The intended flow, in the team's words: capture to-dos in the Inbox "until you have time for
  them", then check them off there or move them into a free slot. The getting-started post says
  nothing about ADHD ([Blog: get started][blog-start]).
- The Inbox has no sorting, filtering or grouping, and no way to hide completed tasks
  ([Help: Inbox][help-inbox]). "Deadlines in Inbox" (1.5k votes, planned) and "Inbox Sort /
  Filter / Folders" (1.3k) are among the top requests ([feedback board][feedback]).

### Replan: missed tasks

- Shipped in Structured 4.2 on 27 May 2025, for iOS and macOS only ([changelog][changelog-42]).
  It "allows you to revisit your unfinished tasks and decide on how to handle them"
  ([blog][blog-replan]).
- **Scope:** only tasks in the past. Recurring tasks, all-day tasks and imported calendar events
  are excluded by default. You can include the first two, and you can set imported events to be
  auto-completed when Replan starts. Requires Pro; not on Android or the web
  ([Help: Replan][help-replan]).
- **Trigger:** long-press a day and choose "Replan x Tasks", or turn on a scheduled prompt
  (Morning, Evening or a chosen time). Prompts are **off by default**. They appear as a bar
  above the tab bar, with an optional push notification ([Help: Replan][help-replan]).
- **Flow:** tasks are shown one at a time as a card at the bottom of the screen. Each card offers:
  - Reschedule (swipe up). Opens today's or tomorrow's timeline depending on the time of day,
    and you drag the task into a slot.
  - Move to Inbox (swipe left): "Out of sight, but not out of mind."
  - Check off (swipe right).
  - Delete (swipe down): "Out of sight, out of mind."
  - You can also skip a task or undo the last action ([Help: Replan][help-replan]).
- **Moved-count badge:** "After you reschedule a task at least 3 times in Replan, a badge appears
  next to it showing how often it has been moved" ([Help: Replan][help-replan]).
- **Other routes:** ask Structured AI to "Push all unfinished tasks into tomorrow's schedule,
  except for the buying presents task". It returns a proposal you accept. You can also drag tasks
  by hand, or run an Apple Shortcut that pushes today's unfinished tasks into the Inbox
  ([Help: reschedule][help-resched]).
- **Why it's manual:** the request "(Auto-)Move Incomplete Tasks" has 2.5k votes and 112 comments
  and is now marked completed by Replan. The comments split. One ADHD user calls the lack of
  auto-move "the BIGGEST flaw in this app", because "someone with ADHD may not remember to move
  tasks". Another called missed tasks that "never appear anywhere" "a bug not a feature". Others
  chose Structured *because* it "DOESN'T automatically roll over" tasks. They warn that rolling
  over routines would flood the Inbox ("100 instances of 'Read'") and "feel like a bug" after a
  holiday or illness. Several ask for a per-task setting such as "move to inbox if uncompleted"
  ([feedback: auto-move][feedback-automove]).

### Structured AI and MCP

- Structured AI turns typed text (up to 2,000 characters), speech or a photo of a paper planner
  into tasks. You get **suggestions to review**: Accept All, Edit, Add, Discard per item, or
  change the instructions and Regenerate. It can also edit existing tasks ([Help: AI tasks][help-ai-create]).
- It runs on OpenAI. Input "may be stored for up to 30 days on our and OpenAI's servers", and
  Structured says it does not use your input to train the model. You can hide AI on Apple devices
  (Settings > Advanced > Hide AI). Requires Pro; not on the web ([Help: Structured AI][help-ai]).
- Users still find it limited. A 2-star review: "you can ask a question and the conversation is
  over as soon as it responds". Another: "Ai ruined the app" ([App Store reviews][rss]).
- Structured runs an **MCP server** (`mcp.structured.app/mcp`). It lets ChatGPT, Claude,
  Cursor, Gemini CLI, OpenCode and others "read, manage, and edit your tasks", and needs a
  Structured Cloud account ([Help: MCP][help-mcp]). Reviewers mention it: "I synced it with
  ChatGPT … and it scheduled everything for me" ([App Store reviews][rss]).

### Energy Monitor

- Built with "researchers and medical professionals at the Dutch institute Breinstraat". Off by
  default. Each task gets an energy level: relaxing, neutral, or low/medium/high draining. Points
  are counted roughly per 30 minutes, so a high task costs 3 points per half hour
  ([Help: Energy Monitor][help-energy]).
- The default daily budget is 20 points. The help article suggests 36–42 for an average person
  and 18–22 for someone with limited energy, "e.g., due to illness". The meter is green while you
  are under the budget, orange near it and red over it. When it is red, the suggestion is to
  delete or reschedule tasks ([Help: Energy Monitor][help-energy]).
- Relaxing tasks earn energy back, but only in the middle of the day, not right after waking or
  before bed. Inbox tasks and all-day tasks can't be given an energy level
  ([Help: Energy Monitor][help-energy]).

### Price and scale

- The site claims 15M+ downloads and 500K+ Pro users ([structured.app][home]). The US App Store
  shows 4.8★ from 169K ratings and Editors' Choice ([App Store][appstore]).
- Pro is sold monthly, yearly or as a one-time lifetime purchase; the store lists prices from
  $2.99–7.99 a month, $9.99–49.99 a year and $99.99 lifetime, depending on tier and region
  ([App Store][appstore]).

## What users love and dislike

I read the 150 most recent US App Store reviews from the official review feed, dated
September–November 2026. They split 81× 5★, 26× 4★, 10× 3★, 5× 2★ and 28× 1★
([App Store reviews][rss]). I also read the feedback board ([feedback][feedback]) and the
descriptions of YouTube reviews. YouTube serves no transcripts to scripts, so the videos
themselves weren't analysed.

**Loved:**

- **Seeing time.** "It really helps with time blindness and mapping out my day". "Great for
  seeing the big picture view of my week and day … where I have gaps". The Spanish-language
  review says the same: seeing the hours from waking to sleep makes plans "more realistic"
  ([reviews][rss]).
- **Satisfying progress.** "There's something very satisfying and motivating about being able to
  see the accomplishments through the day and make real progress in your backlog". "Just enough
  whimsy and structure to keep me engaged and checking off my day" ([reviews][rss]). One 4★
  review asks for a fireworks-style completion animation like closing the Apple Watch rings.
- **The Inbox as a dump.** "The Inbox is probably what made it click for me. I can just th[row]…"
  ([reviews][rss]).
- **Evening planning to calm the head.** "I am able to sit down each night and organize all of my
  thoughts and tasks for the next day and go to bed without my head spinning" ([reviews][rss]).
- **Energy Monitor** for an "AuDHD & demand avoidant brain": "thoughtful of which tasks are
  draining or boosting for me" ([reviews][rss]).
- **Long-term retention.** Several reviewers have used it for 2–4.5 years: "i've tried tons of
  scheduling apps but i always end up deleting them … had this one for over 4.5 years"
  ([reviews][rss]). An older ADHD review: "probably spen[t] hundreds of $$ on planners just to use
  them for a few weeks" ([App Store][appstore]).
- **Free tier and lifetime option.** "You can genuinely use it without paying … there's a
  one-time purchase option, which is incredibly refreshing" ([reviews][rss]).
- The ADHD-focused YouTube channel AdhDONE (June 2025) praises it for helping "visualize time,
  think about your energy, and build productivity without feeling like you're failing, which
  other productivity apps tend to do" ([YouTube: AdhDONE][yt-adhdone]).

**Disliked:**

- **Paywall, especially recurring tasks.** This is the largest group of 1★ reviews: "A daily
  planner is near useless if you can't make the schedule repeat without paying … This app claims
  to be geared towards those who struggle with ADHD". One paying subscriber says repeating tasks
  was recently "locked behind an additional $30 paywall". Some lifetime buyers say they lost
  access after an OS update ([reviews][rss]).
- **Missed tasks vanish.** Without Pro or a scheduled Replan prompt, missed tasks just sit on
  past days ([feedback: auto-move][feedback-automove]).
- **The Inbox doesn't scale.** A 1★ review titled "I LOVE This App, but the Inbox Is HORRIBLY
  PAINFUL": with ~20 items, dragging to reorder stops at the screen edge and there is "no way to
  filter or sort … I'm starting to wonder if the Inbox is actually helping me … or just giving me
  another frustrating list that I have to manage". Another asks for categories in the Inbox
  ([reviews][rss]).
- **Too much detail up front.** "It seems like I have to pick a date and a time … it's just too
  much at one time like I wish I could do something simple and then add the level of detail"
  (3★). Another: "It takes too long to put tasks at the right times. Dragging a task in 15 minute
  increments is just not the right way". And "autofilled values that destroy user agency"
  ([reviews][rss]).
- **Calendar is one-way.** "Two-Way Calendar Sync" is the second most-voted request (3.2k)
  ([feedback][feedback]). A lifetime buyer left for another app over it ([reviews][rss]). Users
  also ask for travel time and for fixed events that don't move ([App Store][appstore]).
- **Reliability.** Tasks that disappeared, sync failures across devices, a slow and buggy Mac
  app, battery drain, alerts that stopped after a week, and no search (1.7k votes)
  ([reviews][rss], [feedback][feedback]).
- **Feature creep.** "An app that should be simple, buried in an awful UX". "The app seems like
  for kids" ([reviews][rss]).

## Ideas that fit Roji

**For the Today suggestion:**

1. **Review missed tasks, past only, with four outcomes.** Replan's set (do later today or
   tomorrow, back to the pool, done, drop) is small and complete. For Roji the system should
   *propose* the outcome on each card and let the user confirm with one swipe, instead of making
   the user decide each card from scratch. Undo is a must.
2. **Moved-count badge.** "Moved 3×" is a cheap, honest signal. Roji can use it as input to
   automatic sorting (suggest a smaller step, Someday, or dropping the task) instead of showing it
   as a guilt badge.
3. **Recurring tasks don't pile up.** Structured excludes routines from Replan by default, and
   users warn about "100 instances of 'Read'". Roji should never carry a missed recurring task
   forward; only the next one matters.
4. **Neither side of the roll-over fight is happy.** Some users need missed tasks to reappear;
   others resent auto-rescheduling after a holiday or illness. Roji's Today suggestion should
   rebuild each day from the pool, not move yesterday's list forward. That way nothing is lost
   and nothing floods in. Consider a "welcome back" mode after days of absence.
5. **Energy budget next to the time budget.** Low, medium or high drain plus a restorative type,
   shown as a green/orange/red meter, is a lighter way to say "this day is too heavy" than
   counting minutes. It fits the overbooked-day warning from the Sunsama dive.
6. **Duration on Inbox items** from the moment of capture lets a suggestion fit tasks into free
   time without placing them on a timeline. Roji could estimate it instead of asking.
7. **AI output as a draft.** Accept All, Edit, Discard per item and Regenerate is the same
   draft-review-undo pattern the Sunsama dive found.

**For motivation and calm:**

- Visible progress through the day (checked blocks, a filling day) is what reviewers call
  "satisfying". Roji can show it without a timeline: a short list that visibly empties.
- Icons and colors per task make the app feel friendly rather than businesslike. Structured picks
  icons automatically ("Smart Task Icons"), which costs the user nothing.
- Evening planning "to go to bed without my head spinning" suggests an optional, light evening
  capture, not a required ritual.

**Avoid:**

- Making the timeline the main way to plan. Picking a date, time and duration before a task
  "counts" is the friction users name, and Roji wants to remove it.
- An Inbox with nothing keeping it small. Structured's Inbox turns into "another frustrating list
  to manage". Roji's automatic sorting and backlog limits are the answer, and this is the best
  evidence for them.
- Locking basic, high-frequency things like recurring tasks behind a paywall, and changing what a
  paid tier includes later. Users read that as betrayal.

**Architecture note:** a mainstream consumer planner now ships a hosted MCP server, and users
already use it to let ChatGPT fill their day. This supports Roji's MCP-from-day-one choice.

## Open questions

- Does Replan's moved-count badge lead people to drop tasks, or only to feel bad? The docs don't
  say.
- How does Structured AI decide where in the day to put a task? The help pages only describe
  input and review.
- Energy Monitor adoption is unknown; it is off by default and appears in few reviews.

## Sources

[home]: https://structured.app/
[appstore]: https://apps.apple.com/us/app/structured-daily-planner/id1499198946
[rss]: https://itunes.apple.com/us/rss/customerreviews/page=1/id=1499198946/sortby=mostrecent/json
[help-inbox]: https://help.structured.app/en/articles/338178
[help-dnd]: https://help.structured.app/en/articles/338306
[help-replan]: https://help.structured.app/en/articles/4511874
[help-resched]: https://help.structured.app/en/articles/1990594
[help-ai]: https://help.structured.app/en/articles/1782402
[help-ai-create]: https://help.structured.app/en/articles/331074
[help-mcp]: https://help.structured.app/en/articles/9872962
[help-energy]: https://help.structured.app/en/articles/998530
[blog-replan]: https://structured.app/blog/replan
[blog-start]: https://structured.app/blog/getstarted
[changelog-42]: https://feedback.structured.app/changelog/structured-42-replan
[feedback]: https://feedback.structured.app/
[feedback-automove]: https://feedback.structured.app/p/auto-move-uncompleted-tasks
[yt-adhdone]: https://www.youtube.com/watch?v=SFz0FH5u0ok

- Official site: [structured.app][home]
- Help center: [Inbox][help-inbox], [drag and drop][help-dnd], [Replan][help-replan],
  [rescheduling unfinished tasks][help-resched], [Structured AI][help-ai],
  [creating tasks with AI][help-ai-create], [MCP server][help-mcp],
  [Energy Monitor][help-energy]
- Blog and changelog: [Replan post (2 Jun 2025)][blog-replan],
  [get started (12 Jun 2023)][blog-start], [4.2 changelog (27 May 2025)][changelog-42]
- Feedback board: [overview][feedback], [(Auto-)Move Incomplete Tasks][feedback-automove]
- App Store: [listing][appstore], [recent reviews feed][rss] (pages 1–3, 150 reviews)
- YouTube: [AdhDONE, "ADHD-Friendly App Review: Structured" (June 2025)][yt-adhdone]; description
  only, no transcript available
