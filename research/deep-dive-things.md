# Deep dive: Things 3

Research for [#70](https://github.com/piemkay/roji/issues/70), part of the MVP spec map [#43](https://github.com/piemkay/roji/issues/43). Sources were read on 2026-11-24.

## Question

How do Things 3's areas, projects, headings and Today/This Evening/Upcoming/Someday views work, and what makes its design feel calm? What do users love and dislike, and which ideas fit Roji's grouping and calm design?

## Short answer

Things 3 by Cultured Code is an Apple-only personal task manager sold as a one-time purchase per platform (iPhone $9.99, $6.99 in this week's sale; Mac $49.99, $34.99 in the sale; iPad separate), with free sync through Things Cloud. It has two independent axes. **Where** a to-do belongs: an area (an ongoing role that is never finished), a project (a goal that can be finished, optionally split by headings), or nothing. **When** it comes to your attention: Today (with a This Evening section), Upcoming, Anytime or Someday, driven by a start date that is separate from a deadline. There is no priority field, no AI inside the app and no web API.

The calm comes from what it leaves out (no priorities, few colours, one white page per open to-do, lots of whitespace) and from start dates that hide everything you can't act on yet. Users love the look and the Today list. They dislike the Apple lock-in, the per-platform price, the lack of collaboration and integrations, and, for ADHD users, a Today list that keeps growing because nothing clears it except your own daily review.

For Roji, the parts worth taking are the area/project split, the start-date-versus-deadline split, This Evening as a soft divider inside today, headings only inside projects, area-inherited tags, and the restraint in the visual design. The parts to avoid are the daily Inbox processing and Today review it depends on, the manual Someday review, and the lack of any automatic decay of old tasks.

## How it works

### Two axes: context and time

The support docs describe two ways of looking at to-dos: by context (areas and projects you create) and by when you plan to do them ([An In-Depth Look at Today, Upcoming, Anytime, and Someday](https://culturedcode.com/things/support/articles/4001304/)). The default time lists "can't be renamed, removed, or reordered" ([Scheduling To-Dos](https://culturedcode.com/things/support/articles/2803579/)).

### Areas, projects, headings, checklists

- **Area:** "for grouping all of your projects and to-dos that support an ongoing ambition", with examples like Family & Friends, Money, Health, Career: "There's an area for every hat you wear" ([Getting Productive with Things](https://culturedcode.com/things/guide/)). "Unlike projects, areas are ongoing and can't be completed." Areas can hold loose to-dos directly as well as projects.
- **Split areas, filter with tags:** Cultured Code recommends narrow areas ("Think 'roles I actively maintain' rather than every interest") over broad Work/Personal. "You can assign a tag to an entire area, and every to-do inside will inherit it automatically", so all work areas can share a `Work` tag and Today or Upcoming can be filtered to it ([Split Up Areas](https://culturedcode.com/things/support/articles/6037627/)).
- **Project:** anything that takes more than one step ([guide](https://culturedcode.com/things/guide/)). Projects can be completed, can have start dates (hidden from the sidebar until then) and deadlines, and can be sent to Someday.
- **Heading:** splits a long project into parts such as phases or categories. "Headings are only available in projects. This feature is not available in areas or any other list." A heading can be moved or duplicated with its to-dos, converted into its own project, or archived when done, which moves it to the logged items at the bottom of the project ([Using Headings](https://culturedcode.com/things/support/articles/2803577/)).
- **Checklist:** steps inside one to-do. "If a to-do with a checklist becomes unwieldy, just convert it to a project." The same article advises planning only the next one to three steps ([Why Some To-Dos Get Stuck](https://culturedcode.com/things/support/articles/8491676/)).
- **No priority field.** Prioritising means "adding them to your Today list" and dragging them into order; for long lists, use tags and "start off simple by creating a single tag, for example Important" ([How to Prioritize](https://culturedcode.com/things/support/articles/3289315/)).

### The time lists

| List | What lands there | Source |
| --- | --- | --- |
| Inbox | Unprocessed captures from Quick Entry, Mail to Things, share sheet. "A temporary staging ground for your unprocessed thoughts"; review it daily. | [Default lists](https://culturedcode.com/things/support/articles/4001304/), [guide](https://culturedcode.com/things/guide/) |
| Today | To-dos "you want to start before the day ends. They're your priorities." Anything whose start date, deadline or repeat rule matches today appears here. Calendar events show at the top. Can be grouped by project or area when long. | [Default lists](https://culturedcode.com/things/support/articles/4001304/) |
| This Evening | A section at the bottom of Today for things you'll only do later. "Unobtrusive enough to not bother you until you have time." | [Default lists](https://culturedcode.com/things/support/articles/4001304/) |
| Upcoming | Future start dates, repeats and deadlines; the next seven days are listed separately, then further out. Items move into Today when their date arrives. Drag to reschedule. | [Default lists](https://culturedcode.com/things/support/articles/4001304/), [features](https://culturedcode.com/things/features/) |
| Anytime | Every active to-do not blocked by a future start date, loose ones first, then grouped by project and area. Today's items carry a star. Deadline items stay here. | [Default lists](https://culturedcode.com/things/support/articles/4001304/) |
| Someday | Projects and to-dos with "no plan for it yet". They are hidden from Anytime and Upcoming; review "every couple of months". | [Default lists](https://culturedcode.com/things/support/articles/4001304/) |
| Logbook | Completed and cancelled items, kept indefinitely and searchable. | [Default lists](https://culturedcode.com/things/support/articles/4001304/) |

### Start date versus deadline

A start date ("When") "merely indicates the day on which you want to begin work on a to-do"; a deadline (the flag) is when it must be finished, usually from outside pressure such as a bill. For multi-day work, set a start date a few days before the deadline. Reminders are separate again: a time-of-day notification, "a gentle nudge" ([Scheduling To-Dos](https://culturedcode.com/things/support/articles/2803579/)). Quick Find exposes extra lists such as Tomorrow and Deadlines.

The docs don't say what happens to an unfinished to-do at the end of the day beyond "you can easily reschedule the remaining items for another day". In practice a past start date keeps it in Today, so Today only shrinks when you clear it yourself (not confirmed by the docs; see the user reports below).

### Daily routine the app assumes

The guide asks for a short daily ritual: visit Today "early in the day, every day", see what's new, decide what to do, postpone the rest, and order what remains; also review the Inbox daily, look at areas "every now and then", and review Someday regularly ([guide](https://culturedcode.com/things/guide/)). There is no automatic triage or rollover handling.

### Capture and integrations

- **Quick Entry** (Mac, Ctrl+Space) from any app; **Quick Entry with Autofill** adds a link to the current web page, email or file to the to-do's notes ([Quick Entry with Autofill](https://culturedcode.com/things/support/articles/2249437/)).
- **Natural language dates** while typing ("tom" for tomorrow) ([features](https://culturedcode.com/things/features/)).
- **Mail to Things:** a private address that turns an email into an Inbox to-do; needs Things Cloud ([Mail to Things](https://culturedcode.com/things/support/articles/2908262/)).
- **Calendar:** read-only display of Apple Calendar events in Today and Upcoming, local per device; Things can't write to-dos into the calendar ([Calendar Integration](https://culturedcode.com/things/support/articles/2803583/)).
- **Automation:** Apple Shortcuts, Siri, and a `things:///` URL scheme with an auth token for changes ([URL Scheme](https://culturedcode.com/things/support/articles/2803573/)). There is no web API.
- **AI:** none built in apart from Apple's Siri AI integration (Things for Mac 3.24, September 2026) ([Mac release notes](https://culturedcode.com/things/support/articles/1100684/)). There is no official MCP server or ChatGPT/Claude connection; Cultured Code warns that third-party tools that write to the database directly or ask for Things Cloud credentials are unsafe ([Third-Party AI Tools](https://culturedcode.com/things/support/articles/5510170/)).

### Platform and price

iPhone/Watch, iPad, Mac and Vision Pro apps are bought separately; Things Cloud sync is free; only the Mac has a trial (15 days) ([App Store, iPhone](https://apps.apple.com/us/app/things-3/id904237743); [App Store, Mac](https://apps.apple.com/us/app/things-3/id904280696); [Trying Things](https://culturedcode.com/things/support/articles/2803551/)). There is no Android, Windows or web version. It is actively maintained: Mac 3.25 shipped on 2026-11-24.

## What makes it feel calm

- **Leaving things out.** No priority levels, no red overdue lists, no project percentages in the list, no gamification. Prioritising is just "put it in Today and drag it into order".
- **Hiding what you can't act on.** Start dates keep future work in Upcoming and Someday until it matters, and projects with future start dates disappear from the sidebar.
- **A soft divider inside today.** This Evening splits the day without a second list or time blocks.
- **Visual restraint.** MacStories: "The app is dominated by white space", "simple, but beautiful in its simplicity," with "bold fonts, lovely icons, and thoughtful splashes of color" ([MacStories, 2017](https://www.macstories.net/reviews/things-3-beauty-and-delight-in-a-task-manager/)). An open to-do becomes a plain white page with details (tags, checklist, deadline) tucked into a corner; Slim Mode hides the sidebar ([features](https://culturedcode.com/things/features/)). Reviewer quotes on the site: it "never feels messy or overbearing, no matter the length of your task list", "a clean, crisp piece of paper" ([culturedcode.com/things](https://culturedcode.com/things/)).
- **Satisfying, quiet interactions.** The Magic Plus button can be dragged to where the new to-do or heading should go, or onto a day in Upcoming ([MacStories](https://www.macstories.net/reviews/things-3-beauty-and-delight-in-a-task-manager/)).
- **Recognition:** two Apple Design Awards, Editors' Choice, App Store Awards 2026 finalist for Mac App of the Year ([culturedcode.com/things](https://culturedcode.com/things/)).

## What users say

YouTube could not be fetched from this environment (the search page returned no listings), so no video reviews are used as evidence. The sources below are the App Store, a Mac power-user forum and editorial reviews.

**What they love**

- **Simplicity and look.** "Wonderfully simple and clarifying"; scheduling was "the real game changer for me" ([App Store, iPhone](https://apps.apple.com/us/app/things-3/id904237743), 4.8 stars from 28K US ratings). "The visual density of projects and tasks is calibrated much better than other to-do apps" ([App Store, Mac](https://apps.apple.com/us/app/things-3/id904280696)).
- **ADHD users like the Today list.** "Things is the best I have found for me personally" (Mac App Store reviewer with ADHD). On MacPowerUsers: "Things and TickTick are great at providing this needed daily list" ([MPU thread](https://talk.macpowerusers.com/t/for-people-with-add-adhd-how-do-you-make-task-management-software-work-for-you/27823)).
- **No subscription.** TechRadar lists "cost-effective (one-time fees)" and the "stellar user interface" as the pros ([TechRadar](https://www.techradar.com/reviews/things-3), updated May 2025).

**What they dislike**

- **Today grows.** "Things' daily list starts getting incredible long and it's also very difficult to use to manage complex projects" (same MPU thread). Nothing trims Today but the user.
- **Apple only, no web.** No Android, Windows or web app (TechRadar and most reviews).
- **Per-platform pricing.** Mac, iPhone and iPad bought separately; "still way overpriced" (App Store reviews).
- **Solo only, few integrations.** No shared projects or assignment ([TechRadar](https://www.techradar.com/reviews/things-3)); no web API, so automation is "severely limited" ([MacStories](https://www.macstories.net/reviews/things-3-beauty-and-delight-in-a-task-manager/)).
- **Rigid structure in places.** Areas model confusing to one reviewer; requests for more flexible headings and other Today layouts (App Store, Mac); no file attachments (MacStories).

## Fit for Roji

**Take**

- **Area versus project.** An area is an ongoing part of life that never completes (work, family, health); a project is a finishable goal. This is a clean grouping model for Roji's work-and-private scope, and areas are the natural unit for per-area model privacy opt-outs.
- **Start date versus deadline.** "When it should surface" and "when it must be done" are different fields. Roji's suggester needs both, and hiding not-yet-actionable tasks is a large part of the calm.
- **Hide what isn't actionable.** Future-dated and Someday items stay out of sight until they matter. This fits "no ever-growing backlog" as long as Roji, not the user, decides what resurfaces.
- **This Evening as a soft divider.** A low-key section at the bottom of today, similar to Llama Life's "enough for now" divider, without time blocks.
- **Headings only inside projects.** One level of sub-grouping, only where it helps; a heading can be promoted to a project.
- **Tags inherited from areas** for cross-cutting filters (all work areas share `Work`), so the user tags rarely.
- **No priority field.** Order and "is it in Today" carry priority; at most one "important" marker.
- **Visual restraint:** whitespace, few colours, one plain page per open task with details tucked away, no red overdue badges.
- **Read-only calendar at the top of Today**, as Roji already plans.

**Avoid**

- **Depending on daily rituals.** Things works only if you process the Inbox daily, review Today every morning and look at Someday every couple of months. Roji should sort captures automatically and build Today itself.
- **A Today list that only grows.** Unfinished items stay until you move them. Roji needs automatic rollover with decay (compare Sunsama's auto-archive).
- **Manual reschedule as the only answer to "didn't finish".**
- **No API, Apple only.** Roji's capture-from-anywhere, MCP and email detection need an open API from day one.

**Open questions for later tickets**

- Are areas user-defined like Things, or does Roji start with a few (Work, Private) and suggest splits?
- Does Roji expose Someday as a list at all, or is it the place auto-archived tasks go?
- Does the user ever set a start date, or does Roji infer it from text and email ("next week", "after the 5th")?
