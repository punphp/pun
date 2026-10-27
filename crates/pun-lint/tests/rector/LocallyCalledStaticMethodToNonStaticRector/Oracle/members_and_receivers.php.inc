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

    private function helper()
    {
        return self::LIMIT;
    }
}
