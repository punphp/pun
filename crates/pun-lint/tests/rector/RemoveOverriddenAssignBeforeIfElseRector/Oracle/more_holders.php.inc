<?php

namespace Rector\Tests\DeadCode\Rector\If_\RemoveOverriddenAssignBeforeIfElseRector\Oracle;

final class MoreHolders
{
    public string $hooked {
        get {
            $value = 'a';
            if ($this->flag) {
                $value = 'b';
            } else {
                $value = 'c';
            }
            return $value;
        }
    }

    public bool $flag = false;

    public function dynamicStatic($name)
    {
        $value = 1;
        if (self::${$name}) {
            $value = 2;
        } else {
            $value = 3;
        }
        return $value;
    }

    public function dynamicStaticUsesIt($value)
    {
        $value = 'name';
        if (self::${$value}) {
            $value = 2;
        } else {
            $value = 3;
        }
        return $value;
    }
}
-----
<?php

namespace Rector\Tests\DeadCode\Rector\If_\RemoveOverriddenAssignBeforeIfElseRector\Oracle;

final class MoreHolders
{
    public string $hooked {
        get {
            $value = 'a';
            if ($this->flag) {
                $value = 'b';
            } else {
                $value = 'c';
            }
            return $value;
        }
    }

    public bool $flag = false;

    public function dynamicStatic($name)
    {
        if (self::${$name}) {
            $value = 2;
        } else {
            $value = 3;
        }
        return $value;
    }

    public function dynamicStaticUsesIt($value)
    {
        $value = 'name';
        if (self::${$value}) {
            $value = 2;
        } else {
            $value = 3;
        }
        return $value;
    }
}
