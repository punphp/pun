<?php
namespace DeclarationPurity;

function emptyBody(): void {}
function onlyComment(): void { /* empty */ }
function onlyNop(): void { ; }
function emptyReturn(): void { return; }
function unreachableEffect(): void { return; print 'unreachable'; }
function noVoid(): int { return 1; }
/** @return void */
function docVoid() { $x = 1; }
/** @pure */
function pureDeclaration(): void { $x = 1; }
/** @impure */
function impureDeclaration(): void { $x = 1; }
/** @phpstan-impure */
function phpstanImpureDeclaration(): void { $x = 1; }
/** @phpstan-pure */
function phpstanPureDeclaration(): void { $x = 1; }
/** @psalm-pure */
function psalmPureDeclaration(): void { $x = 1; }
/** @phan-pure */
function phanPureDeclaration(): void { $x = 1; }
/** @phan-side-effect-free */
function phanSideEffectFreeDeclaration(): void { $x = 1; }
/** @phpstan-assert int $x */
function assertion(mixed $x): void { $x = 1; }
/** @psalm-assert-if-true int $x */
function conditionalAssertion(mixed $x): void { $x = 1; }
/** @phpstan-assert int $x->value */
function propertyAssertion(object $x): void { $local = 1; }
/** @phpstan-assert int $x->value() */
function methodAssertion(object $x): void { $local = 1; }
function inferredAssertion(mixed $x): void { if (!is_int($x)) { throw new \Exception(); } }
function reference(int &$x): void { $local = 1; }
function variadicReference(int &...$x): void { $local = 1; }
function localReference(): void { $x = 1; $y =& $x; }
function propertyReference(object $o): void { $x =& $o->value; }
function offsetReference(): void { $x = [1]; $y =& $x[0]; }
function globals(): void { $x = $_GET; }
function globalStatement(): void { global $x; $x = 1; }
function staticStatement(): void { static $x = 1; }
function echoStatement(): void { echo 'effect'; }
function printExpression(): void { print 'effect'; }
function localRead(int $x): void { $x; }
function localWrites(): void { $x = 1; $x += 2; ++$x; $x--; }
function nestedList(): void { list(list($x)) = [[1]]; }
function keyedList(): void { ['key' => $x, , $y] = ['key' => 1, 1 => 2]; }
function literalKeys(): void { $x = [print 'key' => 1]; }
function append(): void { $x = []; $x[] = 1; }
function propertyOffset(object $o): void { $o->value[0] = 1; }
function nestedFunction(): void { function inner(): int { print 'nested'; return 1; } }
function nestedClosure(): void { $x = function (): void { print 'nested'; }; }
function nestedArrow(): void { $x = fn () => print 'nested'; }
function closureReference(): void { $x = 1; $f = function () use (&$x): void { $x = 2; }; }
function onlyTestingCall(): void { \PHPStan\Testing\assertType('int', 1); }
function testingAndWrite(): void { \PHPStan\Testing\assertType('int', 1); $x = 1; }
function testingArgumentEffect(): void { \PHPStan\Testing\assertType('1', print 'effect'); }

/**
 * @pure
 * @throws void
 */
function noThrow(): int { return 1; }
/** @pure */
function implicitThrow(): int { return 1; }
/**
 * @pure
 * @throws \Exception
 */
function documentedThrow(): int { return 1; }
/**
 * @pure
 * @impure
 * @throws void
 */
function purePrecedence(): int { return 1; }
/**
 * @impure
 * @throws void
 */
function explicitImpure(): int { return 1; }
/**
 * @pure
 * @throws void
 */
function pureVoid(): void {}
/**
 * @phpstan-pure
 * @phpstan-throws void
 */
function prefixedNoThrow(): int { return 1; }
function callsNoThrow(): void { noThrow(); }
function callsImplicitThrow(): void { implicitThrow(); }
function callsDocumentedThrow(): void { documentedThrow(); }
function callsPurePrecedence(): void { purePrecedence(); }
function callsExplicitImpure(): void { explicitImpure(); }
function callsPureVoid(): void { pureVoid(); }
function callsPrefixedNoThrow(): void { prefixedNoThrow(); }
function callsUnknown(): void { unknownFunction(); }
function callsVariable(callable $f): void { $f(); }
function callsMethod(object $o): void { $o->value(); }
function callArgumentEffect(): void { noThrow(print 'effect'); }
function caughtImplicitThrow(): void { try { implicitThrow(); } catch (\Throwable $e) {} }
function caughtExplicitThrow(\Exception $e): void { try { throw $e; } catch (\Throwable $caught) {} }
function uncaughtExplicitThrow(\Exception $e): void { throw $e; }
function narrowCatch(\Exception $e): void { try { throw $e; } catch (\LogicException $caught) {} }
function caughtEffect(): void { try { implicitThrow(); } catch (\Throwable $e) { print 'effect'; } }
function finallyEffect(): void { try { $x = 1; } finally { print 'effect'; } }
function finallyLocal(): void { try { $x = 1; } finally { $x = 2; } }
function unreachableCatch(): void { try { $x = 1; } catch (\Throwable $e) { print 'unreachable'; } }
function finallyAfterReturn(): void { try { return; } finally { print 'effect'; } }
function caughtReturn(): void { try { implicitThrow(); return; } catch (\Throwable $e) {} $x = 1; }
function caughtThenEffect(): void { try { implicitThrow(); return; } catch (\Throwable $e) {} print 'effect'; }
function block(): void { { $x = 1; } }

