<?php

namespace App;

class Elsewhere extends \Vendor\Base
{
    private const UNUSED = 1;
    private const BY_PARENT = 2;
    private const BY_OBJECT = 3;
    private const BY_OTHER = 4;

    public function run(self $same, \stdClass $other)
    {
        return [parent::BY_PARENT, $same::BY_OBJECT, $other::BY_OTHER];
    }
}
-----
<?php

namespace App;

class Elsewhere extends \Vendor\Base
{
    private const BY_OBJECT = 3;

    public function run(self $same, \stdClass $other)
    {
        return [parent::BY_PARENT, $same::BY_OBJECT, $other::BY_OTHER];
    }
}
