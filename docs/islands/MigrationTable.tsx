import { useState } from "react";
import mapping from "../generated/migrations.json";

export const client = "load";

export default function MigrationTable({ tool }: { tool: keyof typeof mapping.tools }) {
  const [query, setQuery] = useState("");
  const [unsupported, setUnsupported] = useState(false);
  const rows = mapping.tools[tool].filter(row =>
    (!unsupported || row.unsupported) &&
    `${row.name} ${row.rules.join(" ")} ${row.note}`.toLowerCase().includes(query.toLowerCase())
  );
  return (
    <div className="rule-reference">
      <label>Search upstream names and Pun IDs
        <input type="search" value={query} onChange={event => setQuery(event.target.value)} />
      </label>
      <label className="rule-check">
        <input type="checkbox" checked={unsupported} onChange={event => setUnsupported(event.target.checked)} />
        Unsupported only (includes dropped rules)
      </label>
      <p role="status">{rows.length} of {mapping.tools[tool].length} upstream entries</p>
      <div className="rule-table-scroll" tabIndex={0} role="region" aria-label="Rule mapping">
        <table>
          <thead><tr><th>Upstream name</th><th>Pun rule</th><th>Notes</th></tr></thead>
          <tbody>{rows.map(row => <tr key={`${row.tool}:${row.name}`}>
            <td><a href={row.url}><code>{row.name}</code></a><small>{row.tool}</small></td>
            <td>{row.rules.length ? row.rules.map(rule => <a key={rule} href={`/docs/rules/${rule}`}><code>{rule}</code></a>) : "—"}</td>
            <td>{row.note}</td>
          </tr>)}</tbody>
        </table>
        {!rows.length && <p>No matching upstream rules.</p>}
      </div>
    </div>
  );
}
