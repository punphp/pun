<?php

final class DocTags
{
    public function go()
    {
        return $this->f(1, 2) + $this->g(1, 2);
    }

    /**
     * @param int $a
     * @paramfoo
     * @param $this
     * @param int
     */
    private function f($a, $b)
    {
        return $a;
    }

    /**
     * @param int $a
     * @param int $b
     */
    private function g($a, $b)
    {
        return $a;
    }
}
-----
<?php

final class DocTags
{
    public function go()
    {
        return $this->f(1) + $this->g(1);
    }

    /**
     * @param int $a
     * @paramfoo
     * @param $this
     * @param int
     */
    private function f($a)
    {
        return $a;
    }

    /**
     * @param int $a
     */
    private function g($a)
    {
        return $a;
    }
}
