# Agent frameworks working with a task app over MCP

Research for issue #50 (map #43). Question: what can an outside agent (Hermes, OpenClaw,
Claude) do against a task app exposed over MCP, what do existing to-do MCP servers expose, and
how does that compare with calling a model directly from the app through a provider-agnostic
layer? Facts and trade-offs only; the decision on where Roji's AI lives is made later.

Sources were read on 2026-11-24. Each claim links to the page that owns it.

## 1. MCP as of the current spec (2026-07-28)

- A server offers three primitives: **tools** (model-controlled actions), **resources**
  (application-controlled context data) and **prompts** (user-invoked templates).
  [Server overview](https://modelcontextprotocol.io/specification/2025-06-18/server)
- The current revision is **2026-07-28**. It makes MCP stateless: no `initialize` handshake, no
  `Mcp-Session-Id`; every request carries version and capabilities in `_meta`, and servers must
  implement `server/discover`. Cross-call state goes in server-minted handles passed as tool
  arguments. [Changelog](https://modelcontextprotocol.io/specification/2026-07-28/changelog)
- **Sampling** (a server asking the client's model to generate text) and **Roots** are now
  *deprecated*; the suggested migration for sampling is "integrate directly with LLM provider
  APIs". Server-to-client requests (elicitation) now use the Multi Round-Trip Request pattern.
  [Changelog, Deprecated §1 and Major §7](https://modelcontextprotocol.io/specification/2026-07-28/changelog)
- **Tasks** (long-running, pollable operations) moved out of core into the official extension
  `io.modelcontextprotocol/tasks`. Change notifications go over one `subscriptions/listen`
  stream. [Changelog, Major §4 and §6](https://modelcontextprotocol.io/specification/2026-07-28/changelog)
- Streamable HTTP is the remote transport; HTTP+SSE is deprecated. OAuth client registration
  moves from Dynamic Client Registration (deprecated) to Client ID Metadata Documents.
  [Changelog](https://modelcontextprotocol.io/specification/2026-07-28/changelog)
- The spec puts consent on the host: hosts must get explicit user consent before invoking any
  tool, and tool annotations from untrusted servers are untrusted.
  [Specification, Security](https://modelcontextprotocol.io/specification/latest)

**Consequence for Roji:** an MCP server can't count on borrowing the caller's model (sampling is
on its way out). If Roji itself needs a model (sorting, suggestions, email parsing), it calls a
provider directly either way. MCP is the door for *outside* agents, not a substitute for an
in-app model connection.

## 2. What outside agents can do against an MCP task server

| Capability | Hermes Agent (Nous Research) | OpenClaw | Claude (claude.ai / Claude Code) |
|---|---|---|---|
| MCP client | Yes: stdio and HTTP; bearer headers or OAuth 2.1; include/exclude tool filters; wraps resources and prompts as tools | Yes: Streamable HTTP, SSE, stdio; tool include/exclude; can also *serve* MCP | Yes: remote MCP "custom connectors", OAuth or fixed headers |
| Runs on a schedule | Built-in cron (natural language or cron syntax), run by its gateway daemon; each run is a fresh agent session | Automations (cron jobs, webhooks, one-shot) | Claude Code scheduled tasks (research preview) on schedule, API or GitHub triggers, in Anthropic's cloud, with connectors |
| Reacts to events from the task app | Not documented | **MCP Events**: a remote MCP server sends signed webhooks that start an automation | API trigger on a scheduled task (HTTP POST) |
| Reads mail | Email gateway (IMAP/SMTP) is a chat channel to the agent; Google Workspace skill (Gmail, Calendar, Drive) and Himalaya skill manage a mailbox | IMAP trigger: allowlisted senders only, each mail goes to a sandboxed reader with minimal tools | Via whatever connectors the user adds (no built-in mailbox reading in the sources read) |
| Prepares context (web, subagents) | `web_search`, `web_extract`, browser tools, `delegate_task` subagents, persistent memory | Sub-agents, tools, plugins | Web search and connectors in chat; full Claude Code tooling in scheduled tasks |
| Where it runs | Self-hosted (VPS, homelab), Python, MIT | Self-hosted gateway, MIT | Anthropic cloud; server must be reachable from Anthropic's IPs on port 443 |
| Models | Nous Portal, OpenRouter, OpenAI or any endpoint | Configurable providers, failover, local models | Claude only |

Sources: Hermes [MCP](https://hermes-agent.nousresearch.com/docs/user-guide/features/mcp),
[cron](https://hermes-agent.nousresearch.com/docs/user-guide/features/cron),
[email](https://hermes-agent.nousresearch.com/docs/user-guide/messaging/email),
[tools](https://hermes-agent.nousresearch.com/docs/user-guide/features/tools),
[Google Workspace skill](https://raw.githubusercontent.com/NousResearch/hermes-agent/main/skills/productivity/google-workspace/SKILL.md),
[overview](https://hermes-agent.nousresearch.com/docs/),
[repo (MIT)](https://github.com/NousResearch/hermes-agent).
OpenClaw [overview](https://docs.openclaw.ai/), [MCP](https://docs.openclaw.ai/tools/mcp),
[automation](https://docs.openclaw.ai/automation),
[MCP Events](https://docs.openclaw.ai/automation/mcp-events),
[IMAP](https://docs.openclaw.ai/automation/imap), [repo (MIT)](https://github.com/openclaw/openclaw).
Claude [custom connectors](https://support.claude.com/en/articles/11175166-getting-started-with-custom-connectors-using-remote-mcp),
[scheduled tasks](https://code.claude.com/docs/en/routines).

Notes that matter for Roji:

- **All three can add and read tasks** through a Roji MCP server as soon as it exposes tools.
  Scheduling and mail live in the agent, not in MCP: the agent runs on its own timer, reads mail
  with its own credentials, and writes the result into Roji through tools.
- **Hermes and OpenClaw are self-hosted** and can reach a Roji server on the homelab LAN. Claude's
  connectors connect *from Anthropic's cloud*, so a Roji server would have to be reachable from
  the public internet (or have Anthropic's IP ranges allowlisted) over HTTPS on 443.
- **Mail handling is guarded** in both self-hosted agents: sender allowlists, sender
  authentication (Hermes checks `Authentication-Results`; OpenClaw defaults to DMARC pass), and
  OpenClaw's reader runs sandboxed with web, shell and cron tools denied. Neither "scans the whole
  inbox for tasks" out of the box; that would be a cron job using the Google Workspace or
  Himalaya skill (Hermes) or a custom automation.
- **Push from Roji to an agent** exists only in OpenClaw (MCP Events: signed webhooks, at-least-once,
  untrusted payloads) and Claude (POST to a scheduled task's API trigger). Hermes is pull-only in
  the docs read.
- **Prompt injection:** OpenClaw's docs say event payloads are "untrusted data, separate from the
  instructions the user authorized". Task text and email bodies flowing through Roji are the same
  kind of data.

## 3. What existing to-do MCP servers expose

**Todoist (official, first-party).** Remote server at `https://ai.todoist.net/mcp`, Streamable
HTTP, OAuth. "Read, create, and update your tasks & projects."
[Todoist MCP docs](https://developer.todoist.com/api/v1/todoist-mcp.md). The open-source server
([todoist/todoist-mcp](https://github.com/todoist/todoist-mcp), MIT, TypeScript; `src/tools`) has
about 50 tools, grouped:

- CRUD per object: `add-`/`find-`/`update-` for tasks, projects, sections, comments, labels,
  filters, folders, reminders; `delete-object`, `fetch-object`
- Task verbs: `complete-tasks`, `uncomplete-tasks`, `reschedule-tasks`, `find-tasks-by-date`,
  `find-completed-tasks`, `reorder-objects`, `project-move`, `manage-assignments`
- Overviews and insight: `get-overview`, `get-productivity-stats`, `get-project-health`,
  `analyze-project-health`, `find-activity`, `get-workspace-insights`
- Generic `search` and `fetch` (the OpenAI connector convention), `view-attachment`,
  `user-info`, project template import/export

Batch-shaped tools (`add-tasks`, `complete-tasks`) let an agent act on many tasks in one call.

**Linear (official).** `https://mcp.linear.app/mcp`, Streamable HTTP, OAuth 2.1 or API key as
bearer; a separate `/mcp/readonly` endpoint "only ever exposes read tools". Tools find, create
and update issues, projects and comments. [Linear MCP docs](https://linear.app/docs/mcp)

**Things 3 (community, [hald/things-mcp](https://github.com/hald/things-mcp)).** Local stdio
server (macOS only), reads the Things database via Things.py and writes through the Things URL
scheme. Exposes the app's lists (Inbox, Today, Upcoming, Anytime, Someday), projects, areas,
tags, search, recent items, checklists, and creating and updating to-dos.

Other to-do MCP servers on GitHub (todo.txt, in-memory demos) are small hobby projects
([GitHub search](https://github.com/search?q=mcp+server+todo&type=repositories)).

**Patterns worth noting:** remote Streamable HTTP plus OAuth is the norm for hosted apps;
a read-only endpoint or scope is a cheap safety lever; tools mirror the app's own views
(Today, Inbox) as well as raw CRUD; batch tools reduce round trips; none of the three servers
documents resources, prompts or subscriptions as central features, tools carry everything.

## 4. Calling a model directly from the app (provider-agnostic layer)

| | LiteLLM | OpenRouter | Vercel AI SDK |
|---|---|---|---|
| What it is | Python SDK plus self-hosted OpenAI-compatible proxy ("LLM gateway") | Hosted gateway, one OpenAI-compatible endpoint (`https://openrouter.ai/api/v1`) | TypeScript library; unified API for text, structured objects, tool calls, agents |
| Providers | "100+ LLMs" incl. local | "Hundreds" of models, fallbacks, cost-based routing | Many providers via provider packages |
| Self-hosted | Yes | No (hosted service) | It's a library in your app |
| Extras | Fallbacks, virtual keys with budgets, cost tracking, logging to Langfuse etc., an MCP gateway | Per-request or account-wide filter to providers with a given data policy (e.g. zero retention) | MCP *client* (`@ai-sdk/mcp`, HTTP recommended), harness agents (Claude Code, Codex) |
| Licence | MIT (except `enterprise/`) | Commercial service | Apache-2.0 |

Sources: [LiteLLM docs](https://docs.litellm.ai/docs/),
[LiteLLM LICENSE](https://github.com/BerriAI/litellm/blob/main/LICENSE),
[OpenRouter quickstart](https://openrouter.ai/docs/quickstart),
[OpenRouter provider logging](https://openrouter.ai/docs/guides/privacy/provider-logging),
[AI SDK intro](https://ai-sdk.dev/docs/introduction),
[AI SDK MCP tools](https://ai-sdk.dev/docs/ai-sdk-core/mcp-tools),
[AI SDK LICENSE](https://github.com/vercel/ai/blob/main/LICENSE).

These are not exclusive: the AI SDK (or any OpenAI-compatible client) can point at a LiteLLM
proxy or at OpenRouter. The choice of library follows the stack, which isn't chosen yet.

## 5. Trade-offs for "where the AI lives"

**AI inside Roji (direct provider call through an agnostic layer)**

- Roji controls when the model runs (on capture, on a nightly sort, when building Today), what
  it sees and what it may change. Privacy controls ("work tasks may go to a cloud LLM", per-area
  opt-outs, logs) sit in one place.
- Works with no outside agent installed; it's the only route for features that must just happen
  (auto-sorting a capture, parsing an email into a task).
- Costs: Roji owns prompts, retries, structured output, model choice and per-call spend.
- Needed anyway: MCP sampling is deprecated, so Roji can't lean on a connected agent's model.

**AI outside Roji (agents over MCP)**

- Roji only exposes tools; Hermes, OpenClaw or Claude bring the model, schedule, mail access,
  web research and memory. Agent preparation ("gather related emails and research for this
  task") is exactly what these frameworks already do.
- The user's choice of agent and model stays open; Roji doesn't ship mail credentials or a
  browser.
- Costs: behaviour depends on a separate system being set up and running; Roji can't guarantee
  it happens. Every tool call is a write from an agent reading untrusted text (emails, web pages),
  so Roji needs scoped tokens, a read-only mode, and a way to see and undo what an agent did.
  Claude requires a publicly reachable server; the self-hosted agents don't.

**Hybrid** (the charting assumption: rules plus a built-in provider-agnostic model connection,
with MCP on top) fits these facts: in-app model for the core loop, MCP for agents that prepare
context and capture from elsewhere. Open questions for the decision ticket:

1. Which jobs are in-app and which are left to agents (email detection is the borderline case:
   in-app IMAP plus model, or an agent's mail skill writing tasks through MCP)?
2. Does Roji need to push events to agents (webhook to OpenClaw MCP Events or a Claude API
   trigger), or is agent-side polling on a schedule enough for the MVP?
3. Is the homelab server exposed publicly (needed for Claude connectors and phone access) or
   LAN/VPN only (fine for Hermes and OpenClaw)?
4. What tool surface to copy: Todoist-style CRUD plus views (Inbox, Today), batch tools, and a
   read-only scope from day one?
5. Self-hosted gateway (LiteLLM) versus hosted router (OpenRouter) versus direct provider SDKs,
   given that work tasks may go to a cloud LLM and private ones may need to stay home.
