<?php
namespace DeclarationConstructorEffects;

interface Contract {}
trait Reusable {}
enum Choice {}
abstract class AbstractEmpty {}
class EmptyClass {}
class PureConstructor {
    /**
     * @pure
     * @throws void
     */
    public function __construct(int $value = 0) {}
}
class InheritedConstructor extends PureConstructor {}
class ImplicitThrow {
    /** @pure */
    public function __construct() {}
}
class ExplicitThrow {
    /**
     * @pure
     * @throws \Exception
     */
    public function __construct() {}
}
class ImpureConstructor {
    /**
     * @impure
     * @throws void
     */
    public function __construct() {}
}
class UnspecifiedConstructor {
    /** @throws void */
    public function __construct() {}
}
class Stops {
    /**
     * @pure
     * @throws void
     * @return never
     */
    public function __construct() { exit; }
}
class ThrowingStops {
    /**
     * @pure
     * @throws \Exception
     * @return never
     */
    public function __construct() { throw new \Exception(); }
}
class PrefixedThrows {
    /**
     * @pure
     * @throws \Exception
     * @phpstan-throws void
     */
    public function __construct() {}
}
/** @phpstan-all-methods-pure */
class ClassPure {
    /** @throws void */
    public function __construct() {}
}
class ConditionalConstructor {
    /**
     * @pure-unless-parameter-passed $value
     * @throws void
     */
    public function __construct(int $value = 0) {}
}
class ReferenceConstructor {
    /**
     * @pure
     * @throws void
     */
    public function __construct(int &$value) { $value = 2; }
}
/**
 * @pure
 * @throws void
 */
function pureArgument(): int { return 1; }
/**
 * @impure
 * @throws void
 */
function impureArgument(): int { return 1; }
/** @pure */
function throwingArgument(): int { return 1; }

function namedEmpty(): void { $x = new EmptyClass; }
function builtinEmpty(): void { $x = new \stdClass(); }
function illegalInterface(): void { $x = new Contract(); }
function illegalTrait(): void { $x = new Reusable(); }
function illegalEnum(): void { $x = new Choice(); }
function illegalAbstract(): void { $x = new AbstractEmpty(); }
function unknownClass(): void { $x = new Missing(); }
function pureConstructor(): void { $x = new PureConstructor(); }
function inheritedConstructor(): void { $x = new InheritedConstructor(); }
function implicitThrow(): void { $x = new ImplicitThrow(); }
function caughtImplicitThrow(): void { try { $x = new ImplicitThrow(); } catch (\Throwable $e) {} }
function explicitThrow(): void { $x = new ExplicitThrow(); }
function caughtExplicitThrow(): void { try { $x = new ExplicitThrow(); } catch (\Throwable $e) {} }
function impureConstructor(): void { $x = new ImpureConstructor(); }
function unspecifiedConstructor(): void { $x = new UnspecifiedConstructor(); }
function prefixedThrows(): void { $x = new PrefixedThrows(); }
function classPure(): void { $x = new ClassPure(); }
function localArgument(): void { $x = new PureConstructor($written = 1); }
function namedArgument(): void { $x = new PureConstructor(value: 1); }
function pureCallArgument(): void { $x = new PureConstructor(pureArgument()); }
function impureCallArgument(): void { $x = new PureConstructor(impureArgument()); }
function throwingCallArgument(): void { $x = new PureConstructor(throwingArgument()); }
function caughtArgumentThrow(): void { try { $x = new PureConstructor(throwingArgument()); } catch (\Throwable $e) {} }
function printArgument(): void { $x = new PureConstructor(print 'effect'); }
function noConstructorArgument(): void { $x = new EmptyClass(print 'effect'); }
function directNever(): void { $x = new Stops(); echo 'unreachable'; }
function explicitThrowNever(): void { $x = new ThrowingStops(); }
function nestedNever(): void { $x = new PureConstructor(new Stops()); echo 'reachable'; }
function caughtNever(): void { try { $x = new ThrowingStops(); } catch (\Throwable $e) {} echo 'reachable'; }
function throwArgument(\Exception $e): void { $x = new PureConstructor(throw $e); echo 'unreachable'; }
function caughtUnknown(): void { try { $x = new Missing(); } catch (\Throwable $e) {} }
function caughtImpure(): void { try { $x = new ImpureConstructor(); } catch (\Throwable $e) {} }
function impureAfterThrowArgument(\Exception $e): void { try { $x = new ImpureConstructor(throw $e); } catch (\Throwable $caught) {} }
function arrayAfterEmpty(): void { $array = []; $x = new EmptyClass(); $array[] = 1; }
function arrayAfterPure(): void { $array = []; $x = new PureConstructor(); $array[] = 1; }

// Unsupported argument policies must remain unknown, never fabricated as empty effects.
function unpacked(): void { $x = new PureConstructor(...[1]); }
function reference(): void { $value = 1; $x = new ReferenceConstructor($value); }
function conditional(): void { $x = new ConditionalConstructor(); }
function closureArgument(): void { $x = new PureConstructor(fn () => 1); }
function builtinConstructor(): void { $x = new \Exception(); }
function dynamicClass(): void { $class = EmptyClass::class; $x = new $class(); }
class ConditionalThrows {
    /**
     * @pure
     * @throws ($value is 0 ? void : \Exception)
     */
    public function __construct(int $value = 0) {}
}
function conditionalThrows(): void { $x = new ConditionalThrows(); }
