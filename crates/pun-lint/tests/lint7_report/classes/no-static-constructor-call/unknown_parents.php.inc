<?php

namespace App;

class FromVendor extends \Vendor\Deep
{
    public function __construct()
    {
        parent::__construct();
        \Vendor\Base::__construct();
        \Vendor\Deep::__construct();
        self::__construct();
        FromVendor::__construct();
    }
}

class Plain
{
    public function __construct()
    {
        parent::__construct();
    }
}
-----
12: Static call to __construct() is only allowed on a parent class in the constructor.
13: Static call to __construct() is only allowed on a parent class in the constructor.
21: Static call to __construct() is only allowed on a parent class in the constructor.
