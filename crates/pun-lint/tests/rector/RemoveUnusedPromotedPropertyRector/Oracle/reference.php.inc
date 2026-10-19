<?php

namespace App;

class ByReference
{
    public function __construct(private &$reference, private int &$typed)
    {
        echo $reference . $typed;
    }
}
-----
<?php

namespace App;

class ByReference
{
    public function __construct($reference, int &$typed)
    {
        echo $reference . $typed;
    }
}
