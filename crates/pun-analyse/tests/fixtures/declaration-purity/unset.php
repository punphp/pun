<?php
namespace SentinelDeliveryEdges;

use function PHPStan\dumpType;

/** @param array{only: int, ...<string, int>} $shape */
function openStrings(array $shape): void {
    unset($shape['only']);
    dumpType($shape);
}
/** @param array{only: int, ...<int, int>} $shape */
function openIntegers(array $shape): void {
    unset($shape['only']);
    dumpType($shape);
}

function localVariable(int $value): void { unset($value); }
function multipleVariables(int $left, int $right): void { unset($left, $right); }
function literalArray(): void { $values = [1, 2]; unset($values[0]); $values[1] = 3; }
function unsetThenReplace(): void { $values = [1]; unset($values); $values = []; $values[] = 1; }
/** @param array<string, int> $values */
function dynamicIndex(array $values, string $key): void { unset($values[$key]); }
/** @param array<int, int> $values */
function localIndexWrite(array $values, int $key): void { unset($values[$key = 0]); }
/** @param array<int, int> $values */
function indexEffect(array $values): void { unset($values[print 'index']); }
/** @param array<int, int> $values */
function multipleEffects(array $values): void { unset($values[0], $values[print 'second']); }
/** @param array<int, int> $values */
function effectAfterUnset(array $values): void { unset($values[0]); print 'after'; }
/** @param array<int, int> $values */
function testingArgumentEffect(array $values): void { unset($values[0]); dumpType(print 'argument'); }
function globalVariable(): void { unset($_GET); }
function globalOffset(): void { unset($_GET['key']); }

class Properties {
    public int $value = 1;
    /** @var array<int, int> */
    public array $values = [1];
}
function property(Properties $object): void { unset($object->value); }
function propertyOffset(Properties $object): void { unset($object->values[0]); }

/** @implements \ArrayAccess<int, int> */
class Offsets implements \ArrayAccess {
    public function offsetExists(mixed $offset): bool { print 'exists'; return true; }
    public function offsetGet(mixed $offset): int { print 'get'; return 1; }
    public function offsetSet(mixed $offset, mixed $value): void { print 'set'; }
    public function offsetUnset(mixed $offset): void { print 'unset'; }
}
function objectOffset(Offsets $object): void { unset($object[0]); }
/** @param array<int, int>|Offsets $values */
function maybeObject(array|Offsets $values): void { unset($values[0]); }
function replacedArray(Offsets $object): void { $values = [1]; $values = $object; unset($values[0]); }
function indexReplacesArray(Offsets $object): void {
    $values = [1];
    unset($values[($values = $object) === $object]);
}
function referenceReplacesArray(Offsets $object): void {
    $values = [1]; $alias =& $values; $alias = $object; unset($values[0]);
}
/** @param array<int, int> $values */
function referenceParameter(array &$values): void { unset($values[0]); }
function nestedObject(Offsets $object): void { $values = [$object]; unset($values[0][0]); }
function referenceVariable(int $value): void { $alias =& $value; unset($alias); }
function unsetFactThenObject(Offsets $object): void {
    $values = [1]; unset($values); $values = $object; unset($values[0]);
}
function branchReference(bool $condition, Offsets $object): void {
    $values = [1];
    if ($condition) { $alias =& $values; $alias = $object; }
    unset($values[0]);
}
class LocalMethod {
    public function __construct(int $value) { $this->local($value); }
    private function local(int $value): void { unset($value); }
}

/** @param array<int, int> $values */
function parameterArrayStaysArray(array $values): void { unset($values[0]); $values[1] = 2; }
// Destruction liveness is a variable-flow concern, not an extra declaration impure point.
function localObject(Properties $object): void { unset($object); }
/** @pure
 * @throws void
 */
function pureIndex(): int { return 0; }
/** @impure
 * @throws void
 */
function effectfulIndex(): int { print 'index call'; return 0; }
/** @pure */
function implicitThrowIndex(): int { return 0; }
/** @param array<int, int> $values */
function pureIndexCall(array $values): void { unset($values[pureIndex()]); }
/** @param array<int, int> $values */
function impureIndexCall(array $values): void { unset($values[effectfulIndex()]); }
/** @param array<int, int> $values */
function throwingIndexCall(array $values): void { unset($values[implicitThrowIndex()]); }

// UnsetHandler forwards throw points without marking the unset statement always-terminating.
function followingOperandEffect(\Exception $exception): void {
    $values = [1];
    try {
        unset($values[throw $exception]);
        print 'after';
    } catch (\Throwable $caught) {}
}
function caughtOperand(\Exception $exception): void {
    $values = [1];
    try { unset($values[throw $exception]); } catch (\Throwable $caught) {}
}
function uncaughtOperand(\Exception $exception): void {
    $values = [1];
    unset($values[throw $exception]);
}
