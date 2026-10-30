<?php

namespace N;

use Override as Ov;

class Base
{
    public $a;
    public $b;
    public $c;
    public $d;
}

class Child extends Base
{
    #[Override]
    public $a;

    #[Ov]
    public $b;

    #[\override]
    public $c;

    #[Override\Sub] public $d;
}

class Orphan extends \Missing\Base
{
    public $a;
}

class OneLine extends Base { public $a; }
-----
<?php

namespace N;

use Override as Ov;

class Base
{
    public $a;
    public $b;
    public $c;
    public $d;
}

class Child extends Base
{
    #[Override]
    #[\Override]
    public $a;

    #[Ov]
    public $b;

    #[\override]
    public $c;

    #[Override\Sub]
    #[\Override] public $d;
}

class Orphan extends \Missing\Base
{
    public $a;
}

class OneLine extends Base { #[\Override]
public $a; }
