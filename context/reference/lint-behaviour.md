# Lint behaviour notes

Implementation notes on how the `types/*` lint rules follow Rector. They are too detailed for the public command page (`docs/content/docs/commands/lint.mdx`).

Rules ported from Rector read types the way Rector 2.8.4 does with PHPStan 2.3.5, the version it requires. In a trait, only `$this` and the trait's own members have no type.
`types/add-return-type-from-constant` reads documented constant types. Anonymous classes are final for late-static constant lookup.
Property-type rules preserve relative `parent` hints from constructors and accessors instead of replacing them with the parent's class name.
Their PHPDoc cleanup also applies Rector's scalar-type comparison to scalar-only unions.
Constructor initialization detection recognizes same-class named static calls and property assignments, including imported and qualified class names.
It also follows documented peer receivers and inherited helper declarations, including external and generic receivers.
Only direct, first-level helper assignments count. Helper calls do not recurse through other helpers.
Class-name case rules and ambiguous intersection receivers follow Rector's distinct call and assignment checks.
Object-type comparison preserves intersection constraints and removes a constraint only when class reflection proves it redundant.

Fix rounds refresh closure and arrow-function signatures after source edits.
Moving a closure preserves its parameters, return type, and captured variables.
