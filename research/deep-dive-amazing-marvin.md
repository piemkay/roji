# Deep dive: Amazing Marvin

Research for issue #71 on the Roji MVP map (#43). Gathered 2026-11-24.

**Question:** Which anti-procrastination and ADHD strategies does Amazing Marvin offer (rewards,
procrastination wizard, daily task limits, Smart Lists), and what does the large number of
settings do to users? What should Roji borrow for motivation, and what does it warn against?

## Short answer

Marvin is a to-do app sold as "built for ADHD minds" ([home][home]). Its core is a plain task
list. Almost everything else is an optional **strategy** (the help center now says "feature")
that you switch on. The help center lists **122 strategies in 24 groups** ([strategies][strategies]),
and the site advertises "over 300 customizable settings" ([home][home]). The anti-procrastination
set is broad. It covers awareness (frogs for dreaded tasks, a delay count of `!` marks), help
starting (a Procrastination Wizard, one-task focus mode, random pickers, shortened lists), rewards
(points, kudos and levels, reward tasks, completion animations) and pressure (nagging reminders,
money pledges). Daily limits are soft: an overbooking warning based on time estimates, and a
Health Dashboard that flags more than 8 tasks in Today.

The settings are both the pitch and the main complaint. Reviewers love that they can "design the
tool around me", and the same reviewers call the first week overwhelming ([alternativeto][alt],
[saner][saner]). Marvin's own advice is to turn on "one or two features at a time" and to start
from ready-made workflow templates ([best practices][best], [pick strategies][pick]). Roji should
take a few of Marvin's motivation mechanisms and build them in as defaults. It should not copy the
toggle catalogue.

## What it offers

### The funnel: narrowing what you see

