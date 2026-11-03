<?php

namespace LaravelMax\Strings;

class Strings
{
    public function interpolated(string $name, mixed $config): void
    {
        $config["auth.guards.{$name}"]->foo();
    }

    /** @param non-empty-string $name */
    public function twoParts(string $name, mixed $config): void
    {
        $config["{$name}{$name}"]->foo();
    }

    public function zero(string $name, mixed $config): void
    {
        $config["0{$name}"]->foo();
    }

    /** @param literal-string $name */
    public function literal(string $name, mixed $config): void
    {
        $config["a{$name}"]->foo();
    }

    /** @param non-empty-string $s */
    public function concatChain(string $s, mixed $m, int $i): string
    {
        $a = $s.'@'.$m;
        $b = 'x'.$i.$m;
        $c = $s.'-'.$i.'-'.$m;

        return $a.$b.$c;
    }
}
