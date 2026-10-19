<?php

final class A
{
    public function run($x, $other)
    {
        $this->$x(1);
        $this->l(1, 2);
        $other->n(1, unused: 2);
    }

    private function l($unused, $used)
    {
        return $used;
    }

    private function n($used, $unused)
    {
        return $used;
    }
}
