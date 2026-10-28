<?php

namespace Oracle\AssertInstanceof;

class Base
{
}

final class Child extends Base
{
}

final class Holder
{
    public function __construct(private Child $child, private ?Child $maybe = null)
    {
    }

    public function run(Child $c, Base $b, Child|Base $either, $untyped)
    {
        echo 0;
        /** keep this doc */
        assert($c instanceof Base);
        // plain comment
        assert($c instanceof Child);
        assert($b instanceof Child);
        assert($either instanceof Base);
        assert($this->child instanceof Child, 'message');
        assert($this->maybe instanceof Child);
        assert($untyped instanceof Child);
        assert(($c instanceof Child));
        \assert($c instanceof Child);
        echo 1;
        assert($c instanceof Child);
    }

    public function only(Child $c)
    {
        assert($c instanceof Child);
        assert($c instanceof Base);
    }
}
-----
<?php

namespace Oracle\AssertInstanceof;

class Base
{
}

final class Child extends Base
{
}

final class Holder
{
    public function __construct(private Child $child, private ?Child $maybe = null)
    {
    }

    public function run(Child $c, Base $b, Child|Base $either, $untyped)
    {
        echo 0;
        /** keep this doc */

        assert($b instanceof Child);
        assert($this->child instanceof Child, 'message');
        assert($this->maybe instanceof Child);
        assert($untyped instanceof Child);
        echo 1;
    }

    public function only(Child $c)
    {
    }
}
