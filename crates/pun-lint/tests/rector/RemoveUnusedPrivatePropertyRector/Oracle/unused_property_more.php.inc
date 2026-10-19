<?php

namespace App;

use Doctrine\ORM\Mapping\Column;

final class A
{
    private int $scalar;

    private Other $object;

    private $nullsafe;

    private $static;

    private $reference;

    /**
     * @Column(type="string")
     */
    private $doctrine;

    private int $hooked {
        set => $value;
    }

    public function run($x)
    {
        $this->scalar = 1;
        $this->object = new Other();
        $this->object->other = 1;
        $y = $this?->nullsafe;
        $z = self::$static;
        $this->reference = &$x;
        $this->doctrine = 1;
        $this->hooked = 1;
        $vars = get_object_vars($x);
    }
}

final class B
{
    private $value;

    public function run()
    {
        $this->value = 1;
        return get_object_vars($this);
    }
}
-----
<?php

namespace App;

use Doctrine\ORM\Mapping\Column;

final class A
{
    private Other $object;

    private $nullsafe;

    private $static;

    private $reference;

    /**
     * @Column(type="string")
     */
    private $doctrine;

    public function run($x)
    {
        $this->object = new Other();
        $this->object->other = 1;
        $y = $this?->nullsafe;
        $z = self::$static;
        $this->reference = &$x;
        $this->doctrine = 1;
        $vars = get_object_vars($x);
    }
}

final class B
{
    private $value;

    public function run()
    {
        $this->value = 1;
        return get_object_vars($this);
    }
}
