<?php
class boundint { public function read(): int { return 1; } }
class boundstring { public function read(): string { return 'bound'; } }
/**
 * @param Closure(): mixed $callback
 * @param-closure-this boundint|boundstring $callback
 */
function bindRuntime($callback): void {}
class lexicalscope {
    public function read(): bool { return true; }
    /** @param-closure-this static $callback */
    public function bindStatic(Closure $callback): void {}
    public function exercise(): void {
        bindRuntime(function () { $this; self::class; return $this->read(); });
        bindRuntime(fn() => [$this, self::class, $this->read()]);
        bindRuntime(function () { return function () { return $this->read(); }; });
        bindRuntime(fn() => fn() => $this->read());
        $ordinary = function () { $this; return $this->read(); };
        $arrow = fn() => [$this, $this->read()];
        $static = static function () { self::class; };
        $staticArrow = static fn() => self::class;
        $this->bindStatic(function () { $this; return $this->read(); });
        $this->bindStatic(fn() => [$this, $this->read()]);
    }
}

// The standalone Analyzer scans source declarations rather than builtin stubs.
class Closure {
    public static function bind($callback, $newThis, $newScope = null) {}
}
function explicitBindings(boundint|boundstring $bound): void {
    Closure::bind(function () { $this; return $this->read(); }, $bound);
    Closure::bind(fn() => [$this, $this->read()], $bound);
    Closure::bind(function () { $this; return $this->read(); }, $bound, boundstring::class);
    Closure::bind(fn() => [$this, $this->read()], $bound, boundstring::class);
    Closure::bind(fn() => [$this, $this->read()], $bound, $bound);
    Closure::bind(function () { return $this->read(); }, null, boundstring::class);
    Closure::bind(function () {}, null);
}
