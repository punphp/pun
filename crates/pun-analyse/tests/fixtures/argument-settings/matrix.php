<?php

declare(strict_types=1);

namespace ArgumentSettings;

function typed(int &$value): void {}

/** @no-named-arguments */
function policy(int &$value): void {}

class Base
{
    public function method(int &$old): void {}

    /** @no-named-arguments */
    public function policy(int &$value): void {}

    public static function stat(int &$value): void {}

    public function exercise(Base $other): void
    {
        $this->method(old: 'bad');
        $other->method(old: 'bad');
        $this->policy(value: 'bad');
        $other->policy(value: 'bad');
        $this->method(unknown: 1);
        $other->method(unknown: 1);
        $this->method();
        $other->method();
        $this->method(...1);
        $other->method(...1);
        $other::stat(value: 'bad');
        self::stat(value: 'bad');
        typed(value: 'bad');
        policy(value: 'bad');
        typed(unknown: 1);
        typed();
        typed(1, value: 2);
        $value = 1;
        typed(value: $value);
        $this->method(old: $value);
        $other->method(old: $value);
    }
}

class Child extends Base
{
    public function method(int &$new): void {}
}
