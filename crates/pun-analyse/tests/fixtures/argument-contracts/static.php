<?php declare(strict_types=1);

namespace ArgumentContracts;

/** @param-closure-this \stdClass $callback */
function bind(\Closure $callback): int { return 1; }
function unbound(\Closure $callback): int { return 1; }
/** @param-closure-this \DateTime $callback */
function otherBinding(\Closure $callback): int { return 1; }

class Binder
{
    /** @param-closure-this \stdClass $callback */
    public function method(\Closure $callback): void {}
    /** @param-closure-this \stdClass $callback */
    public static function many(\Closure ...$callback): void {}
}

class ChildBinder extends Binder
{
    public function method(\Closure $renamed): void {}
}

#[\Attribute(\Attribute::TARGET_CLASS | \Attribute::IS_REPEATABLE)]
class Bindable
{
    /** @param-closure-this \stdClass $callback */
    public function __construct(public \Closure $callback) {}
}

#[Bindable(static function (): void {})]
#[Bindable(callback: static function (): void {})]
#[Bindable(function (): void {})]
class Tagged {}

function staticCalls(Binder $binder, ChildBinder $child, bool $choose): void
{
    bind(static function (): void {});
    bind(static fn (): int => 1);
    bind((static fn (): int => 1));
    bind(callback: static fn (): int => 1);
    bind(function (): void {});
    bind(fn (): int => 1);
    unbound(static fn (): int => 1);
    $static = static fn (): int => 1;
    bind($static);
    bind(...[static fn (): int => 1]);
    $binder->method(static fn (): int => 1);
    $child->method(renamed: static fn (): int => 1);
    Binder::many(static fn (): int => 1, static function (): void {});
    Binder::many(extra: static fn (): int => 1);
    $instance = new Bindable(static function (): void {});
    $instance = new Bindable(callback: static function (): void {});
    $callable = __NAMESPACE__.'\\bind';
    $callable(static fn (): int => 1);
    $result = call_user_func(__NAMESPACE__.'\\bind', static fn (): int => 1);
    $result = call_user_func_array(__NAMESPACE__.'\\bind', [static fn (): int => 1]);
    $first = bind(...);
    $first(static fn (): int => 1);
    /** @param-closure-this \stdClass $callback */
    $closure = function (\Closure $callback): int { return 1; };
    $result = $closure(static fn (): int => 1);
    $one = $choose ? __NAMESPACE__.'\\bind' : __NAMESPACE__.'\\unbound';
    $one(static fn (): int => 1);
    $both = $choose ? __NAMESPACE__.'\\bind' : __NAMESPACE__.'\\otherBinding';
    $both(static fn (): int => 1);
    bind(callback: fn (): int => 1, unknown: static fn (): int => 1);
    $firstMethod = $child->method(...);
    $firstMethod(static fn (): int => 1);
    $firstOne = $choose ? bind(...) : unbound(...);
    $firstOne(static fn (): int => 1);
    $firstBoth = $choose ? bind(...) : otherBinding(...);
    $firstBoth(static fn (): int => 1);
    $result = call_user_func($first, static fn (): int => 1);
    $result = call_user_func_array($first, [static fn (): int => 1]);
    byReference(static fn (): int => 1);
}

/** @param-closure-this \stdClass $callback */
function byReference(\Closure &$callback): void {}

function variadicAndRecursiveCalls(bool $choose): void
{
    $many = $choose ? __NAMESPACE__.'\\Binder::many' : __NAMESPACE__.'\\OtherBinder::many';
    $many(static fn (): int => 1);
    $recursive = function (\Closure $callback) use (&$recursive): int {
        return $recursive(static fn (): int => 1);
    };
    $result = $recursive(static fn (): int => 1);
}

class OtherBinder
{
    /** @param-closure-this \DateTime $callback */
    public static function many(\Closure ...$callback): void {}
}
