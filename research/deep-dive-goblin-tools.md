# Deep dive: Goblin Tools

Research for issue #69 on the Roji MVP map (#43). Gathered 2026-11-24.

**Question:** What do Goblin Tools' AI helpers do (Magic ToDo breakdown with spiciness,
Compiler from brain dump, Estimator, Formalizer, Judge), how well do they work, and how do
people with ADHD use them? Which helpers fit Roji's capture and built-in AI?

## Short answer

Goblin Tools is a set of nine single-purpose AI helpers "for the neurospicy", built by one
Belgian engineer, Bram De Buyser ([About][about]). Each tool is one text box and one button:
**Magic ToDo** splits a task into steps, **Compiler** turns a ramble into tasks,
**Estimator** guesses a duration, **Formalizer** rewrites tone, **Judge** reads the emotional
tone of a message, **Taskmaster** walks the list one task at a time with a timer, plus
**Consultant** (pros and cons), **Professor** (explain) and **Chef** (recipes). A
**🌶 spiciness** setting (one to five peppers) tells each tool how hard the user finds
something: for Magic ToDo more peppers mean more, smaller steps; for Estimator it shifts the
guess "not as a simple multiplier" ([Magic ToDo][todo], [Estimator][estimator]).

The web site is free without an account; Pro (about $3-4 a month) adds sync, history,
templates and sharing ([About][about], [App Store][appstore]). Requests go to OpenAI and are
not stored for free users ([Privacy][privacy]).

Users with ADHD and autism describe it as life-changing for **task initiation**: the steps are
"already there", so they can start. The common chain is *speak a brain dump into Compiler ->
send to Magic ToDo -> work through it in Taskmaster*. Complaints: the phone app crashes and
loses lists, breakdowns are sometimes generic or excessive, estimates can be far off, and a
2026 switch to subscriptions angered people who had bought the app.

For Roji, the helpers that fit are **Compiler** (capture text becomes tasks, behind a
review-and-undo step), **breakdown on request with a single granularity knob**,
**Estimator** (a suggested duration the user can override) and **Taskmaster's** "suggest the
next task, break it down if stuck" inside the focus view. Formalizer, Judge, Consultant,
Professor and Chef are general writing and chat helpers; outside agents over MCP or any chat
model already do them, so they don't belong in Roji's MVP.

## The tools

All quotes are from each tool's page on goblin.tools.

| Tool | What it does | Spiciness |
| --- | --- | --- |
| Magic ToDo | "a standard todo list, with some special sauce": a button on each item "automatically come[s] up with the steps you need" ([ToDo][todo]) | "how hard or stressful you find the task. The spicier, the more steps"; applies to all items |
| Compiler | "takes a text and tries to distill a list of actionable items from it. You can ramble about anything"; the result is sent "directly to the magic todo list" ([Compiler][compiler]) | none |
| Estimator | "a reasonable timeframe if you give it an activity ... just a rough guess"; also on every Magic ToDo item ([Estimator][estimator]) | "how hard it is for you to concentrate on the activity, or how physically hard you find it" |
| Formalizer | "a style translator or code switcher": 17 tones, e.g. more professional, more polite, less snarky, unwaffle, bullet points ([Formalizer][formalizer]) | how strongly to apply the tone |
| Judge | "tries to figure out what the emotional content of the text is"; can "Suggest a response" and follow a conversation ([Judge][judge]) | none |
| Taskmaster | "walks you through your Magic Todo items one at a time, with a timer based on the task's estimate", "an optional 'what next' suggestion based on deadlines, category and the time of day"; 🪄 breaks down the current task "if you're stuck" ([Taskmaster][taskmaster]) | none |
| Consultant | lists pros and cons, then suggests; "it can be wrong" ([Consultant][consultant]) | n/a |
| Professor, Chef | explain a topic; recipes from what you have ([About][about]) | n/a |

### Magic ToDo in detail ([ToDo][todo])

- **Breakdown is recursive:** each step is itself an item that can be broken down again, so
  the list becomes a tree. Subtasks can also be added by hand.
- **One granularity knob for the whole list.** "You can change this at any time, it'll apply
  to all items", and "it's not an exact measure, more a hint to the system".
- **Automatic category:** "Top level tasks are automatically assigned a category indicated by
  an emoji", and the list can be filtered by category or hide completed items.
