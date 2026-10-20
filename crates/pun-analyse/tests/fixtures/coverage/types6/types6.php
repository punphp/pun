<?php

namespace Types6;

use Closure;

class Foo
{
    private int $x = 1;

    private static int $y = 2;

    /**
     * @param callable(): int $callable
     * @param (Closure(): int)|null $n
     */
    public function bound(Closure $c, callable $callable, ?object $maybe, ?Closure $n, string $name): void
    {
        $_ = Closure::bind(function (): int { return $this->x; }, $this);
        $_ = Closure::bind(function (): int { return $this->x; }, ($this));
        $_ = Closure::bind(function (): int { return $this->x; }, new self());
        $_ = Closure::bind(function (): int { return $this->x; }, new static());
        $_ = Closure::bind(function (): int { return $this->x; }, new Foo());
        $_ = Closure::bind(function (): int { return 1; }, $c);
        $_ = Closure::bind(static fn (): int => self::$y, null, Foo::class);
        $_ = Closure::bind(static fn (): int => self::$y, null, self::class);
        $_ = Closure::bind(static fn (): int => self::$y, null, static::class);
        $_ = Closure::bind(static fn (): int => self::$y, null, $this::class);
        $_ = Closure::bind(static fn (): int => self::$y, null, 'Types6\Foo');
        $_ = Closure::bind(static fn (): int => self::$y, null, 'static');
        $_ = Closure::bind(static fn (): int => self::$y, null, $this);
        $_ = (function (): int { return $this->x; })->call($this);
        $_ = (function (): int { return 1; })->call(...);

        $f = function (): int { return $this->x; };
        $s = static fn (): int => 1;
        $_ = $f->bindTo($this)() + 1;
        $_ = Closure::bind($f, $this)() + 1;
        $_ = $s->bindTo(null)() + 1;
        $_ = $s->bindTo($this);
        $_ = $s->bindTo($maybe);
        $_ = $this->closure()->bindTo($this);
        $_ = $c?->bindTo($this);
        $_ = Closure::bind($c, $this);
        $_ = Closure::bind(Closure::fromCallable($callable), $this);
        $_ = Closure::bind(strlen(...), $this);
        $_ = Closure::bind($n, $this);
        $_ = Closure::bind($callable, $this);
        $_ = $s->bindTo();
        $_ = Closure::bind(function (): int { return 1; }, new $name());
    }

    public function closure(): Closure
    {
        return fn () => 1;
    }
}

function benevolent(): void
{
    $ch = curl_init();
    curl_setopt($ch, CURLOPT_URL, 'https://example.com');
    $_ = scandir('.');
}
