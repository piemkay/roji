/** The answer of the server's `GET /api/health`. */
export type Health = {
  status: string;
  database: string;
};

/**
 * Asks the server for its health. The server answers with JSON also when its database is down
 * (status 503), so only a missing or non-JSON answer counts as a failure.
 */
export async function fetchHealth(signal?: AbortSignal): Promise<Health> {
  const response = await fetch("/api/health", { signal });
  if (!response.headers.get("content-type")?.includes("application/json")) {
    throw new Error(`the server answered ${response.status} without JSON`);
  }
  return (await response.json()) as Health;
}
