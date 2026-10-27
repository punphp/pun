<?php

namespace Pun\Oracle\Ctor2;

class Base
{
    public function __construct($a)
    {
    }

    public function other($a)
    {
    }
}

class BaseInt
{
    public function __construct(int $a)
    {
    }
}

class NoCtor
{
}

class WithConstant extends Base
{
    const X = 1;

    public function __construct($a)
    {
        parent::__construct($a);
    }
}

abstract class AbstractCtor extends Base
{
    abstract public function __construct($a);
}

class UnknownParent extends \Pun\Oracle\Missing\Nowhere
{
    public function __construct($a)
    {
        parent::__construct($a);
    }
}

class ParentWithoutCtor extends NoCtor
{
    public function __construct($a)
    {
        parent::__construct($a);
    }
}

class EchoBody extends Base
{
    public function __construct($a)
    {
        echo $a;
    }
}

class OtherClassCall extends Base
{
    public function __construct($a)
    {
        Base::__construct($a);
    }
}

class OtherMethod extends Base
{
    public function __construct($a)
    {
        parent::other($a);
    }
}

class NamedArgument extends Base
{
    public function __construct($a)
    {
        parent::__construct(a: $a);
    }
}

class PrivatePromoted extends Base
{
    public function __construct(private $a)
    {
        parent::__construct($a);
    }
}

class ExtraUntypedDefault extends Base
{
    public function __construct($a, $b = 1)
    {
        parent::__construct($a, $b);
    }
}

class ExtraTyped extends BaseInt
{
    public function __construct(int $a, int $b)
    {
        parent::__construct($a, $b);
    }
}

class OtherType extends BaseInt
{
    public function __construct(string $a)
    {
        parent::__construct($a);
    }
}
-----
<?php

namespace Pun\Oracle\Ctor2;

class Base
{
    public function __construct($a)
    {
    }

    public function other($a)
    {
    }
}

class BaseInt
{
    public function __construct(int $a)
    {
    }
}

class NoCtor
{
}

class WithConstant extends Base
{
    const X = 1;
}

abstract class AbstractCtor extends Base
{
    abstract public function __construct($a);
}

class UnknownParent extends \Pun\Oracle\Missing\Nowhere
{
    public function __construct($a)
    {
        parent::__construct($a);
    }
}

class ParentWithoutCtor extends NoCtor
{
    public function __construct($a)
    {
        parent::__construct($a);
    }
}

class EchoBody extends Base
{
    public function __construct($a)
    {
        echo $a;
    }
}

class OtherClassCall extends Base
{
    public function __construct($a)
    {
        Base::__construct($a);
    }
}

class OtherMethod extends Base
{
    public function __construct($a)
    {
        parent::other($a);
    }
}

class NamedArgument extends Base
{
}

class PrivatePromoted extends Base
{
    public function __construct(private $a)
    {
        parent::__construct($a);
    }
}

class ExtraUntypedDefault extends Base
{
}

class ExtraTyped extends BaseInt
{
}

class OtherType extends BaseInt
{
    public function __construct(string $a)
    {
        parent::__construct($a);
    }
}
