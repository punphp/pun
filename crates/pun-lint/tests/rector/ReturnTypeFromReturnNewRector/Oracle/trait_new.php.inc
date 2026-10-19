<?php

namespace App;

class Coll
{
}

trait T
{
    public function a()
    {
        $c = new Coll;

        return $c;
    }

    public function b()
    {
        return new Coll;
    }

    public function c(string $s)
    {
        return $s;
    }

    public function d(array $s)
    {
        return [];
    }

    public function e()
    {
        return $this;
    }
}
-----
<?php

namespace App;

class Coll
{
}

trait T
{
    public function a()
    {
        $c = new Coll;

        return $c;
    }

    public function b(): \App\Coll
    {
        return new Coll;
    }

    public function c(string $s)
    {
        return $s;
    }

    public function d(array $s)
    {
        return [];
    }

    public function e()
    {
        return $this;
    }
}
