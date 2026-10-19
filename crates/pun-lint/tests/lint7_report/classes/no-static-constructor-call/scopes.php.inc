<?php

namespace App;

class Base
{
    public function __construct()
    {
    }
}

class Middle extends Base
{
}

class Child extends Middle
{
    public function __construct()
    {
        parent::__construct();
        Base::__construct();
        \App\Middle::__construct();
        self::__construct();
        static::__construct();
        Child::__construct();
        Other::__construct();
        $name = 'Base';
        $name::__construct();
        parent::__CONSTRUCT();
        parent::{'__construct'}();
        parent::other();
        $f = function () {
            parent::__construct();
        };
        $g = fn () => parent::__construct();
    }

    public function other()
    {
        parent::__construct();
    }
}

function __construct()
{
    Base::__construct();
}

Base::__construct();

new class extends Base {
    public function __construct()
    {
        parent::__construct();
        Middle::__construct();
    }
};

trait T
{
    public function __construct()
    {
        parent::__construct();
    }
}

class UsesT extends Base
{
    use T;
}

trait Unused
{
    public function __construct()
    {
        Base::__construct();
    }
}

class Upper extends Base
{
    public function __Construct()
    {
        parent::__construct();
    }
}

enum E
{
    public function __construct()
    {
        Base::__construct();
    }
}
-----
23: Static call to __construct() is only allowed on a parent class in the constructor.
24: Static call to __construct() is only allowed on a parent class in the constructor.
25: Static call to __construct() is only allowed on a parent class in the constructor.
26: Static call to __construct() is only allowed on a parent class in the constructor.
28: Static call to __construct() is only allowed on a parent class in the constructor.
40: Static call to __construct() is only allowed on a parent class in the constructor.
46: Static call to __construct() is only allowed on a parent class in the constructor.
49: Static call to __construct() is only allowed on a parent class in the constructor.
55: Static call to __construct() is only allowed on a parent class in the constructor.
84: Static call to __construct() is only allowed on a parent class in the constructor.
92: Static call to __construct() is only allowed on a parent class in the constructor.
