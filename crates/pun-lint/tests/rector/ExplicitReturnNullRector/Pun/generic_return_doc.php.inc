<?php

namespace Pun\ExplicitNull;

class Base
{
}

final class Runner extends Base
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

    /** @return parent|void */
    public function parent(int $n)
    {
        if ($n > 1) {
            return new Base();
        }
    }

    public function anonymous()
    {
        return new class {
            /** @return self|void */
            public function inner(int $n)
            {
                if ($n > 1) {
                    return 1;
                }
            }
        };
    }
}
