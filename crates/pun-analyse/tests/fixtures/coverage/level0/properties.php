<?php

namespace CoverageProperties;

class Base
{
    private int $hidden = 1;
}

class Child extends Base
{
    public static int $count = 0;

    function read(): int
    {
        return $this->hidden;
    }

    function destructure(string $name): void
    {
        [, $this->missing] = [1, 2];
        [[, $this->nested]] = [[1, 2]];
        if (isset(self::$$name)) {
            echo self::$count;
        }
        $other = new self();
        echo $other::$count;
    }
}

class Response extends \HttpResponse
{
    function cache(): mixed
    {
        return \HttpResponse::$cache;
    }
}

class PrivateParent
{
    private int $hidden = 1;

    private static int $staticHidden = 1;
}

class PrivateChild extends PrivateParent
{
    public function read(): void
    {
        echo $this->hidden;
        echo self::$staticHidden;
        echo self::$hidden;
        echo $this->staticHidden;
    }
}
