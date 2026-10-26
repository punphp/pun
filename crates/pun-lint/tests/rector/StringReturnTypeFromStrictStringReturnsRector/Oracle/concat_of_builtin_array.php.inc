<?php

final class Names
{
    /**
     * @param  string  $name
     */
    public static function direct($name)
    {
        return 'a.' . str_replace('/', '.', $name);
    }

    /**
     * @param  string  $name
     */
    public static function outer($name)
    {
        return 'a.' . str_replace('/', '.', $name) . '.b';
    }

    /**
     * @param  string  $name
     */
    public static function viaVariable($name)
    {
        $replaced = str_replace('/', '.', $name);

        return 'a.' . $replaced;
    }

    public static function typed(string $name)
    {
        return 'a.' . str_replace('/', '.', $name);
    }

    public static function stringReturn($name)
    {
        return 'a.' . substr($name, 1);
    }
}
-----
<?php

final class Names
{
    /**
     * @param  string  $name
     */
    public static function direct($name)
    {
        return 'a.' . str_replace('/', '.', $name);
    }

    /**
     * @param  string  $name
     */
    public static function outer($name): string
    {
        return 'a.' . str_replace('/', '.', $name) . '.b';
    }

    /**
     * @param  string  $name
     */
    public static function viaVariable($name)
    {
        $replaced = str_replace('/', '.', $name);

        return 'a.' . $replaced;
    }

    public static function typed(string $name)
    {
        return 'a.' . str_replace('/', '.', $name);
    }

    public static function stringReturn($name): string
    {
        return 'a.' . substr($name, 1);
    }
}