- **List actions:** Estimate (all), Prioritize, undo and redo, tabs (several lists), templates,
  share link, and export to Todoist, iCal tasks or events, Markdown, Asana, Notion, Amazing
  Marvin and TickTick.
- No due dates, reminders, recurrence or calendar on the list itself; Taskmaster reads
  deadlines, but the list is a plain checklist. App Store reviewers ask for recurrence and
  widgets ([App Store][appstore], [reviews][rss]).

### Platform, price and privacy

- Web site free "and available to all"; the maker promises "existing features will never be
  locked behind a paywall" ([About][about]).
- iOS and Android apps by Skyhook Belgium, 4.8 stars from 3.1K ratings on iOS
  ([App Store][appstore]); Google Play lists 100K+ downloads ([Play][play]). Version 2.0 (August
  2026) was a native rewrite with Pro accounts and sync; widgets and notifications are
  "planned" ([App Store][appstore]).
- Pro: $3.99 a month or $39.99 a year on iOS; adds continuous sync, templates, list sharing,
  custom Formalizer tones and history across tools ([App Store][appstore]).
- AI: "models from multiple providers, both open and closed source", "ethical models where
  possible" ([About][about]); the privacy policy names OpenAI and says input is sent without
  identity and not stored for free users. Pro stores lists, the last 50 items per tool,
  templates and custom instructions; accounts via Clerk; cookieless analytics
  ([Privacy][privacy]).
- Warnings everywhere: "Nothing returned by any of the tools should be taken as a statement of
  truth, only guesswork" ([Product Hunt][ph]); "It doesn't know your exact situation, it only
  knows what you tell it" ([Consultant][consultant]).

## How well it works, and how people with ADHD use it

Sources: the 150 most recent US App Store reviews (July 2025 to November 2026; 67 five-star,
42 one-star) read from Apple's review feed ([reviews][rss]), Product Hunt ([ph]) and the App
Store page ([appstore]). YouTube review pages ([yt1], [yt2], [yt3]) could not be read (no
transcript reachable), so they are listed but not used as evidence.

### What users love

- **Starting becomes possible.** "It helps me break down everything in my life that would not
  get done otherwise because I struggle with initiation. Having the direction outlined for me
  allows me to just go!"; "I've tried so many times to use the breaking down tasks method ...
  and would stop after writing the steps out because I had zero motivation or energy left to
  do the thing" ([reviews][rss]). The AI does the breakdown so the user's energy goes to the
  task.
- **Seeing all steps helps prioritising and time blindness.** "I could prioritize because I
  could see all the steps"; "I have pretty bad time blindness and also struggle to break down
  tasks ... create healthy expectations for myself" ([reviews][rss]).
- **The capture-to-done chain.** One AuDHD reviewer: "I used voice to text in the Compiler to
  say what I needed to do, sent the list to the Magic To-Do, and used TaskMaster to get it
  done!! ... 14 things done today" ([reviews][rss]).
- **Small and plain.** "Just enough! ... hits the sweet spot between offering plenty and not
  overwhelming"; "No frills, just help"; "helpful without feeling intrusive" ([reviews][rss]).
- **Tone help for messages.** Formalizer for emails and Judge for reading other people's tone
  are the second most praised uses, especially by autistic users ([reviews][rss],
  [appstore]).
- **Free and no sign-up** on the web, recommended by therapists ([reviews][rss]).

### What users dislike

- **Reliability of the phone app** dominates the one-star reviews: crashes on opening Magic
  ToDo (January to April 2026), lists deleted or edits reverted, an iOS 26 layout bug, "a list
  program that doesn't reliably save your changes is not doing its job". Several say the web
  site still works ([reviews][rss]).
- **Pricing change.** People who paid once for the app were asked to subscribe after the
  August 2026 rewrite: "Paid app w/ monthly fee? Yuck" ([reviews][rss]).
- **Breakdown quality.** "the breakdown into tasks seemed excessive" ([ph]); "it breaks it down
  so broadly it's useless", "don't bother trying to plug anything crafty in there"; "spit out
  the same comments over and over only slightly more detailed. It doesn't give you a plan, or
  timelines" ([reviews][rss]). One user wanted to "give the AI more information" about the
  task; one got steps in Swedish ([reviews][rss]).
- **Estimates can be far off:** "two minute to choose a history essay topic? 3.5 hours ... for
  1500 words with citations? I think they just roll dice" ([reviews][rss]); a Product Hunt
  reviewer said time measurement "could be more accurate" ([ph]).
