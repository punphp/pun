<?php

namespace CovHookClasses;

class C
{
    /** @var Missing */
    public $a {
        get => $this->a;
        set => $value;
    }

    public $b {
        /** @return Gone */
        get => 1;
        /** @param Gone2 $value */
        set(mixed $value) => $value;
    }

    /** @var foo */
    public $c {
        get => $this->c;
    }
}

class Foo
{
}

trait T
{
}

class D
{
    public function __construct(
        /** @var T */
        public $p {
            set => $value;
        },
    ) {
    }
}
