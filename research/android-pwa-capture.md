# Installed web app on Android: share target, offline capture, notifications

Research for issue #51 (map #43). Researched 2026-11-24 against Chrome developer docs, web.dev,
MDN and Chromium mailing lists. The question: on current Android Chrome, what can an installed
web app (PWA) do for fast capture, where does it fall short of a native app, and what does that
mean for building web first and native later?

## Short answer

An installed PWA on Android Chrome covers Roji's MVP capture needs. It can sit in the system share
sheet, add long-press shortcuts such as "Quick add", keep captures in a local queue while offline
and send them when the network comes back, and show push notifications. The gaps compared with
a native app: no home-screen widget, no numeric badge, no notifications scheduled on the device
(reminders must come from the server by push), no reliable offline voice input, and no lock-screen
or assistant entry points. The hard requirement for the homelab is **HTTPS with a certificate the
phone trusts**. Without it you get no install, no service worker, no share target and no push.

## Capabilities

| Capability | Works on Android Chrome? | Notes |
|---|---|---|
| Install (WebAPK) | Yes | Real APK minted by Google, in launcher and Settings > Apps [5] |
| Web Share Target | Yes, Chrome 76+ | App must be installed first; one `share_target` per manifest [1] |
| App shortcuts (long-press icon) | Yes | OS decides how many are shown; keep it to a few [2][5] |
| Offline capture queue | Yes | Service worker + IndexedDB, flushed by Background Sync [7][8] |
| Push notifications | Yes | Push API + Notifications API via service worker; needs permission [4] |
| Locally scheduled notifications | **No** | Notification Triggers development ended without shipping [11] |
| Numeric app badge | **No** | Badging API not supported on Android; the OS shows a dot for unread notifications [3] |
| Voice input in the page | Partly | Web Speech API exists; on-device mode not on Android, so it needs the network [9][10] |
| Home-screen widget, lock-screen, assistant | No | No web API; needs a native app |

### Install and the WebAPK

On Android, Chrome sends the PWA to a minting server that builds and signs an APK (a "WebAPK").
The phone installs it silently, and it then "ha[s] an icon in the app launcher and home screen"
and appears in "Settings, Apps". WebAPKs support badging, app shortcuts and OS link capture.
They need Google Mobile Services. If minting fails or the app doesn't meet the criteria,
Chrome falls back to a plain shortcut, which **cannot be a share target** [5].

Installability criteria [6]: served over HTTPS; a manifest with `name` or `short_name`, 192 px and
512 px `icons`, `start_url`, `display` set to `standalone`, `fullscreen`, `minimal-ui` or
`window-controls-overlay`, and `prefer_related_applications` absent or false; and some user
engagement (at least one tap and 30 seconds on the page).

### Share to Roji (Web Share Target)

Add a `share_target` to the manifest with an `action` URL, a `method` and `params` that map the
shared `title`, `text` and `url` to fields [1]:

- **GET** (default) opens a window at the `action` URL with the shared values in the query string.
  This is the simplest choice for text and links.
- **POST** with `multipart/form-data` is needed for shared files (images, screenshots). The
  service worker must intercept it: "The foreground page cannot process this data directly." [1]
- **Android quirk:** the `url` field is often empty because of Android's share system. URLs
  usually arrive in `text`, sometimes in `title`. Parse a URL out of all three [1].
- Only one `share_target` per manifest, so "share as task" vs "share as note" must be a choice
  on the landing page [1].
- The user must install the app before it appears in the share sheet [1].

For offline sharing, precache the `action` page so it opens without a network [1]. With POST,
the service worker can write the share straight into the offline queue (below) and answer with a
redirect, so a share never fails because the homelab is unreachable.

### Home-screen shortcuts

The manifest `shortcuts` member lists entries with `name`, `url` (inside the app's scope) and
optional `short_name`, `description` and `icons`. They appear when the user long-presses the
icon. "The presentation and the number of shortcuts shown to users is at the discretion of
browsers and/or the operating system" [2]. For Roji, one or two: "Quick add" (opens straight
into the capture field) and maybe "Today". Users can also drag a shortcut onto the home screen
from the long-press menu (Android launcher behaviour, not checked against a primary source here).

### Offline capture queue

The pattern: the service worker precaches the app shell; captures are written to IndexedDB first
and sent to the server later.

- **IndexedDB** works in pages and service workers [8]. Chrome lets one origin use up to 60% of
  disk. By default the data is "best effort" and can be evicted under storage pressure, least
  recently used origin first. Request persistent storage with `navigator.storage.persist()` to
  protect the queue [8].
- **Background Sync** (one-off): the page registers a tag; the service worker gets a `sync`
  event when connectivity returns, "even if the page is closed". If the handler's promise
  rejects, the browser retries with exponential back-off. No permission is needed, but you can
  only register while a window of the site is open. Feature-detect `SyncManager` and fall back to
  sending from the page [7]. Chrome has shipped it since Chrome 49 [7].
- **Periodic Background Sync** exists but only for installed apps with engagement, at a
  frequency the browser chooses [12]. Don't rely on it for anything time-critical.

Sync must be idempotent: give every capture a client-generated ID so a retry after a lost reply
can't create a duplicate task.

### Notifications and badges

- **Push** works on Android Chrome: the server sends an encrypted message, signed with VAPID, to
  the browser vendor's push service (an `fcm.googleapis.com` endpoint for Chrome). The service
  worker wakes and calls `showNotification()`. Permission is requested with
  `Notification.requestPermission()`, which should be triggered by a user gesture [4].
- **Implication for a homelab:** the homelab server must make **outbound** HTTPS calls to the
  push service. The phone never connects to the homelab to receive a push, so reminders work
  even when the phone can't reach the homelab (for example off the VPN).
