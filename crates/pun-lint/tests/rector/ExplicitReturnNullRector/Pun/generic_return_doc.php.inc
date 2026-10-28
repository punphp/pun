<?php

namespace Pun\ExplicitNull;

final class Runner
{
    /**
     * @return array<int, string>|void the list
     */
    public function generic(int $n)
    {
        if ($n > 1) {
            return ['a'];
        }
    }
}
