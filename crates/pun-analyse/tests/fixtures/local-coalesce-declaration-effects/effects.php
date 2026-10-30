<?php

namespace LocalCoalesceDeclarationEffects;

/** @pure
 * @throws void
 */
function pureValue(): int { return 1; }
/** @impure
 * @throws void
 */
function impureValue(): int { print 'rhs'; return 1; }
/** @pure */
function implicitThrowValue(): int { return 1; }
/** @impure
 * @throws void
 */
function escape(mixed &$value): int { print 'escape'; $value = new Offsets(); return 1; }

function nullableVariable(?int $value): void { $value ??= 1; }
function absentVariable(): void { $value ??= 1; }
function nullVariable(): void { $value = null; $value ??= 1; }
function pureRhs(?int $value): void { $value ??= pureValue(); }
function impureRhs(?int $value): void { $value ??= impureValue(); }
function implicitThrowRhs(?int $value): void { $value ??= implicitThrowValue(); }
function skippedImpureRhs(int $value): void { $value ??= impureValue(); }
function skippedPureRhs(int $value): void { $value ??= pureValue(); }
function printRhs(?int $value): void { $value ??= print 'rhs'; }

/** @param array{required: int} $shape */
function requiredKey(array $shape): void { $shape['required'] ??= 0; }
/** @param array{required: ?int} $shape */
function nullableKey(array $shape): void { $shape['required'] ??= 0; }
/** @param array{optional?: int} $shape */
function optionalKey(array $shape): void { $shape['optional'] ??= 0; }
/** @param array<string, int> $shape */
function dynamicKey(array $shape, string $key): void { $shape[$key] ??= 0; }
/** @param array<string, int> $shape */
function assignedKey(array $shape, string $key): void { $shape[$key = 'key'] ??= 0; }
/** @param array<int, int> $shape */
function effectfulKey(array $shape): void { $shape[print 'key'] ??= 0; }
/** @param array<string, int> $shape */
function offsetPureRhs(array $shape): void { $shape['key'] ??= pureValue(); }
/** @param array<string, int> $shape */
function offsetImpureRhs(array $shape): void { $shape['key'] ??= impureValue(); }
function literalOffset(): void { $shape = []; $shape['key'] ??= 0; $shape[] = 1; }
function arrayContinuation(): void { $shape = []; $shape ??= []; $shape[] = 1; }
function nullArrayContinuation(): void { $shape = null; $shape ??= []; $shape[] = 1; }

function ordinaryAssignment(?int $value): void { $value = pureValue(); }
function ordinaryImpureAssignment(?int $value): void { $value = impureValue(); }
function compoundAddition(int $value): void { $value += 1; }
function compoundDivision(int $value, int $divisor): void { $value /= $divisor; }
function referenceParameter(int &$value): void { $value ??= 1; }
function localReference(?int $value): void { $alias =& $value; $alias ??= 1; }
function superglobal(): void { $_GET ??= []; }
function superglobalOffset(): void { $_GET['key'] ??= 1; }

class Holder {
    public ?int $value = null;
    /** @var array<string, int> */
    public array $values = [];
    public function run(?int $value): void { $this->local($value); }
    private function local(?int $value): void { $value ??= 1; }
}
function property(Holder $holder): void { $holder->value ??= 1; }
function propertyOffset(Holder $holder): void { $holder->values['key'] ??= 1; }

/** @implements \ArrayAccess<string, int> */
class Offsets implements \ArrayAccess {
    public function offsetExists(mixed $offset): bool { print 'exists'; return false; }
    public function offsetGet(mixed $offset): int { print 'get'; return 1; }
    public function offsetSet(mixed $offset, mixed $value): void { print 'set'; }
    public function offsetUnset(mixed $offset): void { print 'unset'; }
}
function objectOffset(Offsets $shape): void { $shape['key'] ??= 1; }
/** @param array<string, int>|Offsets $shape */
function unionOffset(array|Offsets $shape): void { $shape['key'] ??= 1; }
function escapedOffset(): void { $shape = []; escape($shape); $shape['key'] ??= 1; }
function rhsReplacesContainer(Offsets $object): void {
    $shape = [];
    $shape['key'] ??= ($shape = $object);
    $shape['next'] = 1;
}
function rhsEscapesContainer(): void {
    $shape = [];
    $shape['key'] ??= escape($shape);
    $shape['next'] = 1;
}
function rhsChangesOtherLocal(?int $value, Offsets $object): void {
    $shape = [];
    $value ??= ($shape = $object);
    $shape['key'] = 1;
}
function rhsKeepsOtherArray(?int $value): void {
    $shape = [];
    $value ??= ($shape = []);
    $shape[] = 1;
}

function caughtNullable(?int $value, \Exception $exception): void {
    try { $value ??= throw $exception; print 'continuing'; } catch (\Throwable $caught) {}
}
function caughtNull(\Exception $exception): void {
    $value = null;
    try { $value ??= throw $exception; print 'unreachable'; } catch (\Throwable $caught) {}
}
function caughtNonNull(int $value, \Exception $exception): void {
    try { $value ??= throw $exception; print 'continuing'; } catch (\Throwable $caught) {}
}
function caughtTargetTermination(\Exception $exception): void {
    $shape = [];
    try { $shape[throw $exception] ??= 1; } catch (\Throwable $caught) {}
}
function unconditionalReturn(?int $value): void { return; $value ??= impureValue(); }

function rhsReplacesContainerOnly(Offsets $object): void {
    $shape = [];
    $shape['key'] ??= ($shape = $object);
}
function indexReplacesContainer(Offsets $object): void {
    $shape = [];
    $shape[($shape = $object) === $object] ??= 1;
}
function aliasedObjectOffset(Offsets $object): void {
    $shape = $object;
    $alias =& $shape;
    $alias = $object;
    $shape['key'] ??= 1;
}
function offsetRhsAssignsOtherLocal(): void {
    $shape = [];
    $shape['key'] ??= ($other = 1);
}
