<?php

namespace ClosureThis;

use Closure;
use function ClosureThis\bindTarget as importedBinding;

class Target
{
    public function targetMethod(): void {}
    private function targetPrivate(): void {}
}

/** @param-closure-this Target $callback */
function bindTarget(Closure $callback): void {}

function untagged(Closure $callback): void {}

/** @param-closure-this object $callback */
function bindObject(Closure $callback): void {}

trait Macroable
{
    /** @param-closure-this static $macro */
    public static function macro(string $name, Closure $macro): void {}

    /** @param-closure-this self $macro */
    public static function macroSelf(Closure $macro): void {}
}

class Request
{
    use Macroable;

    public function input(): void {}
    public function validate(): void {}
}

class ChildRequest extends Request
{
    public function childMethod(): void {}
}

class Binder
{
    /** @param-closure-this Target $callback */
    public function bind(int $unused, Closure $callback): void {}

    /** @param-closure-this Target $callbacks */
    public static function many(Closure ...$callbacks): void {}

    /** @param-closure-this self $callback */
    public static function own(Closure $callback): void {}

    public function binderOnly(): void {}

    public function run(): void
    {
        $this->bind(1, function () { $this->targetMethod(); });
    }
}

class InheritedBinder extends Binder
{
    public function bind(int $unused, Closure $callback): void {}

    public function childOnly(): void {}
}

class Provider
{
    private function providerPrivate(): void {}
    public static function providerStatic(): void {}

    public function register(): void
    {
        Request::macro('validate', function () {
            $this->input();
            $this->validate();
            $this->input();
            $this->validate();
            $this->missingRequestMethod();
        });
        ChildRequest::macro(macro: function () { $this->childMethod(); }, name: 'child');
        ChildRequest::macroSelf(function () { $this->input(); $this->childMethod(); });
        bindTarget(function () { $this->targetMethod(); });
        importedBinding(fn () => $this->targetMethod());
        \ClosureThis\bindTarget((function () { $this->targetPrivate(); }));
        (new Binder())->bind(callback: function () { $this->targetMethod(); }, unused: 1);
        (new InheritedBinder())->bind(1, fn () => $this->targetMethod());
        Binder::many(function () { $this->targetMethod(); }, fn () => $this->targetMethod());
        Binder::many(extra: function () { $this->targetMethod(); });
        Binder::many(function () { $this->targetMethod(); }, ...[]);
        bindGlobal(fn () => $this->targetMethod());
        bindObject(function () { self::providerStatic(); });
        InheritedBinder::own(function () { $this->binderOnly(); $this->childOnly(); });
        bindTarget(function () {
            $this->targetPrivate();
            self::providerStatic();
            $this->providerPrivate();
            untagged(function () { $this->targetMethod(); });
            bindTarget(function () { $this->targetMethod(); });
        });
        untagged(function () { $this->targetMethod(); });
        untagged(fn () => $this->targetMethod());
        bindTarget(static function () { $this->targetMethod(); });
        bindTarget(static fn () => $this->targetMethod());
        $this->targetMethod();
        $this->providerPrivate();
        $accept = static function (Closure $callback): void {};
        $accept(function () { $this->targetMethod(); });
        $method = 'accept';
        $this->$method(function () { $this->targetMethod(); });
    }

    public function accept(Closure $callback): void {}
}

bindTarget(function () { $this->targetMethod(); });
bindTarget(fn () => $this->targetMethod());
bindTarget(static function () { $this->targetMethod(); });
bindTarget(static fn () => $this->targetMethod());

class LateRequest extends ChildRequest
{
    public function lateOnly(): void {}

    public function late(): void
    {
        static::macro('late', function () { $this->lateOnly(); });
        self::macro('self', fn () => $this->lateOnly());
        parent::macro('parent', function () { $this->lateOnly(); });
        $this->macro('object', function () { $this->lateOnly(); });
    }
}