- **No scheduled local notifications.** "The development of Notification Triggers API ... has
  ended" and it never shipped [11]. Every reminder must be timed and sent by the server.
- **Badges:** the Badging API is not supported on Android. Android "only ever shows a dot", and
  it does so automatically when the installed app has an unread notification [3]. A count
  ("3 due today") is not possible. For a calm app that may be a good thing.

### Voice input

- The Web Speech API (`SpeechRecognition`, prefixed `webkitSpeechRecognition` in Chrome) works
  in Chrome. "Depending on the browser, your audio might be sent to a server-based recognition
  service, in which case recognition won't work offline" [9].
- On-device recognition (`processLocally = true`) shipped in Chrome 139 on desktop, but Chromium
  engineers state "Android & ChromeOS currently do not support on-device Web Speech" [10]. On
  Android, in-page voice capture needs a network, and the audio goes to Google.
- **Simpler alternative:** a plain text field already gets the microphone button of the Android
  keyboard (Gboard voice typing), which may work offline depending on the keyboard's language
  packs. For the MVP, rely on keyboard dictation and skip the Web Speech API.

## Requirements for a self-hosted homelab

- **Secure context is mandatory.** Service workers can only be registered from secure contexts
  [13]. A secure context is HTTPS or a loopback address (`localhost`, `127.0.0.0/8`, `::1`).
  `http://192.168.x.x` is **not** secure [13]. Installation also requires HTTPS [6]. So without
  valid HTTPS: no install, no share target, no offline queue, no push.
- **The certificate must be trusted by Android Chrome without warnings.** In practice that means
  a real domain with a public CA certificate (for example Let's Encrypt with a DNS-01 challenge,
  which works without exposing the server to the internet) or a tunnel or VPN product that issues
  real certificates. A self-signed certificate or a private CA installed on the phone is fragile:
  Android apps don't trust user-added CAs by default, and it is worth testing whether WebAPK
  minting and push work in that set-up before choosing it. (Not settled by a primary source
  here; flag for the deployment ticket.)
- **WebAPK minting:** Chrome uses Google's minting server; how much of the manifest and icons it
  needs to fetch from a site that is only reachable on the home network or VPN was not confirmed
  from a primary source. Test install on the real phone early.
- **Push** needs outbound internet from the homelab to the push service, plus storage of each
  `PushSubscription` and VAPID keys on the server [4].
- **Offline is the normal case**, not an edge case: whenever the phone is off the home network or
  VPN, the homelab is unreachable. The offline capture queue is therefore core to the MVP.

## Web first, native later

- **Trusted Web Activity** wraps the same PWA in an Android app shell for the Play Store (Chrome
  72+, built with Bubblewrap). The site and app are linked with Digital Asset Links. Content is
  still rendered by Chrome and "the host app doesn't have direct access to web content" [14].
  This gives a store listing but none of the missing native capabilities. It is a step, not the
  native app.
- What only a native app gives Roji: home-screen widgets with a capture box, Quick Settings tile,
  lock-screen and assistant capture, scheduled local reminders, offline speech recognition,
  numeric badges, and capture of shares that work without any server round trip.
- **Design consequences now** so a native client can come later without rework:
  - Keep capture as a plain HTTP API (create a capture with a client-generated ID, idempotent),
    used by the PWA's sync queue today and by native apps, MCP and email intake later.
  - Keep reminders and notification timing on the server (the PWA has no choice anyway); a
    native app can later add local scheduling on top.
  - Treat the client as offline-first with a sync queue, which a native app reuses unchanged.

## Sources

1. Chrome for Developers, "Receive shared data with the Web Share Target API".
   https://developer.chrome.com/docs/capabilities/web-apis/web-share-target
2. MDN, "shortcuts" (web app manifest).
   https://developer.mozilla.org/en-US/docs/Web/Progressive_web_apps/Manifest/Reference/shortcuts
3. Chrome for Developers, "Badging for app icons".
   https://developer.chrome.com/docs/capabilities/web-apis/badging-api
4. web.dev, "Push notifications overview".
   https://web.dev/articles/push-notifications-overview
5. web.dev, "Learn PWA: Installation" (WebAPK).
   https://web.dev/learn/pwa/installation
6. web.dev, "What does it take to be installable?"
   https://web.dev/articles/install-criteria
7. Chrome for Developers, "Introducing Background Sync".
   https://developer.chrome.com/blog/background-sync
8. web.dev, "Storage for the web".
   https://web.dev/articles/storage-for-the-web
9. MDN, "Using the Web Speech API".
   https://developer.mozilla.org/en-US/docs/Web/API/Web_Speech_API/Using_the_Web_Speech_API
10. blink-dev, "Intent to Ship: Web Speech API: On-Device Recognition Quality" (Android statement)
    and "Intent to Ship: On-device Web Speech API".
    https://groups.google.com/a/chromium.org/g/blink-dev/c/P8P-x7AnC6I ,
    https://groups.google.com/a/chromium.org/g/blink-dev/c/VNOok2dbmHM
11. Chrome for Developers, "Notification Triggers" (development ended).
    https://developer.chrome.com/docs/web-platform/notification-triggers
12. Chrome for Developers, "Richer offline experiences with the Periodic Background Sync API".
    https://developer.chrome.com/docs/capabilities/periodic-background-sync
13. MDN, "Secure contexts".
    https://developer.mozilla.org/en-US/docs/Web/Security/Secure_Contexts
14. Chrome for Developers, "Trusted Web Activity".
    https://developer.chrome.com/docs/android/trusted-web-activity
