import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, expect, test, vi } from "vitest";
import { App } from "./App.tsx";

afterEach(() => {
  cleanup();
  vi.unstubAllGlobals();
});

function serverAnswers(answer: Response | Error) {
  const fakeFetch = vi.fn<typeof fetch>(async () => {
    if (answer instanceof Error) {
      throw answer;
    }
    return answer;
  });
  vi.stubGlobal("fetch", fakeFetch);
  return fakeFetch;
}

function statusText() {
  return screen.getByRole("status").textContent;
}

test("says hello and that server and database are connected", async () => {
  const fakeFetch = serverAnswers(
    Response.json({ status: "ok", database: "ok" }),
  );

  render(<App />);

  expect(screen.getByRole("heading").textContent).toBe("Hello from Roji");
  await vi.waitFor(() =>
    expect(statusText()).toBe("Server and database are connected."),
  );
  expect(fakeFetch).toHaveBeenCalledWith("/api/health", expect.anything());
});

test("reports a database that is down", async () => {
  serverAnswers(
    Response.json(
      { status: "unavailable", database: "unreachable" },
      { status: 503 },
    ),
  );

  render(<App />);

  await vi.waitFor(() =>
    expect(statusText()).toBe(
      "The server is up, but the database is unreachable.",
    ),
  );
});

test("reports a server that cannot be reached", async () => {
  serverAnswers(new TypeError("fetch failed"));

  render(<App />);

  await vi.waitFor(() =>
    expect(statusText()).toBe("The server cannot be reached."),
  );
});