Marvin's main idea against overwhelm is a funnel of narrower and narrower views
([funnel][funnel]). Backburner holds the not-yet-actionable. Then Master List ("everything you
could possibly do right now"), Planning Ahead (weeks and months), **Orbit** ("your current focus
area"), Today, **Spotlight** ("your next work session (1-3 hours)") and finally **Focus Mode**
("That one task you are going to do right now. Everything else disappears.").

- **Orbit** sits between the backlog and Today. It holds what matters now without scheduling it,
  and it is meant to replace dragging unfinished tasks from one day to the next
  ([orbit][orbit]). Tasks enter by hand, by label, or by auto-orbit rules (due soon, planned
  this week). The article names no expiry, though the Health Dashboard flags items in Orbit longer
  than 30 days ([health][health]).
- **Spotlight** builds a session from today's tasks. "Marvin Suggests" looks at deadlines and
  estimates. Presets include **Quick Wins** (tasks under 15 minutes, "to build momentum"),
  Power Hour and Deep Work. Finishing every task in the session shows a dancing Marvin
  ([spotlight][spotlight]).
- **Super Focus Mode** shows a single task ([search: SFM][sfm]).
- **Truncated List** cuts any list to N items. "Top up" refills as you finish. "Sets" makes you
  finish the group before the next one appears. Picks are random or "Suggested" (most important
  and urgent) ([truncated][truncated]).
- **Decision makers:** Task Jar, Spinning Wheel, Random task, Suggested Task and "This or That"
  (choose between two, then go straight into focus mode) ([strategies][strategies]).

### Procrastination tools

The procrastination page sorts these into awareness, starting and motivation
([procrastinators][procr]).

- **Procrastination Wizard.** "Help me start!" on any task opens a click-through dialog. It asks
  what feels hard, tells "anxious" apart from "unmotivated", and suggests a strategy. Some steps
  ask you to write something down or start a short timer. It "starts with a different question
  each time". It ends with "I am ready to start on it!", which cheers you on and starts time
  tracking. You can also open it by clicking a task's delay marks. It is on by default for new
  users, but on **desktop and web only** ([wizard][wizard], [procrastinators][procr]). The
  question tree itself is not documented publicly.
- **Delay Count.** Marvin remembers when a task was first scheduled and adds one `!` per day
  since then. The marks are orange and turn red after 3 days, and only completing the task
  clears them: unscheduling it keeps the count ([delay count][delay]). A related report lists
  the most procrastinated tasks ([search][wizard-search]). The stated aim is awareness: "You
  can't fix what you can't see" ([playground][playground]).
- **Frogs and "weighing on me".** Task attributes for dread and for tasks that sit on your mind
  ([procrastinators][procr]).
- **Bug Me.** A repeat reminder every 5 minutes to 3 hours "until you finish the task", for
  today, 3 days, 7 days or until done, held back outside a quiet-hours window
  ([bug me][bugme]).
- **Accountability Pledge.** You stake money on a deadline. The pledge "is locked in and cannot be
  undone", and deleting the task counts as failing. The money goes to Marvin's development. If
  you don't pay, you lose the pledge feature, but "We will not go to collection agencies"
  ([pledge][pledge]).
- **Focus Music** (binaural, lo-fi, ambient beats that start with Focus Mode) and **Beat the
  Clock** (race your own estimate) ([adhd][adhd], [procrastinators][procr]).
- Marvin is candid that none of this cures procrastination by itself: "simply storing your tasks
  in Marvin is not going to transform you into a productivity monster overnight"
  ([will marvin fix][fix]).

### Rewards and gamification

Five strategies sit in the Gamification group ([strategies][strategies]).

- **Reward Points.** You give tasks points by difficulty and value, around 1 for easy and 5 for
  hard, with a suggested daily goal of about 20. You spend points on rewards you define, in two
  steps: claim, then mark as spent. An optional daily reset makes you spend them the same day
  ([reward points][points]). **Smart defaults** set points automatically from Smart List
  matches ([search: rewards][rewards-search]).
- **Marvin Kudos.** Automatic points per completed task, weighted by procrastination, deadlines
  met and priority, with **daily streaks**, levels, 2x and 3x multipliers, and cosmetic unlocks
  (mostly check-off animations) ([kudos][kudos]).
- **Reward Tasks.** Turn a task into a treat you do after earning it, shown with a rainbow
  border ([reward tasks][rtasks]).
- **Satisfying completions:** sounds and animations on check-off ([procrastinators][procr]).

### Daily limits and capacity

Marvin has no hard cap. It warns instead:

- **Day Planning capacity estimator:** "left to do" against "left to plan" before your work hours
  end, minus calendar events. Tasks without an estimate count as **20 minutes**, projects as 45.
  There is an optional **one-time overbooking warning**, plus emoji markers comparing the day with
  your usual days over the last 30 ([day planning][dayplan]).
- **Health Dashboard:** a 0-100 score with checks such as "Today is overloaded" (default **max 8
  tasks**), recurring tasks piling up, stale inbox (7 days), inactive projects (90 days) and old
  tasks (180 days). It offers one-click fixes ("Unschedule all", "Keep just one of each" recurring)
  and a one-item-at-a-time sweep. It advises: start with the weakest area and do one short
  sweep ([health][health]).
- **1-3-5** lays the day out as 1 large, 3 medium and 5 small tasks, sorted by estimate. The
  sections set the limit, nothing enforces it ([1-3-5][135]).
- **Rollover** moves unfinished past tasks to today at midnight. A **staleness warning** marks
  tasks older than N days ([search: rollover][rollover]).
- The best-practices guide says to plan a day you can finish: it "feels better to finish your
  list and add more than to move half of it to tomorrow". It also says "Many tasks don't need a
  date at all" ([best practices][best]).

### Smart Lists

Smart Lists are saved filters (all filters must match), with an **advanced filter written in
Reverse Polish Notation** for OR logic, for example `@home @quick &&` ([smart lists][smart]).
They also drive other features: reward-point defaults, 1-3-5 sections, Smart List day alerts. The
power is real, but writing RPN is a developer skill, not a calm-app gesture.

### Price and platforms

Pro costs $8 a month or $96 a year, with a 14-day trial and no free plan. After the trial you can
no longer add or complete tasks ([pricing][pricing]). Apps: web, Mac, Windows, iOS, Android. The
iOS app has 4.0 stars from 130 ratings, and reviewers say "The phone app isn't as useable as the
desktop app" ([app store][appstore]). The wizard and the Health Dashboard are desktop and web
only ([wizard][wizard], [health][health]).

## What the settings do to users

- **Praise for fit:** "the only productivity app that truly feels designed for neurodivergent
  people"; "instead of adapting myself to a tool, I can design the tool around me"
  ([alternativeto][alt]). "So customizable, that when I feel I start to get bored, I can change my
  flow or strategy" ([app store][appstore]). Switching setups for novelty is a deliberate ADHD
  feature: themes can even shuffle automatically ([adhd][adhd]).
- **Cost at the start:** "The number of options can feel overwhelming at first, and there is a
  learning curve"; "the few default settings available were not to my taste" and the lack of
  direction was frustrating ([alternativeto][alt]). Review roundups describe the first week as
  overwhelming and warn "do NOT choose Amazing Marvin if you just want a simple to-do list"
  ([saner][saner], [toolguide][toolguide]).
- **Marvin's own fix is more structure on top:** strategy tags by hurdle (procrastination,
  overwhelm), favourites, videos, one-click Workflow Library templates, and advice to "start
  simple" since Marvin "works great even without any of the strategies turned on"
  ([pick strategies][pick]). One reviewer said a one-click GTD template made everything "magical
  and simple" ([alternativeto][alt]).
