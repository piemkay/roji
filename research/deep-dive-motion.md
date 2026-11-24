# Deep dive: how Motion plans a day

Researched 2026-11-24. Motion's help center was mid-migration (old `help.usemotion.com` links now
301 to `usemotion.com/help`), and several pages carry no date. Claims marked "unverified" could
not be traced to a page I could read myself.

## Answer in brief

Motion is a constraint solver over your calendar. Every task needs a duration plus a deadline or
priority; Motion then writes time blocks for it into your calendar within work hours, ordered
roughly ASAP > hard deadline > soft deadline by date > priority, and re-runs the whole plan
whenever anything changes ("dozens of times a day") [1][3][9]. Missed work is rescheduled into
the next free slot, and tasks that can no longer meet their deadline turn red, yellow or "Could
not fit" and land in a past-due queue the user must clear [2][5][6]. Since 2025 a daily "AI
Agenda" document sits on top of the scheduler as a Today list [12]. The company has since
repositioned as an "AI SuperApp for Work" with notetaker, docs, AI project manager and (from May
2025) agent "AI Employees", which appear to have been de-emphasized or withdrawn by late 2026
[9][17][19][23]. Users love handing off the "what next" decision and the automatic reflow; they
dislike setup burden, clutter, rigidity, warnings that pile up, a product drifting toward
team/AI features, and price and billing practices [20][21][22][23]. For Roji: copy the inputs
that cost little (rough size, a "must by" date, a start-not-before date) and the single
"do this next" framing; avoid continuous silent reshuffling, time-slot precision, red overdue
queues, and mandatory fields.

## Inputs

Per task, Motion's scheduler reads [1][3]:

- **Duration** (required). A task with no duration is not scheduled [2].
- **Deadline**, either **hard** or soft. Hard deadlines may be scheduled outside work hours to be
  met (the help example places a task at 6 PM when 9 to 5 is full) [3][4].
- **Priority**: ASAP, High, Medium, Low. ASAP "overrides all other scheduling parameters" [3].
  A task with neither deadline nor priority is not scheduled [2].
- **Start date**: tasks "will not be scheduled before this point, even if time is available" [1].
- **Chunking**: long tasks split into blocks ("min chunks") to fit gaps; with no chunks defined,
  Motion looks for one slot fitting the whole duration [1][3].
- **Recurrence**: recurring tasks move only within their recurrence window; a missed instance
  becomes overdue and the next one still appears on schedule [1][4].
- **"Warn if past due"** toggle, which decides whether a task counts in the overdue badge [15].

Per user [1][3]:

- **Schedules** (work hours, plus named schedules such as Workday or Weekend that tasks can be
  assigned to), **breaks** between tasks, and **flexible hours** overrides for a single day.
- **Calendar events** marked Busy count as unavailable; Free events do not [1][4].

Inputs are fine-grained and mostly required. Reviewers repeatedly name the setup effort as the
cost [21][22].

## How it prioritizes and schedules

- Order, per the main help page: ASAP; then hard deadlines; then soft deadlines by due date; then
  priority (High > Medium > Low); then duration, start date, recurrence [3]. The same page also
  says priority "always comes first", so the documented order is internally inconsistent [3].
- Tasks from all projects in a workspace compete for the same time [2].
- It places tasks only in free time inside the chosen schedule, chunking if allowed, respecting
  breaks [1][2].
- It works over a rolling window (two weeks per a third-party review and a help-search summary)
  [4][21]; tasks that cannot fit inside it are marked "Could not fit" and stay unscheduled until
  the user changes duration, deadline or priority [2][4].
- Each task gets an ETA, which serves as an early warning that it is likely to miss its deadline
  [2].
- Re-planning is continuous: adding a meeting, moving a task, or changing hours triggers it, and
  the homepage claims the schedule is re-optimized "dozens of times a day" [3][9].
- Users can lock a task's time if it "keeps moving" [2]; the help center gives little detail.

## Missed tasks and rescheduling

- A past-due task is rescheduled "into the next available slot" and shown as overdue [2].
- Status indicators: a red marker for tasks "scheduled past its deadline", and project-level
  states such as Missed Deadline, Scheduled Past Deadline, Stage Missed Deadline, On Track and
  Ahead of Schedule [6].
- A yellow "!!" badge next to AI Agenda opens the overdue list. Help text: "Until cleared,
  overdue tasks will block your schedule and throw off prioritization." Per task the user picks
  Do ASAP, Extend deadline, Ignore warning, Complete, or Delete [5].
