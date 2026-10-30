<?php

namespace App\Instance;

final class Members
{
    public const LIMIT = 3;

    private int $count = 0;

    public function run(string $name, self $other)
    {
        self::$name();
        $other::helper();
        self::helper();
    }

    public static function make()
    {
        return array(1, 2);
    }

    private static function helper()
    {
        return self::LIMIT;
    }
}
-----
<?php

namespace App\Instance;

final class Members
{
    public const LIMIT = 3;

    private int $count = 0;

    public function run(string $name, self $other)
    {
        self::$name();
        $other::helper();
        $this->helper();
    }

    public static function make()
    {
        return array(1, 2);
    }

    private function helper()
    {
        return self::LIMIT;
    }
}
