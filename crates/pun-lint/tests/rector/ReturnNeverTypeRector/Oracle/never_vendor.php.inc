<?php

trait HasStop
{
    abstract public function stop();
}

trait HasHalt
{
    public function halt()
    {
    }
}

final class UsesTraits
{
    use HasStop;
    use HasHalt;

    public function stop()
    {
        throw new \RuntimeException();
    }

    public function halt()
    {
        throw new \RuntimeException();
    }
}
-----
<?php

trait HasStop
{
    abstract public function stop();
}

trait HasHalt
{
    public function halt()
    {
    }
}

final class UsesTraits
{
    use HasStop;
    use HasHalt;

    public function stop(): never
    {
        throw new \RuntimeException();
    }

    public function halt(): never
    {
        throw new \RuntimeException();
    }
}