- An older "Resolve Issues" box grouped past-due and unfittable tasks, red for hard deadlines and
  orange for soft [8] (unverified: the page now redirects; known only from search snippets).
- Hard deadlines are a common cause of "Could not fit"; the FAQ's advice is to turn the hard
  deadline off [4].

The net effect: the user's backlog of slipped work becomes a visible queue of colored warnings,
and the plan for everything else moves underneath them.

## Calendar use

- Motion is a writer to the calendar. Tasks become time blocks in Motion's calendar, and
  optionally on the user's Google or Outlook calendar [7].
- Three display modes: tasks only in Motion; all tasks shown as Free externally; or tasks shown
  Free until "at risk" (one more reschedule would make them late), then Busy [7].
- Events sync both ways; tasks flow one way, Motion to external (from a help-search summary,
  unverified against the page itself) [7].
- External calendar events "always override task blocks" [2].
- Motion itself responded to calendar clutter: Agenda View (Oct 2024) was built because "many of
  our users told us that their auto-scheduled calendars were becoming cluttered and hard to
  navigate" [13], and was replaced in May 2025 by the AI Agenda [12].

## AI Agenda (the closest analogue to Roji's Today)

Launched 2025-05-26 as an AI-generated daily document with sections: New tasks added today;
Today's Tasks ("to keep you on schedule for your own and your team's deadlines"); Tomorrow's
Tasks; Tasks due rest of this week; Tasks past deadline [12]. It is editable, reorderable, and
backed by the scheduler: "Today is precise... tasks scheduled further out are approximate" and
the plan "adjusts automatically when meetings shift" [14][15]. The help center does not document
how Today's tasks are selected [14][15]. A relayed October 2025 r/UseMotion post called the
redesign "useless", saying "I constantly find myself manually moving tasks around" [23][24].

## AI features

- **AI tier** (2025-04-21): AI Notetaker for Meet and Zoom, AI Docs that create tasks and
  projects inline, AI project templates from a prompt or SOP, task capture by email forward and
  Siri [11].
- **AI Notetaker**: joins Zoom, Meet or Teams; notes, transcripts and summaries live in an AI
  Meeting Notes tab; suggested action items must be approved one by one before they become
  tasks [16].
- **Current positioning** (homepage, late 2026): "The AI Powered SuperApp for Work", with AI Task
  Planner, AI Project Manager, AI Calendar Assistant ("plans your day for you"), Meeting
  Notetaker, Docs Assistant, Search, Workflow Builder, Dashboards and Gantt [9].
- **AI Employees**: launched May 2025 as named agents (executive assistant, sales rep, support
  rep, marketing assistant) priced by credits; TechCrunch reported 10,000+ B2B customers and
  $10M ARR within four months and a $38M Series C at $550M post-money (2025-09-08) [17]. By
  November 2026 the help-center sitemap lists no AI Employees pages, the old concept pages 404,
  and `/ai-employees` shows generic AI tools instead [18][19]. A relayed December 2025 Reddit
  thread was titled "AI employees have been deprecated" [23][24]. Status: likely withdrawn or
  folded in; unverified, no official announcement found.
- **Pricing** (late 2026): Pro AI $19 and Business AI $29 per seat per month (billing period not
  stated on page), with metered AI credits (7,500 or 15,000 per seat per month) [10]. A
  competitor review lists individual plans at $49 monthly or $29 per month annually, a 7-day
  trial with card required [22]. Older individual pricing was $34 per month or $228 per year
  [21].

## What users love

- **Handing off "what next".** "Automatically prioritizes tasks and schedules everything
  efficiently" (Trustpilot, Mar 2026) [20]; ChoosingTherapy credits it with reducing decision
  fatigue, "especially those with ADHD" [21].
- **Automatic reflow when the day slips.** Auto-rescheduling is the most praised feature across
  reviews; "I love how it reschedules and rearranges my day" (r/UseMotion, Feb 2026, relayed)
  [21][23]. A two-year user said auto-scheduling "more than made up for these shortcomings" [23].
- **Deadline foresight.** The ETA and at-risk warnings help users who underestimate durations [2][21].

## What users dislike (overwhelm, loss of control)

- **Overwhelm and clutter.** "The AI calendar booking is overwhelming" and schedules over
  higher-priority work (Trustpilot, Jun 2026) [20]. Motion's own blog concedes auto-scheduled
  calendars became "cluttered and hard to navigate" [13].
