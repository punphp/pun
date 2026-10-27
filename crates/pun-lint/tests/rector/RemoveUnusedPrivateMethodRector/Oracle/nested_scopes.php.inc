<?php

namespace Pun\Oracle\UnusedPrivate;

class NestedBase
{
    public static function fromParent()
    {
    }
}

final class NestedScopes
{
    public function run()
    {
        return new class extends NestedBase {
            public function go()
            {
                self::helper();
                parent::fromParent();
            }

            private static function helper()
            {
            }
        };
    }

    private static function helper()
    {
    }

    private static function fromParent()
    {
    }
}
-----
<?php

namespace Pun\Oracle\UnusedPrivate;

class NestedBase
{
    public static function fromParent()
    {
    }
}

final class NestedScopes
{
    public function run()
    {
        return new class extends NestedBase {
            public function go()
            {
                self::helper();
                parent::fromParent();
            }

            private static function helper()
            {
            }
        };
    }

    private static function helper()
    {
    }
}
