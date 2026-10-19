<?php

class A
{
    /**
     * Does things.
     *
     * @param int $a the a
     * @param int $b the b
     *               continued
     * @return void
     */
    private function one(int $a, int $b)
    {
        echo $a;
    }

    /**
     * @param int $b
     */
    private function two(
        int $a,
        int $b,
    ) {
        echo $a;

    }

    private function three(int $a, int $b = 5, $c = null)
    {

        echo $a;
    }

    public function run()
    {
        $this->one(1,
            2);
        $this->two(1, 2);
        $this->three(1, 2,
            3);
        static::three(1);
        $x->three(1, 2, 3);
    }

    /** @param int $x */
    private function four($x)
    {
    }

    // c
    private function five($x)
    {
        // body comment
    }
}
-----
<?php

class A
{
    /**
     * Does things.
     *
     * @param int $a the a
     * @return void
     */
    private function one(int $a)
    {
        echo $a;
    }

    private function two(
        int $a,
    ) {
        echo $a;

    }

    private function three(int $a)
    {

        echo $a;
    }

    public function run()
    {
        $this->one(1);
        $this->two(1);
        $this->three(1);
        static::three(1);
        $x->three(1, 2, 3);
    }

    private function four()
    {
    }

    // c
    private function five()
    {
        // body comment
    }
}
