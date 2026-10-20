import { useEffect, useState } from "react";
import { fetchHealth, type Health } from "./health.ts";

type Connection =
  | { kind: "checking" }
  | { kind: "answered"; health: Health }
  | { kind: "unreachable" };

/** The hello page: shows that the web app reaches the server and the server its database. */
export function App() {
  const [connection, setConnection] = useState<Connection>({
    kind: "checking",
  });

  useEffect(() => {
    const controller = new AbortController();
    fetchHealth(controller.signal).then(
      (health) => setConnection({ kind: "answered", health }),
      () => {
        if (!controller.signal.aborted) {
          setConnection({ kind: "unreachable" });
        }
      },
    );
    return () => controller.abort();
  }, []);

  return (
    <main>
      <h1>Hello from Roji</h1>
      <p role="status">{describe(connection)}</p>
    </main>
  );
}

function describe(connection: Connection): string {
  switch (connection.kind) {
    case "checking":
      return "Checking the server…";
    case "unreachable":
      return "The server cannot be reached.";
    case "answered":
      return connection.health.database === "ok"
        ? "Server and database are connected."
        : "The server is up, but the database is unreachable.";
  }
}
