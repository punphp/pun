<?php

namespace Pun\Oracle\Ctor;

class Base
{
    /**
     * @param int $count
     */
    public function __construct(string $name = 'x', ?\DateTimeInterface $at = null, $count = 0)
    {
    }
}

class Same extends Base
{
    public function __construct(string $name = 'x', ?\DateTimeInterface $at = null, int $count = 0)
    {
        parent::__construct($name, $at, $count);
    }
}

class DifferentDefault extends Base
{
    public function __construct(string $name = 'y')
    {
        parent::__construct($name);
    }
}

class Untyped extends Base
{
    public function __construct($name = 'x')
    {
        parent::__construct($name);
    }
}

class UntypedDifferent extends Base
{
    public function __construct($name = 'z')
    {
        parent::__construct($name);
    }
}

class Protected_ extends Base
{
    protected function __construct(string $name)
    {
        parent::__construct($name);
    }
}

class OtherOrder extends Base
{
    public function __construct(string $name, ?\DateTimeInterface $at)
    {
        parent::__construct($at, $name);
    }
}
-----
<?php

namespace Pun\Oracle\Ctor;

class Base
{
    /**
     * @param int $count
     */
    public function __construct(string $name = 'x', ?\DateTimeInterface $at = null, $count = 0)
    {
    }
}

class Same extends Base
{
}

class DifferentDefault extends Base
{
    public function __construct(string $name = 'y')
    {
        parent::__construct($name);
    }
}

class Untyped extends Base
{
}

class UntypedDifferent extends Base
{
    public function __construct($name = 'z')
    {
        parent::__construct($name);
    }
}

class Protected_ extends Base
{
    protected function __construct(string $name)
    {
        parent::__construct($name);
    }
}

class OtherOrder extends Base
{
    public function __construct(string $name, ?\DateTimeInterface $at)
    {
        parent::__construct($at, $name);
    }
}
