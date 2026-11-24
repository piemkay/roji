# Deep dive: Llama Life

Research for issue #48 on the Roji MVP map (#43). Gathered 2026-11-24.

**Question:** How does Llama Life work in detail: time-boxed lists, the focus timer,
celebrations, ADHD-friendly design, integrations? What do users love and dislike, and which
ideas fit Roji's focus view and motivation?

## Short answer

Llama Life is a **timeboxed list**, not a planner or calendar. You type today's tasks, give
each one a duration, and work down the list one task at a time while a countdown runs on the
top task. The list shows its total length and a **projected end time** that moves as you work,
and a draggable **Time Divider** marks the part of the list you mean to finish. When a task's
time runs out, **Overtime** counts up instead of failing you, and you decide afterwards whether
the extra time counts. Finishing a task fires **confetti**. Fun comes from colors, emoji themes,
soundscapes and gentle interval **chimes**. The only documented integration is a one-way pull of
Todoist tasks due today. The founder, Marie Ng, was diagnosed with ADHD as an adult and pitches
the app as "one task at a time, so you're not just making lists, you're actually getting
THROUGH them".

Users love the end-time calculation (it cures overbooking and time blindness), the per-task
timer, presets for routines, the low-clutter design and the confetti. They dislike the hard
paywall and cancellation friction, reliability problems (alarms not firing, lists reverting,
needing internet), no fixed-time anchors for meetings, and no widgets, watch or iPad apps.

For Roji's focus view the strongest ideas are the live **end time** for the chosen work, the
**divider** as a "this is enough for now" line, **Overtime without judgment**, the
**Not-Now** list as a pressure valve, and small, optional celebration. Roji should not copy
the manual setup: Llama Life asks you to estimate and order every task yourself each day, and
it has no capture, sorting or backlog. Roji wants to remove that work.

## How it works

### Lists and timeboxing

- The core concept is "timeboxing, which is the practice of setting a fixed amount of time to
  each task" ([How does Llama Life work?][how]). The store text calls it "a (positive)
  constraint around the time we have to do something" ([App Store][appstore], [Google Play][play]).
- There is one **Main List** for today. The top task is the loaded one; its timer runs and the
  pie timer takes its color ([Using timers][timers]).
- **Smart Time Detection:** a number at the end of a task title becomes its duration in
  minutes ("Reply to emails 10" -> 10 min). It can be turned off ([numbers in titles][numbers]).
  The same rule applies to Todoist imports ([Todoist integration][todoist]).
- **List Time and End Time:** the bottom of the list shows the total planned time and the
  projected finishing time ([finishing time][endtime]). An optional setting shows a start and
  end time for every task, which update as you work, change durations, add or reorder tasks:
  "unlike a static calendar" ([start and end time][startend]).
- **Time Divider:** a draggable line in the list. "Only the tasks above it will be captured in
  the List and End times." The docs suggest moving it up until the end time fits a fixed window,
  such as a three-hour morning ([Time Divider][divider]). Random Task only picks from tasks above
  it ([App Store version history][appstore]).
- **Not-Now list:** a second list, "your temporary place for tasks that still needs to be done,
  just not at this very minute". Tasks move back and forth with time spent and Overtime kept;
  you can move all incomplete tasks at once and filter for tasks of 10 minutes or less. The
  docs suggest it for overwhelm or when End time grows too long ([Not-Now][notnow]).
- **Presets:** saved lists for repeated tasks and routines, loaded whole or task by task. There
  is no recurrence scheduler; recurring work means reloading a preset. A template gallery covers
  methods, work routines and personal tasks ([Presets][presets]).
- **Break it down:** from a task's menu, generates smaller steps "with approximate times",
  which you prune and then add (replacing the original), save as a preset, or run again
  ([Break it down][breakdown]). The maker calls this AI ([Product Hunt launch][ph]). One reviewer
  says the generated steps are often "nonsense" and cannot be edited before adding
  ([App Store reviews][rss]).
