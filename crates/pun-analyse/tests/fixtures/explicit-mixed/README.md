# Explicit mixed producer controls

`expected-phpstan.json` records PHPStan 2.3.1's public `Scope::getType()` and `getNativeType()` results.
`expected-phpstan-227.json` records the same queries with Rector's PHPStan 2.2.7. The captures agree byte for byte.
Both use PHP 8.4 analysis settings. The host runs PHP 8.5.

From the repository root, regenerate each capture with:

```sh
php crates/pun-analyse/tests/fixtures/explicit-mixed/oracle.php \
	.oracle/phpstan/vendor/phpstan/phpstan/bootstrap.php \
	crates/pun-analyse/tests/fixtures/explicit-mixed/{declarations,xm,xm2}.php \
	> crates/pun-analyse/tests/fixtures/explicit-mixed/expected-phpstan.json
php crates/pun-analyse/tests/fixtures/explicit-mixed/oracle.php \
	.oracle/rector-ecosystem/vendor/phpstan/phpstan/bootstrap.php \
	crates/pun-analyse/tests/fixtures/explicit-mixed/{declarations,xm,xm2}.php \
	> crates/pun-analyse/tests/fixtures/explicit-mixed/expected-phpstan-227.json
```

The capture configures `pathRoutingParser` with the analysed files before parsing.
Without that step PHPStan uses its reflection parser, which removes assignments and statements.
Those stripped bodies cannot supply a flow-inference baseline.

The Rust tests use the public Analyzer with the real prelude and default plugins.
They compare mixed identity, offset constraints, and overwritten integer literals.
Available results from the partial native-type API are checked independently against the native capture.
That API does not support method bodies, closures, offsets, or built-in calls. The tests do not broaden it.

`xm.php` and `xm2.php` are the supplied Rector repros, unchanged. These existing boundaries remain separate:

- `xm.php:54`: PHPStan infers `null` for invalid `array_pop(string)`. This return-provider gap is not changed.
- `xm2.php:27`: PHPStan intersects `clone` operands with `object`. Mago currently copies the mixed operand.
- `xm2.php:42`: PHPStan keeps the complete falsey union after the conditional overwrite. Mago does not reconstruct that union.
- `xm.php:39`: PHPStan reports explicit mixed for the declared property in the full scope, but implicit mixed in the native scope.
  Mago's full Analyzer correctly reports explicit mixed. Lint's separate docblock-blanked full analysis is not PHPStan's native scope.
  Changing its filter alone must not claim this property contract is fixed.

The first three entries are not compared as explicit-mixed producer contracts.
The complete oracle records remain in both capture files. The property is compared against the full-scope contract.
Unknown property and method reads are compared and must remain implicit mixed.

Rector 2.6.1's `AddArrowFunctionReturnTypeRector` reads full types for array offsets and native types for other expressions.
Its `MixedTypeMapper` emits `mixed` only for explicit mixed. Built-in calls have an additional full-type fallback
in `NodeTypeResolver::resolveNativeTypeWithBuiltinMethodCallFallback()`.
