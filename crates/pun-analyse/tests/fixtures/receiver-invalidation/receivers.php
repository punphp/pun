<?php

namespace ReceiverInvalidation;

use function PHPStan\dumpType;

class Box
{
    public ?int $value;
    public readonly ?int $fixed;

    /** @phpstan-impure */
    public function mutate(): int { $this->value = null; return 1; }

    /** @phpstan-pure */
    public function pure(): int { return 1; }

    public function unknown(): int { return 1; }

    public function nothing(): void { $this->value = null; }

    /** @phpstan-impure */
    public static function staticMutation(): int { return 1; }

    /** @phpstan-impure
     * @return 'foo'|'bar'
     */
    public function choose(): string { $this->value = null; return rand() ? 'foo' : 'bar'; }
}

function direct(Box $box, Box $boxes): void
{
    if ($box->value !== null && $boxes->value !== null) {
        $box->mutate();
        dumpType($box->value);
        dumpType($boxes->value);
    }
    if ($box->value !== null) {
        $box->pure();
        dumpType($box->value);
    }
    if ($box->value !== null) {
        $box->unknown();
        dumpType($box->value);
    }
    if ($box->value !== null) {
        $box->nothing();
        dumpType($box->value);
    }
    if ($box->value !== null) {
        $box->staticMutation();
        dumpType($box->value);
    }
    if ($box->fixed !== null) {
        $box->mutate();
        dumpType($box->fixed);
    }
}

class Sink
{
    public function foo($x, $y): void {}
    public function bar($x, $y): void {}
}

function arguments(Sink $sink, Box $box, bool $flag): void
{
    $method = $flag ? 'foo' : 'bar';
    if ($box->value !== null) {
        $sink->$method($box->mutate(), $box->value);
        dumpType($box->value);
    }
    if ($box->value !== null) {
        $sink->$method($box->pure(), $box->value);
        dumpType($box->value);
    }
    if ($box->value !== null) {
        $sink->{$box->choose()}($box->value, [$box->value]);
        dumpType($box->value);
    }
}

/** @param array{one: Box, two: Box} $boxes */
function offset(array $boxes): void
{
    if ($boxes['one']->value !== null && $boxes['two']->value !== null) {
        $boxes['one']->mutate();
        dumpType($boxes['one']->value);
        dumpType($boxes['two']->value);
    }
}

class Wrapper
{
    public Box $box;
    public readonly Box $fixed;
}

function nested(Wrapper $wrapper): void
{
    if ($wrapper->box->value !== null) {
        $wrapper->box->mutate();
        dumpType($wrapper->box->value);
        dumpType($wrapper->box);
    }
    if ($wrapper->fixed->value !== null) {
        $wrapper->fixed->mutate();
        dumpType($wrapper->fixed->value);
        dumpType($wrapper->fixed);
    }
}

class ThisBox extends Box
{
    public function observe(): void
    {
        if ($this->fixed !== null && $this->value !== null) {
            $this->mutate();
            dumpType($this->fixed);
            dumpType($this->value);
        }
    }
}

class FluentBox extends Box
{
    /** @return $this */
    public function fluent() { $this->value = null; return $this; }

    /** @phpstan-pure
     * @return $this
     */
    public function pureFluent() { return $this; }

    /** @phpstan-pure */
    public function pureVoid(): void {}
}

function fluent(FluentBox $box): void
{
    if ($box->value !== null) {
        $box->fluent();
        dumpType($box->value);
    }
    if ($box->value !== null) {
        $box->pureFluent();
        dumpType($box->value);
    }
    if ($box->value !== null) {
        $box->pureVoid();
        dumpType($box->value);
    }
}

class ConstructedBox extends Box
{
    /** @phpstan-pure */
    public function __construct() { $this->fixed = 1; }
}

function constructor(ConstructedBox $box): void
{
    if ($box->value !== null) {
        $box->__construct();
        dumpType($box->value);
    }
}

function conditional(Box $box, bool $flag): void
{
    if ($flag) { $method = 'foo'; $box->value = 1; }
    else { $method = 'bar'; $box->value = null; }
    $box->mutate();
    if ($method === 'foo') {
        dumpType($box->value);
    }
}

class PrivateBox extends Box
{
    private ?int $hidden;

    /** @phpstan-impure */
    public function ownMutation(): int { $this->hidden = null; return 1; }

    public function inspect(PrivateBox $other): void
    {
        if ($other->hidden !== null) {
            $other->mutate();
            dumpType($other->hidden);
        }
        if ($other->hidden !== null) {
            $other->ownMutation();
            dumpType($other->hidden);
        }
    }
}

trait Mutator
{
    /** @phpstan-impure */
    public function traitMutation(): int { return 1; }
}

class TraitBox
{
    use Mutator { traitMutation as aliasMutation; }
    private ?int $hidden;

    public function inspect(TraitBox $other): void
    {
        if ($other->hidden !== null) {
            $other->traitMutation();
            dumpType($other->hidden);
        }
        if ($other->hidden !== null) {
            $other->aliasMutation();
            dumpType($other->hidden);
        }
    }
}

class SelfBox extends Box
{
    private ?int $hidden;

    public function ownUnknown(): int { return 1; }

    public function inspectSelf(): void
    {
        if ($this->value !== null) {
            $this->ownUnknown();
            dumpType($this->value);
        }
        if ($this->value !== null) {
            $this->staticMutation();
            dumpType($this->value);
        }
        if ($this->hidden !== null) {
            $this->mutate();
            dumpType($this->hidden);
        }
    }
}

function readonlyGuard(Box $box): void
{
    if ($box->fixed === 1) { $tag = 'foo'; }
    else { $tag = 'bar'; }
    $box->mutate();
    if ($box->fixed === 1) {
        dumpType($tag);
    }
}

trait ReturningMutator
{
    /** @return $this */
    public function fluent(): object { return $this; }

    /** @return void */
    public function documentedVoid(): mixed { return null; }

    public function boot() {}
}

class ReturningBox extends Box
{
    use ReturningMutator {
        fluent as aliasFluent;
        documentedVoid as aliasVoid;
        boot as __construct;
    }
}

class ReturningChild extends ReturningBox {}

function aliasedReturns(ReturningChild $box): void
{
    if ($box->value !== null) {
        $box->fluent();
        dumpType($box->value);
    }
    if ($box->value !== null) {
        $box->aliasFluent();
        dumpType($box->value);
    }
    if ($box->value !== null) {
        $box->documentedVoid();
        dumpType($box->value);
    }
    if ($box->value !== null) {
        $box->aliasVoid();
        dumpType($box->value);
    }
    if ($box->value !== null) {
        $box->boot();
        dumpType($box->value);
    }
    if ($box->value !== null) {
        $box->__construct();
        dumpType($box->value);
    }
}
