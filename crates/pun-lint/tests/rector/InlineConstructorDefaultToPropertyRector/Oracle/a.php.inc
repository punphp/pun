<?php

final class A
{
    /**
     * Doc.
     */
    private $a = 0;
    private $b;
    private $c;

    public function __construct()
    {
        /**
         * Multi
         * line
         */
        $this->a = 1;

        $this->b = [
            'x' => 1,
        ];
        echo 1;
        $this->c = 3;
    }
}
-----
<?php

final class A
{
    /**
     * Doc.
     */
    /**
     * Multi
     * line
     */
    private $a = 1;
    private $b = [
        'x' => 1,
    ];
    private $c = 3;

    public function __construct()
    {
        echo 1;
    }
}