- **Maintenance creep:** the Health Dashboard exists because a configurable system collects
  clutter: stale inbox, piled-up recurring tasks, stuck Orbit, old projects ([health][health]).
  That is a weekly-review ritual in another form, and Roji wants none.
- **YouTube:** the reviews found ([yt-1][yt1], [yt-2][yt2], [yt-adhd][yt3], [yt-strategies][yt4])
  stress customization and the "100+ strategies". Their pages and transcripts could not be
  fetched, so they add only that framing.

## Ideas for Roji

| Marvin idea | Fit for Roji | Why |
|---|---|---|
| Funnel: the user sees a narrow slice, never the master list | **Adopt as a principle** | Roji's short Today and focus view are this funnel with the middle levels automated. |
| "Help me start" on a stuck task (wizard) | **Adapt** | A short, optional prompt on a task Roji sees slipping: name what's hard, offer one tactic (smallest step, 5-minute timer). Run by rules or the model, not a big decision tree. |
| Delay count | **Adapt, quietly** | Use the slip count internally to trigger "help me start", a breakdown or a "still want this?" prompt. Don't paint red `!` marks on the task: that is visible shame. |
| Quick Wins / momentum set | **Adopt** | A "few quick ones to warm up" option in the Today suggestion costs little and fits "one suggestion, not a menu". |
| Top-up truncated list (show N, refill on done) | **Adopt** | Matches a short Today that never grows. Same idea as Llama Life's "enough for now" divider. |
| Random or "this or that" picker | **Consider** | Cheap help for decision paralysis. Pick between two, then go to the focus view. |
| Capacity warning from estimates (default 20 min if none) | **Adopt** | Same direction as Sunsama's overbooking warning. Default estimates mean the user doesn't have to estimate everything. |
| Health checks (today overloaded, stale items, piled-up recurrences) | **Adapt** | Run them silently as automatic sorting (archive, collapse recurrences, demote stale items) instead of a score and a sweep the user must do. |
| Orbit (relevant now, not scheduled) | **Consider for the model** | A "now-ish" pool between backlog and Today is close to what Roji's automatic sorting needs. |
| Completion delight (sound, small animation, end-of-session celebration) | **Adopt, small** | A cheap dopamine hit. Keep it calm and turn-offable. |
| Reward tasks (treat after work) | **Consider later** | Gentle and self-directed. Not MVP. |
| Points, kudos, levels, multipliers, daily streaks | **Avoid** | Streaks punish missed days. Points ask the user to price every task, which is setup work. |
| Bug Me nagging, money pledges | **Avoid** | Pressure and fear are the opposite of calm, and pledges are punitive by design. |
| 122 toggleable strategies, 300+ settings, RPN filters | **Avoid** | The clearest warning in this survey: choice becomes setup, setup becomes overwhelm. Roji should ship one opinionated default with very few switches. |
| Mood tracker, focus music | **Defer** | Not core. Music is better left to the user's own player. |