- **Random Task:** for when starting is hard; it picks a task and offers "Yes start now!" or
  "No thanks", leaning on "interest and novelty" rather than urgency ([Random Task][random]).

### Focus timer

- A digital countdown starts with play and has **-5 / +5** minute buttons. The alternative
  **pie timer** "shows time in an absolute sense, meaning 10 mins will always look the same"
  ([Using timers][timers]).
- **Auto Start Next Task:** completing a task starts the next timer, useful for small tasks and
  for people who forget to start timers ([Auto Start][autostart]).
- **Alarms** at time-up: visual (window flashes until clicked) and audio, in three strengths:
  once, **Nag** (every minute until stopped) and **Continuous** ([Alarms][alarms]).
- **Chimes:** "a gentle alarm that can help to nudge your attention back to the task at hand",
  at an interval and volume you choose, e.g. every 5 minutes ([Chimes][chimes], [Alarms][alarms]).
- **Overtime:** when the timer hits zero (after a configurable delay) a count-up appears above
  it. You can dismiss alarms and keep working; at the end you **Add** or **Delete** the extra
  time. A red dot marks completed tasks with unresolved Overtime ([Overtime][overtime]).
  A YouTube comment surfaced by search calls the overtime counter "helpful and underrated" for
  learning time estimation (search snippet only, not verified at the source).
- **Report:** planned vs. actual time per task and in total for the Main List. There is no
  history: to see a day or week you clear completed tasks at the start of it ([Report][report]).
  A 4-star reviewer asks for timestamps of when tasks were started and completed
  ([App Store reviews][rss]).
- Soundscapes for background noise; Live Activities and Dynamic Island on iOS
  ([How it works][how], [App Store][appstore]).

### Celebrations and fun

- "We also love celebrating the wins, big or small, so very importantly, you get confetti
  (woo hoo!) when you finish a task" ([App Store][appstore]).
- Per-task colors, Color Me themes, a "surprise me" color button, emoji themes including
  user-made ones (up to 10 emoji), AI emoji matching, dark mode, a llama mascot and playful
  notification copy ([colors][colors], [emoji][emoji], [App Store][appstore], [reviews][rss]).
- No streaks, points or scores appear in the docs or store text. Motivation is per-task
  delight plus the satisfaction of a shrinking list and an honest end time.

### ADHD-friendly design

- Positioning: "designed to help you focus on ONE task at a time"; the store title is now
  "ADHD Organizer: Llama Life" ([App Store][appstore]). The founder built it after her adult
  diagnosis, wanting "some structure but not too much", and prefers 3-5 minute blocks to
  25-minute Pomodoros ([Product Hunt launch][ph]).
- Design moves aimed at ADHD traits: time blindness (end time, pie timer, chimes, overtime),
  task initiation (Random Task, Break it down, Auto Start), overwhelm (divider, Not-Now),
  boredom (colors, emoji, confetti, soundscapes).
- Many users arrive as orphans of **30/30**, a discontinued list-of-timers iOS app
  ([App Store reviews][rss]).

### Integrations and platforms

- **Todoist:** "a one-way sync that will pull tasks that are due today in Todoist", delivered
  as a Todoist preset list you load whole or in part ([Todoist integration][todoist]). The docs
  don't describe completion syncing back.
- The 2022 product demo also mentions Notion ([product demo][demo]), and a 2024 reviewer
  mentions "integration with notion or todoist" ([App Store reviews][rss]); the current help
  center lists only Todoist ([Help center: features][features]).
- Platforms: web app (the original), desktop app, iPhone (since February 2024), Android; synced.
  No iPad, Apple Watch or home-screen widgets ([App Store][appstore], [reviews][rss]).
- Pricing: 7-day free trial (no card needed), then monthly or annual; App Store IAPs list
  $4-6/month, $29/6 months, $39/year ([App Store][appstore]). Google Play: 3.8 stars, 154
  reviews, 10K+ downloads ([Google Play][play]). App Store US: 4.5 stars, 118 ratings.

## What users love

