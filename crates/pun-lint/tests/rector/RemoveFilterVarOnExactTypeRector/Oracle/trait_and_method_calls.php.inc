<?php

namespace App\FilterVar;

trait Values
{
    private int $count = 0;

    public function fromTrait()
    {
        return filter_var($this->count, FILTER_VALIDATE_INT);
    }
}

final class Holder
{
    use Values;

    private int $number = 0;

    public function fromMethod()
    {
        return $this->other(filter_var($this->number, FILTER_VALIDATE_INT));
    }

    private function other($value)
    {
        return $value;
    }
}
-----
<?php

namespace App\FilterVar;

trait Values
{
    private int $count = 0;

    public function fromTrait()
    {
        return filter_var($this->count, FILTER_VALIDATE_INT);
    }
}

final class Holder
{
    use Values;

    private int $number = 0;

    public function fromMethod()
    {
        return $this->other($this->number);
    }

    private function other($value)
    {
        return $value;
    }
}
