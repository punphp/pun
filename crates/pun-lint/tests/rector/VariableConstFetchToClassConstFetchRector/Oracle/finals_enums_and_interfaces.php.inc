<?php

namespace Oracle\ConstVar;

interface HasLimit
{
    public const LIMIT = 3;
}

final class Sealed implements HasLimit
{
    public const NAME = 'n';
}

class Open
{
    final public const FIXED = 1;
    public const LOOSE = 2;
}

class OpenChild extends Open
{
}

enum Suit: string
{
    case Hearts = 'h';
    public const DEFAULT = 'h';
}

/** @final */
class DocFinal
{
    public const NAME = 'd';
}

final class Runner
{
    public function run(Sealed $s, Open $o, OpenChild $c, Suit $suit, DocFinal $d, ?Sealed $maybe, object $any, string $name)
    {
        return [
            $s::NAME,
            $s::LIMIT,
            $s::MISSING,
            $o::FIXED,
            $o::LOOSE,
            $c::FIXED,
            $suit::DEFAULT,
            $suit::Hearts,
            $d::NAME,
            $maybe::NAME,
            $any::NAME,
            $s::class,
            $this::class,
        ];
    }
}
-----
<?php

namespace Oracle\ConstVar;

interface HasLimit
{
    public const LIMIT = 3;
}

final class Sealed implements HasLimit
{
    public const NAME = 'n';
}

class Open
{
    final public const FIXED = 1;
    public const LOOSE = 2;
}

class OpenChild extends Open
{
}

enum Suit: string
{
    case Hearts = 'h';
    public const DEFAULT = 'h';
}

/** @final */
class DocFinal
{
    public const NAME = 'd';
}

final class Runner
{
    public function run(Sealed $s, Open $o, OpenChild $c, Suit $suit, DocFinal $d, ?Sealed $maybe, object $any, string $name)
    {
        return [
            \Oracle\ConstVar\Sealed::NAME,
            \Oracle\ConstVar\Sealed::LIMIT,
            $s::MISSING,
            \Oracle\ConstVar\Open::FIXED,
            $o::LOOSE,
            \Oracle\ConstVar\OpenChild::FIXED,
            \Oracle\ConstVar\Suit::DEFAULT,
            \Oracle\ConstVar\Suit::Hearts,
            $d::NAME,
            $maybe::NAME,
            $any::NAME,
            $s::class,
            $this::class,
        ];
    }
}