- **End time and list time.** The most repeated praise. "No more squeezing in 10+ hours into an
  8 hour work day"; "No I can't fit 86 hours of tasks into a day"; "the End Time features are
  the only thing that have helped" with time blindness ([App Store reviews][rss],
  [App Store AU][appstore-au]).
- **Flexible timeboxing.** "Standard time blocking has been a nightmare for me"; one reviewer
  says it taught them "time sometimes does slip" without replanning ([App Store][appstore],
  [reviews][rss]). "Pomodoro on steroids... not all tasks are equal" ([App Store][appstore]).
- **Calm and simple.** "What surprises me the most is that I feel calm"; "takes away stress
  instead of replacing it"; "basic and there isn't a ton of extra stuff" ([App Store][appstore]).
- **Presets for routines** (morning routine, guests coming over, night prep) ([reviews][rss]).
- **Confetti, colors and emoji:** "little dopamine hits with every task you finish"
  ([App Store AU][appstore-au]).
- **Card-free trial** and responsive support from the founder ([reviews][rss]).
- The non-sponsored YouTube review by anomalily (2022) calls it "a kitchen timer, a to do list,
  and a bunch of cute emojis had a baby", used alongside a paper planner, Todoist or Toggl; it
  has chapters on meetings, studying and procrastination ([YouTube review][yt-anomalily]).
  Its transcript was not reachable, so only the description is cited.

## What users dislike

- **Paywall and cancellation.** You can't create a task without subscribing; pricing is not
  clear before sign-up; cancelling and deleting the account takes email or a 30-day delay;
  "too expensive for what it is" ([App Store reviews][rss]).
- **Reliability.** Alarms not sounding ("the whole point of the app"), late notifications,
  lists reverting to an older state, unsaved task names, crashes ([App Store reviews][rss],
  [Google Play][play]). Recent releases fix list reversion and closed-app alarms
  ([App Store version history][appstore]).
- **Needs internet.** "You can't update anything with a lack of internet connection"
  ([App Store reviews][rss]).
- **No fixed-time anchors.** "The inability to add fixed times at all is annoying... manually
  readjusting placeholders for meetings because the tasks before them had moved"; the request
  is two years old ([Google Play][play]).
- **Out of sight, out of mind.** No widgets ("a dealbreaker"), no watch or iPad app
  ([App Store reviews][rss]).
- **Thin organisation.** Colors but no real categories or notes; no search for presets; no
  history or timestamps; no bulk import on mobile ([App Store reviews][rss]).
- One skeptic: "a simple pomodoro app... more marketing, polished app but not much
  functionality" (via search summary of app store reviews; not verified at the source).

## What fits Roji

Roji's map scopes a focus view with a timer inside the web app and a retroactive time log, and
promises a short suggested Today list with no planning rituals.

**Take:**

1. **A live end time for the chosen work.** Show "you'll be done around 15:40" for the tasks in
   focus, updating as you work. It is the single most-loved feature and it answers time
   blindness without a calendar. Roji can fill in durations itself (estimates from rules or the
   model) so the user doesn't have to.
2. **A divider as "enough for now".** The Today suggestion could end in a visible line; tasks
   below it are not failures. This matches "short suggested Today list".
3. **Overtime without judgment.** When a timebox ends, count up quietly and keep the time.
   It feeds Roji's retroactive time log directly; Llama Life's lack of history and timestamps is
   a gap Roji's log already fills.
4. **Not-Now as a pressure valve.** One gesture to move everything but the current task out of
   view, without deleting or rescheduling it.
5. **Gentle, optional cues.** Interval chimes, a pie timer that shows absolute time, a heads-up
   before time ends (users asked for "1 min left, 5 min left"), alarm strength as a setting.
6. **Small celebration per task**, not streaks. Confetti or an equivalent calm moment on
   completion, switchable off to fit Roji's calm tone.
7. **Starting help:** "pick one for me" and break-down-into-steps, with editable steps.

**Avoid:**

- Making the user estimate and order every task by hand each morning; that is the ritual Roji
  removes.
