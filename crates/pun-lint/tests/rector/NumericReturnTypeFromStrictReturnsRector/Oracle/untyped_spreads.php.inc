<?php

class Moment extends \DateTime
{
}

final class Propagated
{
    public function ternary(Moment $m, bool $flag)
    {
        return $flag ? $m->getTimestamp() : time();
    }

    public function short(Moment $m)
    {
        return $m->getTimestamp() ?: 0;
    }

    public function parenthesized(Moment $m)
    {
        return ($m->getTimestamp());
    }

    public function scaled(Moment $m)
    {
        return $m->getTimestamp() * 1000;
    }

    public function typed(bool $flag)
    {
        return $flag ? time() : 0;
    }
}
-----
<?php

class Moment extends \DateTime
{
}

final class Propagated
{
    public function ternary(Moment $m, bool $flag): int
    {
        return $flag ? $m->getTimestamp() : time();
    }

    public function short(Moment $m): int
    {
        return $m->getTimestamp() ?: 0;
    }

    public function parenthesized(Moment $m): int
    {
        return ($m->getTimestamp());
    }

    public function scaled(Moment $m): int
    {
        return $m->getTimestamp() * 1000;
    }

    public function typed(bool $flag): int
    {
        return $flag ? time() : 0;
    }
}
