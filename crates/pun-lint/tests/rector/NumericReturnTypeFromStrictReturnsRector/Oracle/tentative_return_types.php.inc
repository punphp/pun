<?php

final class MyDate extends \DateTime
{
}

final class Probe
{
    public function tentative(MyDate $d)
    {
        return $d->getTimestamp();
    }

    public function real(MyDate $d)
    {
        return $d->getMicrosecond();
    }

    public function builtinTentative(\DateTime $d)
    {
        return $d->getTimestamp();
    }
}
-----
<?php

final class MyDate extends \DateTime
{
}

final class Probe
{
    public function tentative(MyDate $d)
    {
        return $d->getTimestamp();
    }

    public function real(MyDate $d): int
    {
        return $d->getMicrosecond();
    }

    public function builtinTentative(\DateTime $d): int
    {
        return $d->getTimestamp();
    }
}
