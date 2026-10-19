<?php

final class UnionShapes
{
    public function nothing()
    {
    }

    public function withVoid($x)
    {
        if ($x) {
            return;
        }
        return 1;
    }
}
