# Deep dive: Tiimo

Research for issue #47 on the Roji MVP map (#43). Gathered 2026-11-24.

**Question:** How does Tiimo work in detail as an ADHD-focused visual planner: timeline, focus
timers, routines, AI planning, motivation and calm design? What do users love and dislike,
and which ideas fit Roji?

## Short answer

Tiimo is a **visual day planner**: everything you plan becomes a colored block on a vertical
timeline. A countdown timer starts when each block begins, and widgets and Live Activities keep
"what's now, what's next" in view. Since 2025 an **AI Co-Planner** turns a typed or spoken brain
dump into tasks with steps, time estimates and priorities, and reshuffles the day when you
run late. The design is deliberately non-judgmental. The team says it dropped full auto-scheduling,
pushy nudges, guilt streaks and scoring. In practice the app still ships streaks and trophies.
Users love the calm visuals, the checklists inside tasks and the transition alerts. They dislike
bugs, unreliable notifications and sign-outs, rigid time editing, and the subscription price.

For Roji the transferable ideas are the "design for the freeze" principle, one suggestion
instead of a menu, rescheduling without guilt, the visible countdown with an early heads-up
before a block ends, and checklists inside a task. Roji should not copy the timeline-first model:
Tiimo asks you to place every task in time, which is the kind of planning ritual Roji wants to
remove.

## How it works

### Visual timeline

- Your schedule becomes "a visual timeline you can actually follow", with custom colors, icons
  and layouts ([tiimoapp.com][home]). The App Store describes a "color-coded daily overview with
  drag-and-drop tasks" ([App Store][appstore]).
- Tasks are either scheduled on the timeline or "Anytime"/To-do tasks without a time. You can
  start a timer on an unscheduled task from its play icon ([FAQ: focus timer][faq-focus]).
- It imports and syncs Apple, Google and Outlook calendars. The FAQ doesn't say whether sync is
  one-way or two-way ([FAQ][faq]). Apple Calendar and Reminders sync is listed in the App Store
  ([App Store][appstore]).

### Focus timer

- The timer "turns minutes into a visual countdown" (a progress ring plus a clock) to "ease
  transitions, and reduce overwhelm". It is framed as a support for time blindness, so you can
  switch "without relying on internal cues" ([Focus][focus]).
- It **starts automatically when a scheduled task begins**. You can switch tasks from the Focus
  tab while other timers keep running ([FAQ: focus timer][faq-focus]).
- Tasks can carry a checklist of steps, for example a laundry task with "sort" and "start wash"
  ([tiimoapp.com][home]).
- The countdown is mirrored in widgets, Live Activities, Dynamic Island and the Apple Watch, so the
  next step stays visible from the lock screen or inside other apps ([Widgets][widgets]).
  Sessions sync across phone, tablet and web ([Focus][focus]).
- A daily "Review Today" catches unfinished tasks so you can move them forward
  ([Focus][focus], [FAQ: focus timer][faq-focus]).

### Routines and reminders

- Routines (a morning routine, a work schedule) are sequences of blocks on the timeline, and each
  block can hold a checklist ([tiimoapp.com][home]). The site doesn't document templates or
  repeat rules in detail.
- Reminder timing and sounds are set per user under Settings → Notifications. The site's stated
  goal is reminders "that help without creating alert fatigue" ([FAQ][faq],
  [FAQ: notifications][faq-notif]).

### AI Co-Planner

- **Input:** type or speak anything, from "Messy list, racing thoughts, half-ideas" to "I'm late,
  move all my task by 10min" ([AI planning][ai]).
- **Output:** tasks broken into steps, "task-specific time estimates", urgency labels and
  placement in the day. When plans change it "reshuffles the day for you" ([AI planning][ai]).
- **Free vs Pro:** the free tier has "limited AI chats". Advanced AI task breakdown is a Pro
  feature ([App Store][appstore], [tiimoapp.com][home]).
- **Design intent**, from the Head of Design's write-up ([Co-planner design][codesign], Sep 2025):
  - "Design for the freeze": build for "the moments when your brain freezes, not when it flows".
  - When a task looks likely to cause a shutdown, it offers to break the task down or move it to
    a day with more capacity.
  - It suggests **one manageable task rather than a menu**, and asks about energy before
    building an ambitious list.
  - **Dropped full auto-scheduling** because it removed the user's sense of agency. It makes no
    assumptions about priorities or about what an ideal day looks like.
  - Explicitly avoided: pushy motivational nudges, guilt-inducing streaks, scoring, penalties for
    changing course, and "relentless cheerfulness".
  - The tone is kind without condescension, because "shame has never motivated sustainable
    change". The team prototyped conversations for exhaustion, scattered thinking and no input
    at all.
  - Planned: personalization that learns patterns without feeling invasive, and **local AI that
    works offline**.