**The warning in one line:** Marvin tries to make overwhelm optional through configuration.
For an ADHD user that just moves the overwhelm to setup. Roji should pick for the user, with a
way out of each choice, not a catalogue of choices.

## Open questions

- The wizard's actual question tree is not in the public docs. A trial account would show it, if
  the design of a "help me start" prompt needs more detail.
- No usage data on which strategies people actually keep enabled. The help center's tags by
  hurdle suggest procrastination and overwhelm are the main draws.
- Capterra and G2 returned 403, and the Google Play listing could not be read, so Android
  reviews are missing.

## Sources

- [home]: https://amazingmarvin.com/ (official site)
- [procr]: https://amazingmarvin.com/for/procrastinators/
- [adhd]: https://amazingmarvin.com/for/adhd/
- [funnel]: https://amazingmarvin.com/features/funnel/ (The Marvin Method)
- [pricing]: https://amazingmarvin.com/pricing/
- [playground]: https://playground.amazingmarvin.com/how-to-beat-procrastination-with-marvin/
- [strategies]: http://help.amazingmarvin.com/en/collections/1139197-strategies (list of all strategies)
- [pick]: https://help.amazingmarvin.com/en/articles/1949346-how-to-pick-strategies
- [best]: https://help.amazingmarvin.com/amazing-marvin/best-practices-checklist
- [fix]: https://help.amazingmarvin.com/en/articles/1949435-will-marvin-fix-my-procrastination-problem
- [wizard]: https://help.amazingmarvin.com/amazing-marvin/procrastination-wizard
- [delay]: https://help.amazingmarvin.com/en/articles/1950154-procrastination-count
- [bugme]: https://help.amazingmarvin.com/en/articles/8950753-bug-me
- [pledge]: https://help.amazingmarvin.com/amazing-marvin/accountability-pledge
- [points]: https://help.amazingmarvin.com/amazing-marvin/reward-points
- [kudos]: http://help.amazingmarvin.com/en/articles/5049720-marvin-kudos
- [rtasks]: http://help.amazingmarvin.com/en/articles/1950239-reward-tasks
- [orbit]: https://help.amazingmarvin.com/amazing-marvin/orbit
- [spotlight]: https://help.amazingmarvin.com/amazing-marvin/spotlight
- [truncated]: https://help.amazingmarvin.com/en/articles/3623546-truncated-list
- [smart]: http://help.amazingmarvin.com/en/articles/1950204-smart-lists
- [dayplan]: https://help.amazingmarvin.com/amazing-marvin/day-planning-2
- [health]: https://help.amazingmarvin.com/amazing-marvin/health-dashboard
- [135]: https://help.amazingmarvin.com/en/articles/4561574-1-3-5
- [rollover]: https://help.amazingmarvin.com/en/articles/3670563-rollover-scheduled-tasks and https://help.amazingmarvin.com/en/articles/1950228-staleness-warning (via search results)
- [sfm]: https://help.amazingmarvin.com/en/articles/5364004-top-of-mind (via search results)
- [rewards-search]: http://help.amazingmarvin.com/en/articles/4510090-reward-point-smart-defaults (via search results)
- [wizard-search]: https://help.amazingmarvin.com/en/articles/3733206-most-procrastinated-tasks (via search results)
- [appstore]: https://apps.apple.com/us/app/amazing-marvin-on-the-go/id1314511546 (v1.71.4, 4.0 from 130 ratings)
- [alt]: https://alternativeto.net/software/amazing-marvin/reviews/ (user reviews, secondary)
- [saner]: https://blog.saner.ai/amazing-marvin-review/ (review roundup, secondary, seen via search only)
- [toolguide]: https://toolguide.io/en/tool/amazing-marvin/ (review, secondary, seen via search only)
- [yt1]: https://www.youtube.com/watch?v=7Hn_LjfJYQM
- [yt2]: https://www.youtube.com/watch?v=KCK6_Cv7GKA
- [yt3]: https://www.youtube.com/watch?v=mtKYHcPKgiQ
- [yt4]: https://www.youtube.com/watch?v=S73qVDasqn0