- Ignoring fixed-time commitments: Roji's read-only calendar should anchor the end time around
  meetings, the gap users complain about most.
- Online-only editing and a hard paywall before first use (irrelevant to a self-hosted MVP, but
  offline tolerance in the installed web app matters).
- Unreliable alarms: browser audio and notifications are fragile (the help center has Safari and
  mobile sound troubleshooting pages), so the focus view's time-up cue needs a visual fallback.

## Sources

[how]: https://intercom.help/llama-life/en/articles/6455699-how-does-llama-life-work
[features]: https://intercom.help/llama-life/en/collections/3576374-2-llama-life-features
[timers]: https://intercom.help/llama-life/en/articles/6456401-using-timers
[overtime]: https://intercom.help/llama-life/en/articles/6575141-using-the-overtime-feature
[alarms]: https://intercom.help/llama-life/en/articles/6456183-using-alarms
[chimes]: https://intercom.help/llama-life/en/articles/8988827-how-to-use-our-chimes
[report]: https://intercom.help/llama-life/en/articles/6456343-using-the-report
[startend]: https://intercom.help/llama-life/en/articles/6459396-how-do-i-see-the-start-and-end-time-of-each-task
[endtime]: https://intercom.help/llama-life/en/articles/6459428-can-i-see-what-time-i-ll-be-finishing-my-tasks
[divider]: https://intercom.help/llama-life/en/articles/8974299-using-the-time-divider
[notnow]: https://intercom.help/llama-life/en/articles/9847313-using-the-not-now-second-list-feature
[presets]: https://intercom.help/llama-life/en/articles/6455997-managing-recurring-tasks-and-creating-lists-with-our-presets
[breakdown]: https://intercom.help/llama-life/en/articles/8332767-how-to-use-our-break-it-down-feature
[random]: https://intercom.help/llama-life/en/articles/7923003-how-to-use-our-random-task-feature
[autostart]: https://intercom.help/llama-life/en/articles/7051604-using-the-auto-start-next-task-feature
[numbers]: https://intercom.help/llama-life/en/articles/6452914-i-m-typing-numbers-in-the-task-title-but-they-are-disappearing
[colors]: https://intercom.help/llama-life/en/articles/6455766-using-task-colors-and-color-me-themes
[emoji]: https://intercom.help/llama-life/en/articles/6455798-using-emojis-and-emoji-me-themes
[todoist]: https://intercom.help/llama-life/en/articles/6456300-todoist-integration
[appstore]: https://apps.apple.com/us/app/adhd-organizer-llama-life/id6454469750
[appstore-au]: https://apps.apple.com/au/app/llama-life/id6454469750
[rss]: https://itunes.apple.com/us/rss/customerreviews/id=6454469750/sortBy=mostRecent/json
[play]: https://play.google.com/store/apps/details?id=com.llamaapp
[ph]: https://www.producthunt.com/p/llama-life/llama-life-2
[demo]: https://www.youtube.com/watch?v=6YcVS7HtoiU
[yt-anomalily]: https://www.youtube.com/watch?v=DT4egaNpr4s
[yt-search]: https://www.youtube.com/@llamalifeco/featured

- Llama Life help center (Intercom): [how it works][how], [features collection][features] and
  the individual feature articles linked above.
- [App Store listing (US)][appstore], [App Store (AU)][appstore-au], [App Store US review feed][rss]
  (50 most recent reviews, 2024-2026).
- [Google Play listing][play] with visible reviews.
- [Product Hunt launch, Llama Life 2][ph] (maker post by Marie Ng).
- YouTube: [official product demo, 2022][demo]; [anomalily review, 2022, non-sponsored][yt-anomalily];
  Megan Renee, "I think I found the best platform for ADHD routines?!" (2022, affiliate code,
  https://www.youtube.com/watch?v=-QjkCV5bt6I); [official channel][yt-search]. Video transcripts
  were not retrievable; only titles, descriptions and chapter lists were read.
- Not reached: llamalife.co itself renders client-side and returned no text; no Reddit threads
  surfaced.