- **Origin** ([AI planner story][story], Oct 2025): the user problems were tasks that feel too
  big, everything feeling equally urgent, trouble starting, and guilt when plans change. The
  stated principles are "accessibility over efficiency", "It's not about being more productive",
  and rescheduling "without judgment". Neither page names the model provider or explains how
  data is handled.
- Co-Planner was iOS-only in October 2025 ([AI planner story][story]). The App Store notes for
  v3.72.0 add a "more fun Co-Planner" ([App Store][appstore]).

### Motivation and wellbeing

- **Streaks** with "trophies and rewards" ([tiimoapp.com][home]). This conflicts with the
  co-planner team's stated rejection of guilt-inducing streaks ([Co-planner design][codesign]).
- **Mood log** on a slider from very unpleasant to very pleasant, with reflection and patterns
  over time, used "to adjust your routine to your energy" ([tiimoapp.com][home]).
- Personal insights and stats, neuroinclusive courses and a community hub; live focus sessions on
  YouTube ([App Store][appstore], [product][product]).

### Calm design and positioning

- "Built by and for neurodivergent people", co-designed with ADHD and autism experts, ad-free
  ([tiimoapp.com][home], [App Store][appstore]). It started as a research project for teens with
  ADHD ([AI planner story][story]).
- Recognition: iPhone App of the Year 2025, Editors' Choice, Apple Design Awards finalist 2024.
  Rated 4.6 from about 22K ratings, with 4M+ downloads claimed ([App Store][appstore]).
- Accessibility: VoiceOver, Voice Control, larger text, dark interface ([App Store][appstore]).

### Platforms and pricing

- iOS, iPadOS, watchOS, Android and web ([tiimoapp.com][home]). The web planner requires Pro
  ([Pricing][pricing]).
- The free tier is mobile only: basic planner, limited AI, one profile. Pro is billed monthly or
  yearly, the 7-day trial applies to yearly plans only, and prices vary by region
  ([Pricing][pricing], [FAQ][faq]). The US App Store lists in-app purchases from $7 to $54
  ([App Store][appstore]). There is no lifetime option.

## What users love and dislike

Sources: App Store reviews, aggregated on [JustUseApp][reviews] and [App Store][appstore]. Many of
the visible reviews date from 2021 to 2023, and some complaints (no Watch app, no widgets) have
since been fixed.

**Love**
- The visual design, colors and icons. "I love the interface and the colors."
- **Transition heads-ups**: "the heads up notifications that a specific routine or activity is
  about to end... is awesome."
- Checklists inside activities ("especially helpful for me when I need to clean").
- Predictability lowers anxiety: "I always know what is coming next which reduces anxiety."
- The widget and calendar sync "100% prevents me from forgetting stuff."
- A team that listens and ships requested features.

**Dislike**
- Bugs: "one step forward and two steps back". Planned activities that "disappear without any
  reason".
- **Reliability of the core loop**: sign-outs that silently stop notifications; a visual timer
  that is "pointless if screen goes dark"; a short notification "ding" that isn't enough to get
  attention.
- **Rigid time editing**: requests to enter a duration instead of an end time, for drag-and-drop
  that pushes later blocks, for no overlaps and for custom repeats (every 3 days).
- The day overview feels cluttered and chaotic.
- The subscription price, plus requests for a one-time purchase. Several YouTube review titles
  frame it as a "subscription trap" or "money pit".

**YouTube reviews.** A search found about ten creator reviews from 2025 and 2026, for example
["Hidden Issues No One Talks About"][yt-issues], ["Best Visual Planner for Every Neurotype"][yt-neuro],
["EXPOSED — Powerful Planner or Money Pit?"][yt-exposed] and ["Overhyped Subscription Trap"][yt-trap].
Their pages and transcripts could not be fetched, so the only usable signal is the titles, which
cluster around price and value for money rather than the design.

## Ideas that fit Roji

