import { useEffect, useState } from "react";
import directory from "../generated/rules.json";

export const client = "load";

const empty = { q: "", command: "", group: "", selection: "", fix: "", ecosystem: "", tool: "", unpreset: "" };
type Filters = typeof empty;
const rules = directory.rules;
const choices = [
  ["command", "Command", ["format", "lint", "analyse"]],
  ["group", "Group", [...new Set(rules.map(rule => rule.group))].sort()],
  ["selection", "Preset or set", [...directory.selections].sort()],
  ["fix", "Fix", ["safe", "unsafe", "mixed", "none"]],
  ["ecosystem", "Ecosystem", [...new Set(rules.map(rule => rule.ecosystem))].sort()],
  ["tool", "Upstream tool", [...new Set(rules.flatMap(rule => rule.tools))].sort()],
] as const;

function readFilters(): Filters {
  const params = new URLSearchParams(window.location.search);
  return Object.fromEntries(Object.keys(empty).map(key => [key, params.get(key) || ""])) as Filters;
}

export default function RulesDirectory() {
  const [filters, setFilters] = useState(empty);
  useEffect(() => {
    const restore = () => setFilters(readFilters());
    restore();
    window.addEventListener("popstate", restore);
    return () => window.removeEventListener("popstate", restore);
  }, []);

  function update(next: Filters, replace = false) {
    const url = new URL(window.location.href);
    Object.entries(next).forEach(([key, value]) => value ? url.searchParams.set(key, value) : url.searchParams.delete(key));
    if (replace) window.history.replaceState(null, "", url);
    else window.history.pushState(null, "", url);
    setFilters(next);
  }

  const matches = rules.filter(rule =>
    (!filters.q || `${rule.id} ${rule.upstream.join(" ")}`.toLowerCase().includes(filters.q.toLowerCase().trim())) &&
    (!filters.command || rule.command === filters.command) &&
    (!filters.group || rule.group === filters.group) &&
    (!filters.selection || rule.enabled.includes(filters.selection)) &&
    (!filters.fix || rule.fix === filters.fix) &&
    (!filters.ecosystem || rule.ecosystem === filters.ecosystem) &&
    (!filters.tool || rule.tools.includes(filters.tool)) &&
    (!filters.unpreset || !rule.enabled.length)
  );

  return (
    <div className="rule-reference" data-rules-directory>
      <label>Search Pun IDs and upstream names
        <input type="search" placeholder="no_alias_functions, StrictArraySearchRector, method.notFound…"
          value={filters.q} onChange={event => update({ ...filters, q: event.target.value }, true)} />
      </label>
      <div className="rule-filters">{choices.map(([key, label, values]) =>
        <label key={key}>{label}
          <select value={filters[key]} onChange={event => update({ ...filters, [key]: event.target.value })}>
            <option value="">All</option>
            {values.map(value => <option key={value} value={value}>{value}</option>)}
          </select>
        </label>
      )}</div>
      <label className="rule-check"><input type="checkbox" checked={!!filters.unpreset}
        onChange={event => update({ ...filters, unpreset: event.target.checked ? "1" : "" })} />
        Not in any preset or set
      </label>
      <div className="rule-chips" aria-label="Active filters">
        {Object.entries(filters).filter(([, value]) => value).map(([key, value]) =>
          <button type="button" key={key} onClick={() => update({ ...filters, [key]: "" })}
            aria-label={`Remove ${key} filter`}>
            {key === "unpreset" ? "Not in any preset" : `${key}: ${value}`} ×
          </button>
        )}
        {Object.values(filters).some(Boolean) && <button type="button" onClick={() => update(empty)}>Clear filters</button>}
      </div>
      <p role="status">{matches.length} of {rules.length.toLocaleString("en-US")} rules</p>
      <p className="rule-legend">Fix: ✓ safe · ! unsafe · ± mixed · — report only. Dim icons do not apply.</p>
      <div className="rule-table-scroll" tabIndex={0} role="region" aria-label="All rules">
        <table>
          <thead><tr><th>ID</th><th>Summary</th><th>Enabled by</th><th>Fix</th></tr></thead>
          <tbody>{matches.map(rule => <tr key={rule.id}>
            <td><a href={`/docs/rules/${rule.id}`}><code>{rule.id}</code></a>
              {!rule.ported && <small className="rule-planned">Planned</small>}</td>
            <td>{rule.summary}</td>
            <td>{rule.enabled.length ? rule.enabled.map(name =>
              <a key={name} href={`/docs/rules/${rule.command === "lint" ? "sets" : "presets"}/${name}`}>{name}</a>
            ) : "Opt-in"}</td>
            <td><span className="rule-icons" role="img" aria-label={`Fix: ${rule.fix}`}>
              {[["safe", "✓"], ["unsafe", "!"], ["mixed", "±"], ["none", "—"]].map(([kind, icon]) =>
                <span aria-hidden="true" key={kind} className={rule.fix === kind ? "" : "rule-dim"}>{icon}</span>
              )}
            </span></td>
          </tr>)}</tbody>
        </table>
        {!matches.length && <p>No matching rules. Remove a filter or try another upstream name.</p>}
      </div>
    </div>
  );
}
