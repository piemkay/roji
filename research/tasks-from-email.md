# Reading a mailbox to find tasks in email

Research for issue #52 (map #43). Question: how can a self-hosted, single-user Roji read a
user's mail to propose tasks (Gmail API, Microsoft Graph, IMAP: auth, push versus polling,
scopes, cost, verification hurdles for a personal Google app)? How do existing tools detect
tasks in email and handle uncertain ones? Which privacy questions must a decision settle?

Sources were read on 2026-11-24. Claims are cited inline; anything marked **inference** is our
reading, not a statement in the source.

## Answer in brief

- **Every route works for one person, but each has a recurring chore.** Gmail API in Google's
  "Testing" mode loses its refresh token every 7 days. An unverified app "In production" avoids
  that but shows an "unverified app" screen. IMAP with a Google app password is the simplest, but
  it grants full mailbox access. Microsoft Graph has 90-day sliding refresh tokens and needs no
  verification for one user.
- **Push is optional and awkward on a homelab.** Gmail push goes through Google Cloud Pub/Sub
  (a *pull* subscription needs no public endpoint). Graph webhooks need a public HTTPS URL.
  IMAP IDLE is a long-lived outbound connection. Polling an incremental-sync cursor every few
  minutes (Gmail `history.list`, Graph `delta`, IMAP CONDSTORE/QRESYNC) covers a to-do app.
- **No mainstream tool silently turns email into tasks.** The dominant pattern is that the user
  marks an email (star, label, flag, drag) and it becomes a task. Where AI judges an email, its
  output shows up as a suggestion or label the user can ignore or undo, never a committed task.
  No product we found documents a confidence threshold or how it treats uncertain cases.

## 1. Gmail API

### Scopes