| Tiimo idea | Fit for Roji | Why |
|---|---|---|
| "Design for the freeze": support the stuck moment, not the planning moment | **Adopt as a principle** | Matches "removes friction". It gives the Today suggestion and the focus view one job: get the user from stuck to started. |
| One manageable suggestion instead of a menu | **Adopt** | Reinforces a short suggested Today list. Consider a single "start here" item. |
| No full auto-scheduling; the user keeps agency | **Adopt** | Consistent with Roji's calendar staying read-only and Today being a suggestion, not a plan. |
| Rescheduling without guilt; a daily review that carries unfinished work forward | **Adapt** | Roji wants no review rituals, so roll unfinished work forward automatically and silently instead of asking. |
| Visual countdown with a heads-up before the end | **Adopt in the focus view** | The most praised feature. Include a gentle warning before time runs out. |
| Checklists inside a task | **Adopt** | Cheap, loved, and a natural home for AI-generated steps (or for agent preparation later). |
| AI breakdown into steps with time estimates | **Adapt** | Useful, but offer it on demand for a task that looks big rather than on every capture. |
| Asking about energy before suggesting a list | **Consider** | A one-tap energy level could shape the Today suggestion. Weigh it against "no rituals". |
| Brain dump to tasks, plus "I'm late, shift everything" | **Adapt** | Fits quick-add and MCP capture. Shifting a timeline matters less without time blocks. |
| Timeline-first day (every task placed in time) | **Avoid** | It is a planning ritual, and users report rigid time editing and a cluttered day. |
| Streaks, trophies, scores | **Avoid** | Tiimo's own design team calls them guilt-inducing. |
| Mood log and insights | **Defer** | Not core to an MVP. The retroactive time log already covers "when did I work on what". |
| Local or offline AI on the roadmap | **Note** | Supports Roji's provider-agnostic model connection and model privacy controls. |

**Lessons from complaints:** reminders and timers must be reliable (no silent sign-out, the timer
keeps running when the screen is off, alerts strong enough to notice), and durations are easier
to enter than end times.

## Open questions

- Whether Tiimo's calendar sync writes back, and how its data reaches the AI model. Neither is
  documented publicly. Its privacy policy was not reviewed.
- Recurring-routine rules and how missed routine blocks are handled. These aren't on public pages.

## Sources

- [home]: https://www.tiimoapp.com/ (official site)
- [product]: https://tiimoapp.com/product
- [focus]: https://www.tiimoapp.com/product/focus
- [ai]: https://www.tiimoapp.com/product/ai-planning
- [widgets]: https://www.tiimoapp.com/product/widgets-live-activities
- [faq]: https://tiimoapp.com/faq
- [faq-focus]: https://www.tiimoapp.com/faq/focus-timer
- [faq-notif]: https://www.tiimoapp.com/faq/notifications
- [pricing]: https://www.tiimoapp.com/pricing
- [codesign]: https://www.tiimoapp.com/resource-hub/ai-co-planner-design ("How we built a co-planner that supports your executive function", Anders Pape, Head of Design, 2025-09-03)
- [story]: https://www.tiimoapp.com/resource-hub/tiimo-ai-planner-story ("Why AI is the next step in making planning more human", 2025-10-07)
- [appstore]: https://apps.apple.com/us/app/tiimo-daily-to-do-ai-planner/id1480220328 (listing, v3.72.0, ratings, reviews, privacy label)
- [reviews]: https://justuseapp.com/en/app/1480220328/tiimo-visual-daily-planner/reviews (App Store review aggregator, secondary)
- [yt-issues]: https://www.youtube.com/watch?v=o-XXYR9qTyA
- [yt-neuro]: https://www.youtube.com/watch?v=TQRWQ5FmIe8
- [yt-exposed]: https://www.youtube.com/watch?v=1UrEoLyjni4
- [yt-trap]: https://www.youtube.com/watch?v=Nl5Ll4PvCbE

The Google Play listing returned 404 for the package ID tried, so Android reviews are not
covered.

[home]: https://www.tiimoapp.com/
[product]: https://tiimoapp.com/product
[focus]: https://www.tiimoapp.com/product/focus
[ai]: https://www.tiimoapp.com/product/ai-planning
[widgets]: https://www.tiimoapp.com/product/widgets-live-activities
[faq]: https://tiimoapp.com/faq
[faq-focus]: https://www.tiimoapp.com/faq/focus-timer
[faq-notif]: https://www.tiimoapp.com/faq/notifications
[pricing]: https://www.tiimoapp.com/pricing
[codesign]: https://www.tiimoapp.com/resource-hub/ai-co-planner-design
[story]: https://www.tiimoapp.com/resource-hub/tiimo-ai-planner-story
[appstore]: https://apps.apple.com/us/app/tiimo-daily-to-do-ai-planner/id1480220328
[reviews]: https://justuseapp.com/en/app/1480220328/tiimo-visual-daily-planner/reviews
[yt-issues]: https://www.youtube.com/watch?v=o-XXYR9qTyA
[yt-neuro]: https://www.youtube.com/watch?v=TQRWQ5FmIe8
[yt-exposed]: https://www.youtube.com/watch?v=1UrEoLyjni4
[yt-trap]: https://www.youtube.com/watch?v=Nl5Ll4PvCbE
