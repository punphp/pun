<?php

namespace App;

class UnreadVar
{
    /** @var array{a: int} */
    private $shape;
    /** @var 'abc' */
    private $literal;
    /** @var array<int, string> */
    public $publicGeneric;
    /**
     * @var string
     *   on two lines
     */
    private $twoLines;

    public function __construct(array $shape, string $literal, array $publicGeneric, string $twoLines)
    {
        $this->shape = $shape;
        $this->literal = $literal;
        $this->publicGeneric = $publicGeneric;
        $this->twoLines = $twoLines;
    }
}
-----
<?php

namespace App;

class UnreadVar
{
    /** @var array{a: int} */
    private array $shape;
    /** @var 'abc' */
    private string $literal;
    /** @var array<int, string> */
    public $publicGeneric;
    /**
     * @var string
     *   on two lines
     */
    private string $twoLines;

    public function __construct(array $shape, string $literal, array $publicGeneric, string $twoLines)
    {
        $this->shape = $shape;
        $this->literal = $literal;
        $this->publicGeneric = $publicGeneric;
        $this->twoLines = $twoLines;
    }
}
