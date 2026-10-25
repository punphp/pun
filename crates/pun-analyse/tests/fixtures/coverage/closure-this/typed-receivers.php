<?php

namespace ClosureThis\Receivers;

use Closure;
use ClosureThis\Binder;
use ClosureThis\ChildRequest;
use ClosureThis\InheritedBinder;

class TypedRequest extends ChildRequest
{
    public function again(): static { return $this; }

    /** @return $this */
    public function same(): static { return $this; }

    public function aliases(): void
    {
        $alias = $this;
        $alias->macroSelf(function () { $this->childMethod(); });
        $this->again()->macroSelf(fn () => $this->childMethod());
        $this->same()->macroSelf(fn () => $this->childMethod());
    }
}

class Holder
{
    public function __construct(public InheritedBinder $binder, public ?TypedRequest $request) {}
}

function binder(): InheritedBinder { return new InheritedBinder(); }
function request(): TypedRequest { return new TypedRequest(); }

class Provider
{
    private function providerOnly(): void {}
    public static function providerStatic(): void {}

    public function run(
        InheritedBinder $binder,
        ?InheritedBinder $nullable,
        ?TypedRequest $request,
        Holder $holder,
        mixed $unknown,
        Binder|Untagged $union,
    ): void {
        $binder->bind(1, function () { $this->targetMethod(); });
        $binder->bind(callback: fn () => $this->targetMethod(), unused: 1);
        $nullable?->bind(1, function () { $this->targetMethod(); });
        $nullable->bind(1, fn () => $this->targetMethod());
        $holder->binder->bind(1, fn () => $this->targetMethod());
        binder()->bind(1, fn () => $this->targetMethod());
        (binder())->bind(1, (function () { $this->targetMethod(); }));
        $local = new InheritedBinder();
        $local->bind(1, fn () => $this->targetMethod());
        $binder->bind(1, function () {
            $this->targetPrivate();
            self::providerStatic();
            $this->providerOnly();
        });

        $request?->macro('nullable', function () { $this->childMethod(); });
        $request?->macro(macro: fn () => $this->childMethod(), name: 'named');
        $holder->request?->macro('property', fn () => $this->childMethod());
        request()->macro('returned', fn () => $this->childMethod());
        request()->again()->macro('static', fn () => $this->childMethod());
        request()->same()->macro('this', fn () => $this->childMethod());
        request()::macro('object-static', fn () => $this->childMethod());
        $request?->macroSelf(fn () => $this->childMethod());
        request()->macro('missing', fn () => $this->missingRequestMethod());

        // No level-2 method diagnostics, including calls whose closure has no binding tag.
        $binder->missing();
        $nullable?->missing();
        $binder->missing(fn () => $this->providerOnly());
        $nullable?->missing(fn () => $this->providerOnly());
        $unknown->bind(1, fn () => $this->providerOnly());
        $union->bind(1, fn () => $this->providerOnly());
        $untagged = new Untagged();
        $untagged->accept(function () { $this->targetMethod(); });
        $untagged?->accept(fn () => $this->targetMethod());
        $binder->bind(1, static function () { $this->targetMethod(); });
        $nullable?->bind(1, static fn () => $this->targetMethod());
        $this->targetMethod();
        $this->providerOnly();
    }
}

class Untagged
{
    public function bind(int $unused, Closure $callback): void {}
    public function accept(Closure $callback): void {}
}
