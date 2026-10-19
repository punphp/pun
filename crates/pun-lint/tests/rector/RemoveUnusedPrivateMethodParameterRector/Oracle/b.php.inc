<?php

namespace App;

class B
{
    /**
     * Does things.
     *
     * @param int $a
     */
    #[Foo]
    private static function &one(int $a): ?array
    {
        // lead
        $x = 1;

        $y = 2;
        if ($x) {
            return null;
        }
        // tail
    }

    public function run()
    {
        $this->one(1);
        self::one(
            1
        );
        $this->two(a: 1);
        $this->three(1, 2);
    }

    private function two(int $a, int $b = 2)
    {
        echo $b;
    }

    /**
     * @param int $b
     *
     * @return void
     */
    private function three($a, $b)
    {
        echo $a;
    }
}

class C
{
    private function one(int $a, int $b)
    {
        echo $a;
    }

    public function run($x)
    {
        $this->one(1, 2);
        $x->one(1, 2);
        $f = [$this, 'one'];
    }
}
-----
<?php

namespace App;

class B
{
    /**
     * Does things.
     */
    #[Foo]
    private static function &one(): ?array
    {
        // lead
        $x = 1;
        $y = 2;
        if ($x) {
            return null;
        }
        // tail
    }

    public function run()
    {
        $this->one();
        self::one();
        $this->two();
        $this->three(1);
    }

    private function two(int $b = 2)
    {
        echo $b;
    }

    /**
     * @return void
     */
    private function three($a)
    {
        echo $a;
    }
}

class C
{
    private function one(int $a)
    {
        echo $a;
    }

    public function run($x)
    {
        $this->one(1);
        $x->one(1, 2);
        $f = [$this, 'one'];
    }
}
