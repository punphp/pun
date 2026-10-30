<?php

final class StaticCall
{
    public static function go(): void
    {
        self::run(['a', 'b']);
        static::run(['c']);
    }

    private static function run(array $items): void
    {
    }
}
-----
<?php

final class StaticCall
{
    public static function go(): void
    {
        self::run(['a', 'b']);
        static::run(['c']);
    }

    /**
     * @param string[] $items
     */
    private static function run(array $items): void
    {
    }
}