- **Setup burden.** "I needed more time to set up Motion the way it would work best"; Reddit
  users found it "too complicated and took too long to load tasks into the system" [21]. Every
  task needs duration plus deadline or priority to be scheduled at all [2].
- **Loss of control and rigidity.** Auto-scheduling "can feel rigid" when plans change at short
  notice (Capterra, cited by a competitor) [22]. Morgen contrasts it with a planner whose
  suggestions you accept or edit [22]. A competitor page claims auto-rescheduling without
  approval is the most-cited complaint on r/productivity (unverified marketing claim) [25].
- **Warnings and overdue debt.** Overdue tasks "block your schedule" until cleared [5]; reviewers
  report items landing past deadline and needing manual triage [20]. Not well documented by
  first-hand Reddit sources I could reach.
- **Poor fit for variable lives and initiation problems.** The ChoosingTherapy reviewer (3/5)
  felt it did not account for her many variable commitments and that weak reminders made it
  "easy to forget about" [21].
- **Price and billing.** Repeated complaints of price increases ("a stunning 30% increase"),
  charges on "Try for free", and surprise auto-renewals; Trustpilot 3.7 from 579 reviews with
  27% one-star [20][23].
- **Product drift.** "Launching 'AI Employees' and other flashy features that literally nobody
  asked for" (Nov 2025); "the product may be stagnating" (Jul 2026), both relayed [23].

## Copy / Avoid for Roji's Today suggestion

Roji never writes to the calendar and wants a short list, so Motion's core mechanism (writing and
rewriting time blocks) is out. What transfers is its input model and its framing.

Copy:

1. **A "do this next" answer.** The thing users actually praise is not having to decide [20][21].
   Roji's Today should be a short ordered list with an obvious first item.
2. **Capacity from the read-only calendar.** Use meetings and Busy events only to size the day
   ("you have about 3 free hours") and cap Today to fit, not to place tasks in slots [1][2].
3. **A few cheap inputs.** Rough size (S/M/L, not minutes), an optional "must by" date that
   behaves like a hard deadline for selection, and "not before" dates so tasks stay hidden until
   relevant [1][3]. Keep everything optional; Motion refusing to schedule tasks without
   duration plus deadline or priority is a setup tax [2].
4. **Early, quiet risk signal.** Motion's ETA idea is good: notice a "must by" item that will not
   fit before its date and suggest it earlier [2]. Show it as a gentle nudge, not a red badge.
5. **Precise today, loose later.** Motion's own principle [14] suits Roji: only Today is
   concrete; the rest is an unordered pool.
6. **Approve-before-adding.** The notetaker's review-then-approve step for suggested tasks [16]
   is the right default for anything AI proposes to Roji.

Avoid:

1. **Continuous silent reshuffling.** Re-planning "dozens of times a day" [9] is what users
   experience as loss of control [22][23]. Generate Today once (morning or on request), keep it
   stable through the day, and only change it when the user asks.
2. **Time-slot precision.** Slots go stale the moment a meeting overruns; Motion needed Agenda
   View and then AI Agenda to undo the clutter [12][13]. A list, not a timetable.
3. **Overdue as debt.** No red or "!!" queue that "blocks your schedule until cleared" [5][6].
   Missed items should silently return to the pool; at most offer a calm "carry over, reschedule,
   or let go" choice, one item at a time.
4. **Long Today lists.** Motion's agenda includes today, tomorrow, rest of week and past
   deadline in one view [12]. Roji should show only Today (three to five items), with the rest one
   tap away.
5. **Hard deadlines that override boundaries.** Motion will schedule past work hours to hit a
   hard deadline [3]. Roji should never expand the day beyond capacity; it should say "this will
   not fit" and let the user choose.
6. **Feature and price drift.** Users punished Motion for chasing team and agent features and
   for billing surprises [20][23]. Keep the personal Today list the product.

## Sources

