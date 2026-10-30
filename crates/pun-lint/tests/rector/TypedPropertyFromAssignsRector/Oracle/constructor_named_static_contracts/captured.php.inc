<?php

namespace ConstructorNamedTargets;

use ConstructorNamedTargets\NamedTargets as ImportedTarget;

final class NamedTargets
{
    private static $direct;
    private static $alias;
    private static $qualified;
    private static $helper;
    private static $items;
    private static $branches;
    private static $foreign;
    private static $dynamic;
    private static $compound;
    private static $assignedCall;
    private static $foreignCall;

    public function __construct(bool $flag, string $class)
    {
        NamedTargets::$direct = 1;
        ImportedTarget::$alias = 'local';
        \ConstructorNamedTargets\NamedTargets::$qualified = true;
        NamedTargets::initialize();
        NamedTargets::$items[] = 1;
        if ($flag) {
            NamedTargets::$branches = 'left';
        } else {
            NamedTargets::$branches = 'right';
        }
        ForeignTarget::$foreign = 2;
        $class::$dynamic = 3;
        NamedTargets::$compound += 4;
        $unused = NamedTargets::assigned();
        ForeignTarget::foreign();
    }

    private static function initialize(): void
    {
        NamedTargets::$helper = 5;
    }

    private static function assigned(): void
    {
        self::$assignedCall = 6;
    }

    private static function foreign(): void
    {
        self::$foreignCall = 7;
    }

    public static function later(): void
    {
        self::$foreign = 8;
        self::$dynamic = 9;
        self::$compound = 10;
    }
}

final class ForeignTarget
{
    public static int $foreign;

    public static function foreign(): void
    {
    }
}
-----
<?php

namespace ConstructorNamedTargets;

use ConstructorNamedTargets\NamedTargets as ImportedTarget;

final class NamedTargets
{
    private static int $direct;
    private static string $alias;
    private static bool $qualified;
    private static int $helper;
    private static array $items;
    private static string $branches;
    private static ?int $foreign = null;
    private static ?int $dynamic = null;
    private static ?int $compound = null;
    private static ?int $assignedCall = null;
    private static ?int $foreignCall = null;

    public function __construct(bool $flag, string $class)
    {
        NamedTargets::$direct = 1;
        ImportedTarget::$alias = 'local';
        \ConstructorNamedTargets\NamedTargets::$qualified = true;
        NamedTargets::initialize();
        NamedTargets::$items[] = 1;
        if ($flag) {
            NamedTargets::$branches = 'left';
        } else {
            NamedTargets::$branches = 'right';
        }
        ForeignTarget::$foreign = 2;
        $class::$dynamic = 3;
        NamedTargets::$compound += 4;
        $unused = NamedTargets::assigned();
        ForeignTarget::foreign();
    }

    private static function initialize(): void
    {
        NamedTargets::$helper = 5;
    }

    private static function assigned(): void
    {
        self::$assignedCall = 6;
    }

    private static function foreign(): void
    {
        self::$foreignCall = 7;
    }

    public static function later(): void
    {
        self::$foreign = 8;
        self::$dynamic = 9;
        self::$compound = 10;
    }
}

final class ForeignTarget
{
    public static int $foreign;

    public static function foreign(): void
    {
    }
}