class Methods {
    private function local(): void { $x = 1; }
    private static function staticLocal(): void { $x = 1; }
    protected function protectedLocal(): void { $x = 1; }
    public function publicLocal(): void { $x = 1; }
    private function __construct() { $x = 1; }
    private function empty(): void {}
    /** @pure */
    private function pureLocal(): void { $x = 1; }
    /** @impure */
    private function impureLocal(): void { $x = 1; }
    /** @phpstan-assert int $x */
    private function assertion(mixed $x): void { $x = 1; }
    private function reference(int &$x): void { $local = 1; }
    private function writeProperty(): void { $this->value = 1; }
}
/** @phpstan-consistent-constructor */
class UnrelatedClassTag {
    private function local(): void { $x = 1; }
}
/** @phpstan-all-methods-pure */
class AllPure {
    private function local(): void { $x = 1; }
}
/** @phpstan-all-methods-impure */
class AllImpure {
    private function local(): void { $x = 1; }
    public function nested(): void { function nestedImpure(): void { $x = 1; } }
}
function emptyBlock(): void { {} }
function blockTesting(): void { { \PHPStan\Testing\assertType('int', 1); } }
function parenthesizedTesting(): void { (\PHPStan\Testing\assertType('int', 1)); }
class OffsetEffects implements \ArrayAccess {
    public function offsetExists(mixed $offset): bool { print 'effect'; return true; }
    public function offsetGet(mixed $offset): mixed { print 'effect'; return 1; }
    public function offsetSet(mixed $offset, mixed $value): void { print 'effect'; }
    public function offsetUnset(mixed $offset): void { print 'effect'; }
}
function objectOffset(OffsetEffects $o): void { $o[0] = 1; }
function lostArrayFact(OffsetEffects $o): void { $a = []; $r =& $a; $r = $o; $a[0] = 1; }
function nestedObject(OffsetEffects $o): void { $a = [$o]; $a[0][0] = 1; }
function returnValue(): void { return 1; }
function parenthesizedValue(): void { $x = (1); }
function nestedLegacyTarget(): void { [array($x)] = [[1]]; }
function noopsAndValue(): void { ; $x = 1; }
function caughtUnion(\Exception $e): void { try { throw $e; } catch (\Exception|\Error $caught) { print 'effect'; } }
function caughtRethrow(\Exception $e): void { try { throw $e; } catch (\Throwable $caught) { throw $caught; } }
function unknownCaughtEffect(): void { try { implicitThrow(); } catch (\Throwable $e) { unknownFunction(); } }
function uncaughtFinally(): void { try { implicitThrow(); } finally { $x = 1; } }
/** @pure-unless-callable-is-impure $callback */
function conditionalCallable(?callable $callback = null): void { $x = 1; }
/** @phpstan-pure-unless-parameter-passed $value */
function conditionalPassed(?int $value = null): void { $x = 1; }
/** @pure */
function pureNever(): never { throw new \Exception(); }
function callsNever(): void { pureNever(); }
class MoreMethods {
    private function noVoid(): int { return 1; }
    public function nested(): void { function nestedUnspecified(): void { $x = 1; } }
    /** @pure-unless-callable-is-impure $callback */
    private function conditionalCallable(?callable $callback = null): void { $x = 1; }
    /** @pure-unless-parameter-passed $value */
    private function conditionalPassed(?int $value = null): void { $x = 1; }
}
abstract class AbstractMethod {
    abstract private function missingBody(): void;
}
enum Choice { case One; }
function switchLocal(int $x): void { switch ($x) { case -1: break; case 1: $y = 1; default: break; } }
function switchAlternative(int $x): void { switch ($x): case +1: break 1; default: break; endswitch; }
function switchEffect(int $x): void { switch ($x) { case 1: print 'effect'; break; } }
function switchCaseEffect(int $x): void { switch ($x) { case print 'effect': break; } }
function switchThenEffect(int $x): void { switch ($x) { case 1: return; } print 'effect'; }
function switchUnknownCase(int $x): void { switch ($x) { case unknownFunction(): break; } }
function classConstant(): void { $x = Choice::One; }
// @phpstan-ignore void.pure
function ignoredPure(): void { $x = 1; }
function narrowImplicitCatch(): void { try { implicitThrow(); } catch (\Exception $e) {} }
function finallyReturns(): void { try { implicitThrow(); } finally { return; } }
