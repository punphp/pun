<?php

final class Registry
{
    /**
     * @var array<string, \Closure|string>
     */
    protected $wildcards = [];

    /**
     * @var array<string, string>
     */
    protected static $aliases = [];

    /**
     * @var array<string, string>
     */
    protected static $cache = [];

    public function listen($event, $listener)
    {
        $this->wildcards[$event][] = $listener;
    }

    public static function alias($alias, $driver)
    {
        static::$aliases[$alias] = $driver;
    }

    public static function remember($value)
    {
        return self::$cache[$value] = strtolower($value);
    }
}
-----
<?php

final class Registry
{
    /**
     * @var array<string, \Closure|string>
     */
    protected $wildcards = [];

    /**
     * @var array<string, string>
     */
    protected static $aliases = [];

    /**
     * @var array<string, string>
     */
    protected static $cache = [];

    public function listen(string $event, $listener)
    {
        $this->wildcards[$event][] = $listener;
    }

    public static function alias(string $alias, $driver)
    {
        static::$aliases[$alias] = $driver;
    }

    public static function remember(string $value)
    {
        return self::$cache[$value] = strtolower($value);
    }
}