[home]: https://amazingmarvin.com/
[procr]: https://amazingmarvin.com/for/procrastinators/
[adhd]: https://amazingmarvin.com/for/adhd/
[funnel]: https://amazingmarvin.com/features/funnel/
[pricing]: https://amazingmarvin.com/pricing/
[playground]: https://playground.amazingmarvin.com/how-to-beat-procrastination-with-marvin/
[strategies]: http://help.amazingmarvin.com/en/collections/1139197-strategies
[pick]: https://help.amazingmarvin.com/en/articles/1949346-how-to-pick-strategies
[best]: https://help.amazingmarvin.com/amazing-marvin/best-practices-checklist
[fix]: https://help.amazingmarvin.com/en/articles/1949435-will-marvin-fix-my-procrastination-problem
[wizard]: https://help.amazingmarvin.com/amazing-marvin/procrastination-wizard
[delay]: https://help.amazingmarvin.com/en/articles/1950154-procrastination-count
[bugme]: https://help.amazingmarvin.com/en/articles/8950753-bug-me
[pledge]: https://help.amazingmarvin.com/amazing-marvin/accountability-pledge
[points]: https://help.amazingmarvin.com/amazing-marvin/reward-points
[kudos]: http://help.amazingmarvin.com/en/articles/5049720-marvin-kudos
[rtasks]: http://help.amazingmarvin.com/en/articles/1950239-reward-tasks
[orbit]: https://help.amazingmarvin.com/amazing-marvin/orbit
[spotlight]: https://help.amazingmarvin.com/amazing-marvin/spotlight
[truncated]: https://help.amazingmarvin.com/en/articles/3623546-truncated-list
[smart]: http://help.amazingmarvin.com/en/articles/1950204-smart-lists
[dayplan]: https://help.amazingmarvin.com/amazing-marvin/day-planning-2
[health]: https://help.amazingmarvin.com/amazing-marvin/health-dashboard
[135]: https://help.amazingmarvin.com/en/articles/4561574-1-3-5
[rollover]: https://help.amazingmarvin.com/en/articles/3670563-rollover-scheduled-tasks
[sfm]: https://help.amazingmarvin.com/en/articles/5364004-top-of-mind
[rewards-search]: http://help.amazingmarvin.com/en/articles/4510090-reward-point-smart-defaults
[wizard-search]: https://help.amazingmarvin.com/en/articles/3733206-most-procrastinated-tasks
[appstore]: https://apps.apple.com/us/app/amazing-marvin-on-the-go/id1314511546
[alt]: https://alternativeto.net/software/amazing-marvin/reviews/
[saner]: https://blog.saner.ai/amazing-marvin-review/
[toolguide]: https://toolguide.io/en/tool/amazing-marvin/
[yt1]: https://www.youtube.com/watch?v=7Hn_LjfJYQM
[yt2]: https://www.youtube.com/watch?v=KCK6_Cv7GKA
[yt3]: https://www.youtube.com/watch?v=mtKYHcPKgiQ
[yt4]: https://www.youtube.com/watch?v=S73qVDasqn0