1. Motion Help, "What Auto-Scheduling considers" (undated). https://www.usemotion.com/help/time-management/auto-scheduling/reference-auto-scheduling/what-auto-scheduling-considers
2. Motion Help, "How auto scheduling works behind the scenes" (undated). https://www.usemotion.com/help/time-management/auto-scheduling/reference-auto-scheduling/how-auto-scheduling-works-behind-the-scenes
3. Motion Help, "Auto scheduling" (undated). https://www.usemotion.com/help/time-management/auto-scheduling
4. Motion Help, "Task scheduling FAQ" (undated). https://www.usemotion.com/help/project-management/task/task-scheduling-faq
5. Motion Help, "AI Agenda how-to guide" (undated). https://usemotion.com/help/time-management/ai-agenda/ai-agenda-how-to-guide
6. Motion Help, "Status indicators" (undated). https://www.usemotion.com/help/time-management/auto-scheduling/reference-auto-scheduling/status-indicators
7. Motion Help, "Free vs busy tasks in Motion" (undated); sync direction from search summary of "Show tasks on external calendars". https://www.usemotion.com/help/project-management/task/reference-tasks/free-vs-busy-tasks-in-motion
8. Motion Help (legacy), "Understanding the Resolve Issues Box" (now redirects; search snippet only, unverified). https://help.usemotion.com/product-and-feature-guides/calendar/tasks/understanding-the-resolve-issues-box
9. Motion homepage (read 2026-11-24). https://www.usemotion.com/
10. Motion pricing page (read 2026-11-24, undated). https://www.usemotion.com/pricing
11. Motion blog, "Introducing Motion's AI Tier", Harysh Menon, 2025-04-21. https://www.usemotion.com/blog/ai
12. Motion blog, "The new AI Agenda", Luis Carrasco, 2025-05-26. https://www.usemotion.com/blog/ai-agenda
13. Motion blog, "Declutter your day with Agenda View", 2024-10-25. https://www.usemotion.com/blog/agenda-view
14. Motion Help, "AI Agenda: purpose" (undated). https://usemotion.com/help/time-management/ai-agenda/concept-ai-agenda/purpose
15. Motion Help, "AI Agenda" (undated). https://www.usemotion.com/help/time-management/ai-agenda
16. Motion Help, "AI Notetaker how-to guide" (undated). https://www.usemotion.com/help/knowledge-management/ai-notetaker/ai-notetaker-how-to-guide
17. TechCrunch, "Y Combinator-backed Motion raises fresh $38M to build the Microsoft Office of AI agents", Julie Bort, 2025-09-08. https://techcrunch.com/2025/09/08/y-combinator-backed-motion-raises-fresh-38m-to-build-the-microsoft-office-of-ai-agents
18. Motion, "/ai-employees" page (read 2026-11-24; now shows generic AI tools). https://www.usemotion.com/ai-employees
19. Motion Help sitemap (read 2026-11-24; no AI Employees pages listed). https://www.usemotion.com/help/sitemap.md
20. Trustpilot, Motion reviews (3.7 from 579 reviews; quoted reviews Feb to Sep 2026). https://www.trustpilot.com/review/www.usemotion.com
21. ChoosingTherapy, "Motion App Review", Lena Suarez-Angelino, LCSW, 2024-09-06, updated 2026-08-03. https://www.choosingtherapy.com/motion-app-review/
22. Morgen (competitor), "Motion Review (2026)", Kamila Olexa, 2026-04-30. https://www.morgen.so/blog-posts/motion-review
23. Carly (competitor), "Motion AI Review (2026): Is Auto-Scheduling Worth It?", 2026-09-28; relays the Reddit posts in [24]. https://www.usecarly.com/blog/motion-ai-review/
24. r/UseMotion threads as linked by [23]; Reddit blocked direct fetching, so quotes are unverified:
    https://www.reddit.com/r/UseMotion/comments/1ofnc7a/why_im_considering_to_leave_motion/ (Oct 2025);
    https://www.reddit.com/r/UseMotion/comments/1olmecj/frustrated_with_motions_priorities_ai_employees/ (Nov 2025);
    https://www.reddit.com/r/UseMotion/comments/1ppxotq/ai_employees_have_been_deprecated/ (Dec 2025);
    https://www.reddit.com/r/UseMotion/comments/1q4dic0/motion_after_2_years/ (Jan 2026);
    https://www.reddit.com/r/UseMotion/comments/1qnj93j/deleting_my_motion_ai_account/ (Jan 2026);
    https://www.reddit.com/r/UseMotion/comments/1qyu8bq/from_motion_to_reclaim_or_clickup/ (Feb 2026);
    https://www.reddit.com/r/UseMotion/comments/1utitgn/does_motion_still_receive_meaningful_updates_or/ (Jul 2026).
25. Codegen (competitor), "Motion alternatives" (claim about r/productivity seen only in a search snippet; unverified). https://codegen.com/alternatives/motion/

Not reached: YouTube transcripts (pages returned only navigation; e.g. "Motion Scheduling App
Review (easy to use? too complex?)", https://www.youtube.com/watch?v=pzw_94tcOd0, channel not
readable), and G2 and Capterra (HTTP 403). Their sentiment appears here only through [21][22].
