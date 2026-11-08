<?php

namespace Pun\Oracle\EmptyMethod;

final class NestedNamedClass
{
    public function run()
    {
        class NestedEmptyInner
        {
            public function go()
            {
                return [self::class, 'other'];
            }
        }
    }

    private function other()
    {
    }
}
-----
<?php

namespace Pun\Oracle\EmptyMethod;

final class NestedNamedClass
{
    public function run()
    {
        class NestedEmptyInner
        {
            public function go()
            {
                return [self::class, 'other'];
            }
        }
    }
}
