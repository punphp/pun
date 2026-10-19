<?php

class A
{
    /** @return void */
    public function __construct() {}

    /**
     * @return void
     * @param int $a
     */
    public function __destruct() {}

    /**
     * @return void Some text
     */
    public function __clone() {}

    /**
     * @return Void
     */
    public function __CONSTRUCT() {}

    /**
     * @psalm-return void
     * @return void
     */
    public function __clone() {}

    /**
     * @return void
     */
    public function __clone(): void {}

    /**
     * @return  void
     * @return void
     */
    public function __destruct() {}

    // a comment
    /**
     * @return void
     */
    public function __construct() {}

    /**
     * @return void
     */
    #[Attr]
    public function __construct() {}

    /**
     * Text.
     *
     * @return void
     *
     * More text.
     */
    public function __construct() {}

    /**
     * @return void
     */
    public function other() {}
}

interface I
{
    /**
     * @return void
     */
    public function __construct();
}
-----
<?php

class A
{
    public function __construct() {}

    /**
     * @param int $a
     */
    public function __destruct() {}

    /**
     * @return void Some text
     */
    public function __clone() {}

    /**
     * @return Void
     */
    public function __CONSTRUCT() {}

    /**
     * @psalm-return void
     */
    public function __clone() {}

    /**
     * @return void
     */
    public function __clone(): void {}

    public function __destruct() {}

    // a comment
    public function __construct() {}

    #[Attr]
    public function __construct() {}

    /**
     * Text.
     *
     * @return void
     *
     * More text.
     */
    public function __construct() {}

    /**
     * @return void
     */
    public function other() {}
}

interface I
{
    public function __construct();
}
