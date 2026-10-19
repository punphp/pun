<?php

class A
{
    /**
     * Just text.
     */
    public function __construct() {}

    /* @return void */
    public function __destruct() {}

    /**
     * @return
     * @return void
     */
    public function __clone() {}
}

function f()
{
    $a = new class {
        /** @return void */
        public function __construct() {}
    };
}
-----
<?php

class A
{
    /**
     * Just text.
     */
    public function __construct() {}

    /* @return void */
    public function __destruct() {}

    /**
     * @return
     * @return void
     */
    public function __clone() {}
}

function f()
{
    $a = new class {
        public function __construct() {}
    };
}
