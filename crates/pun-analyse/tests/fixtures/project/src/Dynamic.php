<?php

// `new $class` in a braced namespace, a class, a trait and an enum, beside functions, interfaces and methods that
// hold none: the class names come from the inferred types.
namespace Dynamic {
    function unrelated(): void
    {
        echo 'no instantiation';
    }

    function makes(): object
    {
        $class = 'Dynamic\MissingFromFunction';

        return new $class();
    }

    interface Unrelated
    {
        public function make(): object;
    }

    class Holder
    {
        use MakesThings;

        public function before(): void
        {
        }

        public function make(): object
        {
            $class = 'Dynamic\MissingFromClass';

            return new $class();
        }

        public function after(): void
        {
        }
    }

    class Untouched
    {
        public function make(): object
        {
            return new Holder();
        }
    }

    trait MakesThings
    {
        public function fromTrait(): object
        {
            $class = 'Dynamic\MissingFromTrait';

            return new $class();
        }

        public function unrelatedInTrait(): void
        {
        }
    }

    enum Kind
    {
        case One;

        public function make(): object
        {
            $class = 'Dynamic\MissingFromEnum';

            return new $class();
        }

        public function unrelatedInEnum(): void
        {
        }
    }
}
