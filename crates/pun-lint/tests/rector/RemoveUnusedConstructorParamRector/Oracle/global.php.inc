<?php

class GlobalClass
{
    public function __construct($unused)
    {
    }
}
-----
<?php

class GlobalClass
{
    public function __construct()
    {
    }
}
