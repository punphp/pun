<?php

namespace PhpDocTypes;

enum Suit: string
{
    case Hearts = 'h';
}

/**
 * @phpstan-type Pair array{int, int}
 */
class Base
{
    /** @var Pair */
    public array $pair = [0, 0];

    /** @var string|null */
    const NAME = 'base';

    /** @var string|null */
    const OTHER = 'other';
}

class Child extends Base
{
    const string NAME = 'child';

    const ?string OTHER = 'child';

    /** @var int */
    const string THIRD = 'x';

    /** @var string|int */
    const string FOURTH = 'x';

    /** @var Base<int> */
    const ?Base LAST = null;

    /** @var int */
    public string $wrong = '';

    /** @var value-of<Suit> */
    public string $unmodelled = '';

    /** @var string $named */
    public string $named = '', $other = '';

    /**
     * @param int ...$numbers
     * @param array<int> $list
     * @param-out int $out
     * @param-out int $notRef
     * @param-closure-this \stdClass $cb
     * @param-closure-this \stdClass $notClosure
     * @param-closure-this \stdClass $missing
     * @param-out int $missingOut
     * @param value-of<Suit> $v
     * @return int
     */
    public function params(\Closure $cb, int $notClosure, &$out, $notRef, string $v, array $list = [], int ...$numbers): string
    {
        return '';
    }

    /**
     * @template T
     * @param T $t
     * @return T
     */
    public function template(string $t): string
    {
        return $t;
    }

    /**
     * @return Foo(
     */
    public function invalidReturn(): int
    {
        return 1;
    }

    /**
     * @return key-of<array{a: 1}>
     */
    public function unmodelledReturn(): string
    {
        return 1;
    }

    /**
     * @param string $promoted
     * @param key-of<array{a: 1}> $unmodelledParam
     */
    public function __construct(
        /** @var int */
        public string $own,
        public int $promoted,
        /** @var value-of<Suit> */
        public string $unmodelledVar,
        public string $unmodelledParam,
    ) {
    }
}

class Hooks
{
    public int $get {
        /** @return string */
        get {
            return 1;
        }
    }

    public int $set {
        /** @param string $value */
        set {
            $this->set = 1;
        }
    }

    public int $typed {
        /** @param string $value */
        set(int $value) {
            $this->typed = $value;
        }
    }

    public $untyped {
        /** @param string $value */
        set($value) {
            $this->untyped = $value;
        }
    }
}

/**
 * @param int $x
 * @return string
 */
function plain(string $x = null): int
{
    return 1;
}