- **A list, not a system:** no reminders, recurrence, dates per step, delegation or widgets;
  adding to the bottom of a long list is hard ([reviews][rss]).
- One reviewer asked the app to drop the word "neurospicy" ([reviews][rss]).

## What fits Roji

Roji's notes call for capture from anywhere, automatic sorting, a short suggested Today, and a
built-in provider-agnostic model connection (#43). Goblin Tools is the clearest evidence that
small, single-purpose AI steps help ADHD users, and also of what goes wrong.

**Take:**

1. **Compiler as a capture path.** A brain dump (typed, shared from the phone, or dictated)
   becomes several tasks. Roji should show the proposed tasks as a draft the user accepts,
   edits or undoes, as with Sunsama's autoplan (#46), rather than dumping them in silently.
   This fits "capture from anywhere" and the email-detected tasks, which are the same job:
   text in, actionable items out.
2. **Breakdown on request, with one knob.** Offer "break it down" on a task and in the focus
   view when stuck (Taskmaster's 🪄), not automatically on every task. Keep spiciness as a
   single, plain setting ("fewer, bigger steps" to "many small steps") that can be changed
   after the fact. Store steps as a checklist inside the task (Tiimo, #47) rather than as new
   tasks, so the backlog doesn't grow. Let the user add context before breaking down.
3. **Suggested estimates the user can override.** Estimator shows that a rough guess is
   welcome and helps time blindness, but it is often wrong. Roji should mark estimates as
   suggestions, learn from the retroactive time log, and never require one (Llama Life, #48).
4. **Automatic category with no setup.** Magic ToDo's emoji category on every top-level task is
   automatic sorting with zero configuration. Roji's areas can be assigned the same way.
5. **Taskmaster's "what next".** One task at a time, a timer from the estimate, and a single
   suggested next task "based on deadlines, category and the time of day" is close to Roji's
   focus view plus Today suggestion.
6. **Plain-language disclaimers and privacy.** Free use sends text to a model without identity
   and stores nothing; that is a fair baseline for Roji's model privacy controls.

**Avoid:**

- Losing data. Goblin Tools' worst reviews are about vanished lists; Roji's capture must be
  durable before any AI runs, and AI output should never overwrite user text without undo.
- Over-splitting: fifteen micro-steps can be as overwhelming as one big task. Default to few
  steps.
- A toolbox of unrelated helpers. Formalizer, Judge, Consultant, Professor and Chef are loved
  but are chat jobs, not task management; leave them to outside agents over MCP.
- Changing what people already paid for.

## Sources

[about]: https://goblin.tools/About
[todo]: https://goblin.tools/ToDo
[compiler]: https://goblin.tools/Compiler
[estimator]: https://goblin.tools/Estimator
[formalizer]: https://goblin.tools/Formalizer
[judge]: https://goblin.tools/Judge
[taskmaster]: https://goblin.tools/Taskmaster
[consultant]: https://goblin.tools/Consultant
[privacy]: https://goblin.tools/Privacy
[appstore]: https://apps.apple.com/us/app/goblin-tools/id6449003064
[rss]: https://itunes.apple.com/us/rss/customerreviews/id=6449003064/sortBy=mostRecent/json
[play]: https://play.google.com/store/apps/details?id=com.goblintools
[ph]: https://www.producthunt.com/posts/goblin-tools
[yt1]: https://www.youtube.com/watch?v=4H5M89uu_8w
[yt2]: https://www.youtube.com/watch?v=pH2dpJkvnOc
[yt3]: https://www.youtube.com/watch?v=GGuPinM_oF8

- [About][about], [Magic ToDo][todo], [Compiler][compiler], [Estimator][estimator],
  [Formalizer][formalizer], [Judge][judge], [Taskmaster][taskmaster],
  [Consultant][consultant], [Privacy policy][privacy] (goblin.tools, read 2026-11-24)
- [App Store listing][appstore] and [App Store review feed, pages 1-3][rss]
- [Google Play listing][play] (figures from search snippets; the page did not render)
- [Product Hunt][ph]
- YouTube, not readable: [ADHD-Friendly App Review][yt1], [Magic To-Do & The Formalizer][yt2],
  [App Review 2026][yt3]
- Not used as evidence: the saner.ai review (the site says it sponsors Goblin Tools).
