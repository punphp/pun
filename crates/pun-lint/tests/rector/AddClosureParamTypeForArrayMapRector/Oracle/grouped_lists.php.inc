<?php

namespace Probe;

final class Cron
{
    /**
     * @param  array<int, int>  $values
     */
    protected static function grouped(array $values, int $offset, int $mod)
    {
        $groups = [];

        foreach ($values as $value) {
            $carry = (int) floor($value / $mod);

            $groups[$carry][] = $value + $offset;
        }

        return array_map(function ($group) {
            sort($group);

            return $group;
        }, $groups);
    }

    /**
     * @param  array<int, int>  $values
     */
    protected static function keyed(array $values)
    {
        $groups = [];

        foreach ($values as $key => $value) {
            $groups[$key][] = $value;
        }

        return array_map(function ($group) {
            return $group;
        }, $groups);
    }

    /**
     * @param  array<int, int>  $values
     */
    protected static function flat(array $values)
    {
        $groups = [];

        foreach ($values as $value) {
            $groups[] = $value;
        }

        return array_map(function ($group) {
            return $group;
        }, $groups);
    }

    /**
     * @param  array<int, int>  $values
     */
    protected static function nested(array $values)
    {
        $groups = [];

        foreach ($values as $key => $value) {
            $groups[$key] = [$value];
        }

        return array_map(function ($group) {
            return $group;
        }, $groups);
    }
}
-----
<?php

namespace Probe;

final class Cron
{
    /**
     * @param  array<int, int>  $values
     */
    protected static function grouped(array $values, int $offset, int $mod)
    {
        $groups = [];

        foreach ($values as $value) {
            $carry = (int) floor($value / $mod);

            $groups[$carry][] = $value + $offset;
        }

        return array_map(function ($group) {
            sort($group);

            return $group;
        }, $groups);
    }

    /**
     * @param  array<int, int>  $values
     */
    protected static function keyed(array $values)
    {
        $groups = [];

        foreach ($values as $key => $value) {
            $groups[$key][] = $value;
        }

        return array_map(function ($group) {
            return $group;
        }, $groups);
    }

    /**
     * @param  array<int, int>  $values
     */
    protected static function flat(array $values)
    {
        $groups = [];

        foreach ($values as $value) {
            $groups[] = $value;
        }

        return array_map(function (int $group) {
            return $group;
        }, $groups);
    }

    /**
     * @param  array<int, int>  $values
     */
    protected static function nested(array $values)
    {
        $groups = [];

        foreach ($values as $key => $value) {
            $groups[$key] = [$value];
        }

        return array_map(function (array $group) {
            return $group;
        }, $groups);
    }
}
