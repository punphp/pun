<?php

final class NamedCompact
{
    public function go()
    {
        return $this->b(1, 2);
    }

    private function b($x, $y)
    {
        return compact(var_name: 'x', var_names: 'y');
    }
}
