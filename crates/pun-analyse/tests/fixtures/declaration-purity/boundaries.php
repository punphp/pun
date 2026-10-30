<?php
namespace DeclarationPurityBoundaries;

// These remain unknown, rather than being manufactured as empty producer results.
function builtinNoThrow(): void { strlen('text'); }
function arithmetic(int $x): void { $y = $x + 1; }
function branch(bool $condition): void { if ($condition) { $x = 1; } }
function loop(): void { for ($x = 0; $x < 1; $x++) {} }
function unsetLocal(): void { $x = 1; unset($x); }
function newObject(): void { $x = new \stdClass(); }
function caughtTypedThrow(\Exception $e): void { try { throw $e; } catch (\Exception $caught) {} }
function caughtDocumentedThrow(): void { try { \DeclarationPurity\documentedThrow(); } catch (\Throwable $caught) {} }
function namespaceFallback(): void { \DeclarationPurity\noThrow(...[]); }
function offsetKey(): void { $x = []; $x[print 'key'] = 1; }
function nestedOffset(): void { $x = []; $x[0][1] = 1; }
function indirectVariable(): void { $x = 'local'; $$x = 1; }
trait Imported {
    private function local(): void { $x = 1; }
}
class UsesTrait { use Imported; }
class ImplicitReturn {
    private function implicit() { $x = 1; }
}
function negative(int $x): void { $y = -$x; }
function compoundDivision(): void { $x = 1; $x /= 1; }
function arrayUnpack(): void { $x = [...[1]]; }
function closureArgument(): void { \DeclarationPurity\noThrow(fn () => 1); }
function referenceCall(): void { $x = 1; \DeclarationPurity\reference($x); }
class ParentClass {}
class Inherited extends ParentClass {
    private function local(): void { $x = 1; }
}
/**
 * @pure
 * @throws void
 */
function invokes(callable $callback): int { $callback(); return 1; }
function variableCallback(): void { $callback = fn () => print 'effect'; invokes($callback); }
