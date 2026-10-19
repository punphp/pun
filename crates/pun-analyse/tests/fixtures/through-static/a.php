<?php

namespace Through;

class ParentClass
{
    private static $parentSecret;

    private const PARENT_SECRET = 1;

    private static function parentHidden(): void
    {
    }
}

trait Helpers
{
    private static $traitSecret;

    private static function traitHidden(): void
    {
    }
}

class Open extends ParentClass
{
    use Helpers;

    private static $secret;

    public static $shared;

    private const SECRET = 1;

    public const SHARED = 1;

    private static function hidden(): void
    {
    }

    private function hiddenInstance(): void
    {
    }

    public function run(): void
    {
        echo static::$secret;
        echo static::$shared;
        echo static::$parentSecret;
        echo static::$traitSecret;
        echo static::$missing;
        echo static::SECRET;
        echo static::SHARED;
        echo static::PARENT_SECRET;
        static::hidden();
        static::hiddenInstance();
        static::parentHidden();
        static::traitHidden();
        echo self::$secret;
        echo Open::$secret;
        $f = fn () => static::$secret;
    }
}

final class Closed
{
    private static $secret;

    private const SECRET = 1;

    private static function hidden(): void
    {
    }

    public function run(): void
    {
        echo static::$secret;
        echo static::SECRET;
        static::hidden();
    }
}

/** @final */
class DocFinal
{
    private static $secret;

    public function run(): void
    {
        echo static::$secret;
    }
}

$anonymous = new class {
    private static $secret;

    public function run(): void
    {
        echo static::$secret;
    }
};
