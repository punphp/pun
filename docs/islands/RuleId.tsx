import { useState } from "react";

export const client = "load";

export default function RuleId({ id }: { id: string }) {
  const [label, setLabel] = useState("Copy rule ID");
  return (
    <button className="rule-copy" type="button" onClick={async () => {
      try {
        await navigator.clipboard.writeText(id);
        setLabel("Copied");
      } catch {
        setLabel("Select the title to copy");
      }
    }}>{label}</button>
  );
}
