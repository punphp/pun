import type { AstroIntegration } from "astro";

// `astro dev` serves every client module unbundled, and the browser finds each level of imports only after the
// level above it arrives: about six round trips for the homepage. Over a slow link (the orb's portal) that is most
// of the load time. In dev only, this adds a <link rel="modulepreload"> for every module a page's scripts import
// statically, so the browser fetches the whole graph in the first round. `blume build` does not run it.

// Static imports and re-exports in Vite's transformed output, whose specifiers are already the URLs the browser
// requests (`/@fs/...?v=...`). Dynamic `import("...")` stays lazy, so it is left out.
const STATIC_IMPORT = /(?:^|[\n;])\s*(?:import|export)\s*(?:[\w$*{}\s,]+?\s*from\s*)?["']([^"']+)["']/g;
const MODULE_SCRIPT = /<script\b[^>]*\btype="module"[^>]*\bsrc="([^"]+)"/g;

type ClientEnvironment = { transformRequest(url: string): Promise<{ code: string } | null> };

async function moduleGraph(env: ClientEnvironment, entries: string[]): Promise<string[]> {
  const seen = new Set<string>(entries);
  const queue = [...entries];
  while (queue.length > 0) {
    const batch = queue.splice(0);
    await Promise.all(
      batch.map(async (url) => {
        // Vite's own transform middleware unwraps virtual ids (`/@id/__x00__astro:...`) the same way.
        const id = url.startsWith("/@id/") ? url.slice(5).replace("__x00__", "\0") : url;
        const result = await env.transformRequest(id).catch(() => null);
        for (const [, spec] of result?.code.matchAll(STATIC_IMPORT) ?? []) {
          if (spec.startsWith("/") && !seen.has(spec)) {
            seen.add(spec);
            queue.push(spec);
          }
        }
      })
    );
  }
  for (const entry of entries) seen.delete(entry);
  return [...seen];
}

export function devModulePreload(): AstroIntegration {
  return {
    name: "pun-dev-modulepreload",
    hooks: {
      "astro:server:setup": ({ server }) => {
        const env = server.environments.client as unknown as ClientEnvironment;
        server.middlewares.use((req, res, next) => {
          if (req.method !== "GET" || !req.headers.accept?.includes("text/html")) return next();
          const chunks: Buffer[] = [];
          const writeHead = res.writeHead.bind(res);
          const end = res.end.bind(res);
          // Headers passed to writeHead move onto the response, so the content-length is set from the final body.
          let status: number | undefined;
          res.writeHead = ((code: number, ...rest: unknown[]) => {
            status = code;
            const headers = rest.find((r) => typeof r === "object" && r !== null) as Record<string, string> | undefined;
            for (const [k, v] of Object.entries(headers ?? {})) res.setHeader(k, v);
            return res;
          }) as typeof res.writeHead;
          res.write = ((chunk: unknown) => {
            if (chunk != null) chunks.push(Buffer.from(chunk as string));
            return true;
          }) as typeof res.write;
          res.end = ((chunk?: unknown) => {
            if (chunk != null && typeof chunk !== "function") chunks.push(Buffer.from(chunk as string));
            let body = Buffer.concat(chunks);
            const finish = () => {
              res.setHeader("content-length", body.length);
              writeHead(status ?? res.statusCode);
              end(body);
            };
            const type = String(res.getHeader("content-type") ?? "");
            if (!type.includes("text/html") || res.getHeader("content-encoding")) return finish(), res;
            const html = body.toString();
            const entries = [...html.matchAll(MODULE_SCRIPT)].map((m) => m[1].replaceAll("&amp;", "&"));
            moduleGraph(env, entries)
              .then((urls) => {
                const links = urls.map((u) => `<link rel="modulepreload" href="${u.replaceAll("&", "&amp;")}">`).join("");
                body = Buffer.from(html.replace("</head>", `${links}</head>`));
              })
              .catch(() => {})
              .finally(finish);
            return res;
          }) as typeof res.end;
          next();
        });
      },
    },
  };
}
