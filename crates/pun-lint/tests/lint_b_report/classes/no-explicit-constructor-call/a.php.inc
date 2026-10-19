<?php

class A {
    public function __construct() {}
    public function reset(): void {
        $this->__construct();
        $this->__CONSTRUCT();
        $this?->__construct();
        $this->construct();
        $m = '__construct';
        $this->$m();
        $c = $this->__construct(...);
        parent::__construct();
    }
}
(new A())->__construct();
-----
6: Call to __construct() on an existing object is not allowed.
7: Call to __construct() on an existing object is not allowed.
8: Call to __construct() on an existing object is not allowed.
16: Call to __construct() on an existing object is not allowed.
