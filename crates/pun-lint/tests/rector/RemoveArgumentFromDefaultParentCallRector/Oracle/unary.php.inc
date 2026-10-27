<?php

namespace Pun\Oracle\DefaultArgument;

class UnaryBase
{
    public function __construct($a = -1, $b = false, $c = 0)
    {
    }
}

final class UnaryNegated extends UnaryBase
{
    public function __construct()
    {
        parent::__construct(-1, !true, ~0);
    }
}

final class UnaryTrailing extends UnaryBase
{
    public function __construct()
    {
        parent::__construct(1, false, -0);
    }
}
-----
<?php

namespace Pun\Oracle\DefaultArgument;

class UnaryBase
{
    public function __construct($a = -1, $b = false, $c = 0)
    {
    }
}

final class UnaryNegated extends UnaryBase
{
    public function __construct()
    {
        parent::__construct(-1, !true, ~0);
    }
}

final class UnaryTrailing extends UnaryBase
{
    public function __construct()
    {
        parent::__construct(1);
    }
}
