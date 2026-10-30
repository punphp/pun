<?php

namespace App;

class Literals
{
    public $quote;
    public $whole;
    public $empty;
    public $small;
    public $negative;
    public $falsy;

    public function __construct()
    {
        $this->quote = 'it\'s \\ x';
        $this->whole = 2.0;
        $this->empty = '';
        $this->small = 0.001;
        $this->negative = -2.5;
        $this->falsy = false;
    }
}
-----
<?php

namespace App;

class Literals
{
    /**
     * @var 'it\'s \\ x'
     */
    public $quote;
    /**
     * @var 2.0
     */
    public $whole;
    /**
     * @var ''
     */
    public $empty;
    /**
     * @var 0.001
     */
    public $small;
    /**
     * @var -2.5
     */
    public $negative;
    /**
     * @var false
     */
    public $falsy;

    public function __construct()
    {
        $this->quote = 'it\'s \\ x';
        $this->whole = 2.0;
        $this->empty = '';
        $this->small = 0.001;
        $this->negative = -2.5;
        $this->falsy = false;
    }
}