Every scope that can read message content is **restricted**: `gmail.readonly`, `gmail.metadata`
(headers and labels only), `gmail.modify` and `https://mail.google.com/`. Only `gmail.labels` is
non-sensitive. Restricted scopes need restricted-scope verification, and "if you store restricted
scope data on servers (or transmit), then you must go through a security assessment."
([Gmail API scopes](https://developers.google.com/workspace/gmail/api/auth/scopes))

So "read only metadata" is no shortcut: `gmail.metadata` is as restricted as `gmail.readonly`.

### Verification and the personal-use paths

- **Exceptions.** No verification is needed for "personal use", where "you are the only user of
  your app or if your app is used by only a few users, all of whom are known personally to you",
  nor for apps in "development, testing, or staging", nor for "internal use only" (people in your
  own Google Workspace or Cloud Identity organization, Internal user type).
  ([Restricted scope verification](https://developers.google.com/identity/protocols/oauth2/production-readiness/restricted-scope-verification))
- **If you did verify** (not needed for Roji): an annual security assessment by a
  Google-empanelled assessor under the App Defense Alliance CASA framework, repeated "at least
  every 12 months". The page does not state the cost.
  ([same page](https://developers.google.com/identity/protocols/oauth2/production-readiness/restricted-scope-verification))
- **User type.** A personal `@gmail.com` account can only use the **External** type. **Internal**
  requires the project to belong to a Google Cloud Organization, and only its members can
  authorize. ([Manage app audience](https://support.google.com/cloud/answer/15549945))

The two External publishing states:

| | Testing | In production, unverified |
| --- | --- | --- |
| Who can authorize | Up to 100 listed test users | Anyone; 100 new users in total once the unverified screen shows |
| Warning screen | Yes, before consent | "Unverified app" screen for sensitive or restricted scopes |
| Refresh token | "Authorizations by a test user will expire seven days from the time of consent", and the refresh token with them | The 7-day rule is stated only for Testing |

Sources: [Manage app audience](https://support.google.com/cloud/answer/15549945),
[Unverified apps](https://support.google.com/cloud/answer/7454865),
[OAuth 2.0 overview, refresh token expiration](https://developers.google.com/identity/protocols/oauth2)
("A Google Cloud Platform project with an OAuth consent screen configured for an external user
type and a publishing status of 'Testing' is issued a refresh token expiring in 7 days", unless it
requests only name, email and profile scopes).

**Inference:** for one person on a homelab, "In production, unverified" under the personal-use
exception is the practical setting. You click through the warning once and the token does not
lapse weekly. The sources say only that the 7-day expiry applies to Testing; confirm in a spike
that a production unverified token survives past 7 days. Testing works, but means re-consenting
every week, which would look like "email capture silently stopped".

Other token limits: 100 refresh tokens per Google Account per client ID, and the oldest is
invalidated "without warning" when a new one is issued.
([OAuth 2.0 overview](https://developers.google.com/identity/protocols/oauth2))

### Getting new mail: push or poll

- **Push** needs a Cloud Pub/Sub topic. Grant `gmail-api-push@system.gserviceaccount.com` the
  right to publish, then call `users.watch` (optionally `labelIds: ["INBOX"]`). "You must call the
  watch method at least once every 7 days" (Google recommends daily). The notification carries
  only `emailAddress` and `historyId`; you then call `history.list`. The rate limit is one event
  per second per user, and notifications "can occasionally be delayed or dropped", so keep
  periodic `history.list` as a fallback. Pub/Sub can deliver by **push** (HTTP POST to your
  webhook) or by **pull** (your app fetches and acknowledges). Pull needs no inbound endpoint.
  ([Gmail push notifications](https://developers.google.com/workspace/gmail/api/guides/push))
- **Poll**: call `history.list` from the last stored `historyId`. Same guide.

### Cost

The Gmail API Standard Tier is "enabled by default at no cost", and Standard Tier usage never
generates charges. Quota: 6,000 units per user per minute. Unit costs: `history.list` 2,
`messages.list` 5, `messages.get` 20 (60 with `format=full/raw`), `users.watch` 100. A paid
Scaled Tier starts on 1 April 2027 and is only for higher limits.
([Gmail API usage limits](https://developers.google.com/workspace/gmail/api/reference/quota))
**Inference:** polling one mailbox every minute with `history.list` costs a tiny fraction of
the quota. Pub/Sub pricing was not checked; push would add a Google Cloud billing dependency.

### Google's data policy

The Limited Use rules for sensitive and restricted scope data: use it only "to providing or
improving user-facing features", no transfers except listed exceptions, and "don't allow humans
to read the data" without affirmative user agreement.
([API Services User Data Policy](https://developers.google.com/terms/api-services-user-data-policy))
Google's Workspace blog states that transferring Workspace user data to generalized AI/ML models
is prohibited, and that developers must commit not to keep it to train non-personalized models.
([Workspace blog, API policy protections](https://workspace.google.com/blog/ai-and-machine-learning/api-policy-protections?hl=en))
**Inference:** sending an email to a cloud LLM for inference, under terms that forbid training
on it, fits "user-facing feature". Sending it to a provider that trains on inputs would not. The
same rules hold for a self-hosted, personal-use app; they apply whether or not the app is verified.

## 2. Microsoft Graph (Outlook.com and Microsoft 365)

- **Permissions.** Delegated `Mail.Read` ("Read user mail") and `Mail.ReadBasic` (everything
  except `body`, `previewBody`, attachments and extended properties) both have
  AdminConsentRequired = **No**, and both are "available for consent in personal Microsoft
  accounts". ([Graph permissions reference](https://learn.microsoft.com/en-us/graph/permissions-reference))
  `Mail.ReadBasic` is a real option for a "subject and sender only" mode, unlike Gmail's metadata
  scope, which is still restricted.
- **Verification.** Publisher verification is free but needs a Microsoft AI Cloud Partner Program
  account. It matters when users in *other* tenants consent to a multitenant app with risk-based
  step-up consent on. ([Publisher verification](https://learn.microsoft.com/en-us/entra/identity-platform/publisher-verification-overview))
  **Inference:** irrelevant for a personal Outlook.com account. For a work Microsoft 365 mailbox,
  the employer's tenant consent policy decides, and it may block a self-registered app.
- **Tokens.** Refresh tokens last 90 days by default (24 hours for single-page apps), and
  "replace themselves with a fresh token upon every use", so a server that refreshes regularly
  stays signed in. ([Refresh tokens](https://learn.microsoft.com/en-us/entra/identity-platform/refresh-tokens))
- **Push.** Change notifications by webhook, Event Hubs or Event Grid. Message subscriptions live
  at most 10,080 minutes (under 7 days), or 1,440 minutes with resource data. Average latency is
  under 1 minute, at most 3 minutes.
  ([Change notifications overview](https://learn.microsoft.com/en-us/graph/change-notifications-overview))
  A webhook needs a publicly reachable HTTPS endpoint, so a homelab needs a tunnel or reverse proxy.
- **Poll.** Delta query on `/me/mailFolders/{id}/messages/delta`, one folder at a time, with
  `changeType=created` to get only new mail; save the `@odata.deltaLink` for the next round.
  ([Delta query for messages](https://learn.microsoft.com/en-us/graph/delta-query-messages))

## 3. IMAP

- **Gmail over IMAP** needs scope `https://mail.google.com/` with XOAUTH2
  ([Gmail XOAUTH2](https://developers.google.com/workspace/gmail/imap/xoauth2-protocol)). That is
  the broadest restricted scope, so the verification analysis above still applies. The
  alternative is an **app password**: only with 2-Step Verification on, not for Advanced
  Protection or (often) work accounts, and revoked when the Google password changes.
  ([App passwords](https://support.google.com/accounts/answer/185833))
  **Inference:** an app password is the lowest-friction self-hosted path, but it gives full
  read/write/delete mailbox access. Roji then holds a credential worth guarding.
- **Microsoft over IMAP**: Basic auth "is now disabled in all tenants" of Exchange Online and
  cannot be re-enabled.
  ([Basic auth deprecation](https://learn.microsoft.com/en-us/exchange/clients-and-mobile-in-exchange-online/deprecation-of-basic-authentication-exchange-online))
  OAuth for IMAP (`https://outlook.office.com/IMAP.AccessAsUser.All` plus `offline_access`) works
  for both Microsoft 365 and Outlook.com.
  ([IMAP OAuth](https://learn.microsoft.com/en-us/exchange/client-developer/legacy-protocols/how-to-authenticate-an-imap-pop-smtp-application-by-using-oauth))
  Graph is the better path for Microsoft.
- **Push** is IDLE: one long-lived connection per folder. "Clients using IDLE are advised to
  terminate the IDLE and re-issue it at least every 29 minutes".
  ([RFC 2177](https://datatracker.ietf.org/doc/html/rfc2177))
- **Efficient resync**: CONDSTORE/QRESYNC give a reconnecting client only what changed since its
  last MODSEQ, plus expunged UIDs. ([RFC 7162](https://datatracker.ietf.org/doc/html/rfc7162))
- **Benefit**: one protocol covers any other provider (Fastmail, Posteo, a self-hosted server),
  with no verification and no cost.

## 4. Comparison for a self-hosted, single user

| | Gmail API | Microsoft Graph | IMAP |
| --- | --- | --- | --- |
| Narrowest read scope | `gmail.metadata` (restricted) or `gmail.readonly` | `Mail.ReadBasic` / `Mail.Read`, no admin consent | Whole mailbox |
| Personal path | Own GCP project; Testing = 7-day tokens; prod unverified = warning screen | Own app registration, no verification | Gmail app password, or OAuth as left |
| Push | Pub/Sub, `watch` renewed under 7 days; pull subscription works behind NAT | Webhook needs public HTTPS; renew under 7 days | IDLE, re-issue every 29 min |
| Poll cursor | `history.list` from `historyId` | `delta` link per folder | CONDSTORE / QRESYNC |
| Cost | Free (Standard Tier) | Free | Free |
| Recurring chore | Weekly re-consent (Testing) or warning screen once | Re-consent only if unused for 90 days | Rotate app password when Google password changes |

## 5. How existing tools find tasks in email

| Tool | How an email becomes a task | AI judgement | What it does with uncertain cases |
| --- | --- | --- | --- |
| **Sunsama** | Manual: drag an email into the task list or calendar, or "Add to today"; the subject becomes the title. Automations (mark read, archive) run on import and completion. ([Sunsama Gmail](https://help.sunsama.com/docs/gmail-browsing-and-importing-emails)) | AI reads Gmail only on request (assistant, Autoplan); it does not choose emails. | Not applicable: the user chooses. |
| **Akiflow** | Star or apply the "Akiflow" label; old stars are not imported. Two-way: removing the star or label marks the task done; completing can unstar. Outlook: flagged emails via Microsoft To Do. ([Akiflow Gmail](https://product.akiflow.com/articles/0294053-gmail), [Akiflow Outlook](https://product.akiflow.com/articles/7592768-outlook-email)) | None for selection. | Not applicable. |
| **Microsoft To Do** | A "Flagged email" list mirrors emails flagged in Outlook; title = subject. ([Flagged email](https://support.microsoft.com/en-us/todo/using-microsoft-to-do-with-flagged-email-from-outlook)) | None. | Not applicable. |
| **Gmail** | Manual "Add to Tasks"; tasks can be made "based on information in a Gemini summary card". ([Create a task in Gmail](https://support.google.com/mail/answer/9920317?hl=en)) Gemini creates tasks only when asked ("Add this to my tasks"). ([Gemini tasks](https://support.google.com/gemini/answer/15230285)) **Nudges**: Gmail "nudges" you to reply to mail you may have missed and to follow up on sent mail with no answer; on by default, and users "can turn it off from their Gmail settings menu". ([Workspace Updates, 2018](https://workspaceupdates.googleblog.com/2018/05/gmail-remind-respond.html)) | Yes, for nudges only. | Shown as a hint next to the email, never as a task. |
| **Superhuman** | Auto Reminders resurface "conversations where you don't get a reply"; it "doesn't send email without your permission". ([Superhuman blog](https://blog.superhuman.com/introducing-email-assistant/)) | Yes, follow-up detection. | Not documented; the user can change which mail gets reminders. |
| **Shortwave** | AI filters in natural language can label, star, archive or "update todos". The pre-built "Needs Action" filter "identifies emails requiring your engagement". "Reapply filters" on sample emails shows "exactly what actions the AI takes and why". ([Shortwave settings guide](https://www.shortwave.com/docs/guides/customize-your-shortwave-settings/)) | Yes, the most automatic of the set. | A label the user can see and test; no documented confidence threshold. |

The Superhuman help center returned 403 to our fetcher, so its documented settings are from the
company blog only.

**Pattern** (inference from the table):

1. An explicit user signal (star, label, flag, drag) is the main path into a task list, and it
   is two-way: undoing the signal undoes the task.
2. AI detection is surfaced as a *suggestion* (Gmail nudge, Shortwave label, Superhuman
   reminder) that is visible, reversible and explainable ("why"), not as a committed task.
3. No product documents confidence scores or an "unsure" bucket. The safe equivalent is to show
   everything as a proposal and let ignoring it cost nothing.

**Inference for Roji:** an ADHD-friendly design that avoids a growing backlog points to email
*proposals* that expire if ignored, plus a user signal (a Gmail label or star, an Outlook flag)
that creates a task without a model. The rules-plus-model split already in the map's AI starting
point fits this.

## 6. Privacy questions a decision must settle

1. **Which mailboxes and which folders?** Inbox only, or a label/flag the user sets? Work
   mailboxes may be governed by an employer's admin consent policy (Microsoft tenant, Google
   Workspace).
2. **Which scope?** Headers only (Graph `Mail.ReadBasic`; Gmail has no non-restricted
   equivalent) versus full body. Does Roji need the body to propose a task, or is subject plus
   sender plus snippet enough?
3. **What is stored?** Message IDs and a link back, or copies of bodies? For how long? Is it
   deleted when a proposal is dismissed?
4. **What goes to a model?** The map allows work tasks to go to a cloud LLM. Does that include
   raw email bodies from third parties who never agreed? Is there a local-model or rules-only
   path for private mail? Does the provider's no-training term meet Google's Limited Use rules?
5. **Credential custody.** Refresh tokens or an app password on the homelab: encrypted at rest,
   in backups, revocable. An app password is full mailbox access, including delete.
6. **Write-back.** Does Roji ever change the mailbox (remove a label or star on completion, mark
   read)? That needs `gmail.modify` or `Mail.ReadWrite` instead of read-only scopes.
7. **Inbound exposure.** Push via webhook means a public endpoint on the homelab. Polling or a
   Pub/Sub pull subscription avoids it.
8. **Logs.** Do prompts, model responses and email excerpts land in logs? This ties into the
   map's open "model privacy controls" item.
