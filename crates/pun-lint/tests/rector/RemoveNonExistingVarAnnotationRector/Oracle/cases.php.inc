<?php

namespace Oracle\InvalidVar;

final class Holder
{
    // a comment
    /**
     * The method.
     */
    public function twoComments()
    {
        /** @var int $missing */
        return 1;
    }

    public function mixed($value)
    {
        /**
         * @phpstan-var int $missing
         * @var int $missing
         * @var string $value
         * @psalm-var int $missing
         */
        echo $value;

        /** @var int */
        echo $value;

        /** @var int */
        foo();

        /** @var int */
        $value = foo();

        /** @var Box<int> */
        return new Box();
    }

    public function otherNew()
    {
        /** @var Box<int> */
        return new Other();
    }

    public function dynamicExtract(array $data)
    {
        extract($data);
        /** @var int $nope */
        echo 1;
    }

    public function staticLater()
    {
        static $count = 0;
        /** @var int $count */
        echo 1;
        echo $count;
    }

    public function tryCatch()
    {
        try {
            /** @var \Exception $error */
            foo();
        } catch (\Exception $e) {
            echo $error;
        }
    }

    public function invalidType()
    {
        /** @var array<int $x */
        echo 1;
    }

    public function caseInsensitive($foo)
    {
        /** @var int $FOO */
        echo $foo;
    }

    public function twoTags($used)
    {
        /**
         * @var int $gone
         * @var string $used
         */
        echo $used;
    }

    public function nested($a)
    {
        if ($a) {
            /** @var int $b */
            while ($a) {
                $a--;
            }
        }

        switch ($a) {
            case 1:
                /** @var int $c */
                echo 1;
                break;
        }

        foreach ([1] as $item) {
            echo $item;
            /**
             * Text stays.
             * @var int $left
             */
        }
    }

    public function closureParam()
    {
        /** @var int $x */
        $f = function ($x) {
            return $x;
        };
    }

    public function withDescription()
    {
        /** @var int $nope Some words. */
        echo 1;
    }
}

final class Box
{
}
-----
<?php

namespace Oracle\InvalidVar;

final class Holder
{
    // a comment
    /**
     * The method.
     */
    public function twoComments()
    {
        /** @var int $missing */
        return 1;
    }

    public function mixed($value)
    {
        /**
         * @phpstan-var int $missing
         * @var string $value
         * @psalm-var int $missing
         */
        echo $value;

        echo $value;

        foo();

        /** @var int */
        $value = foo();

        return new Box();
    }

    public function otherNew()
    {
        return new Other();
    }

    public function dynamicExtract(array $data)
    {
        extract($data);
        /** @var int $nope */
        echo 1;
    }

    public function staticLater()
    {
        static $count = 0;
        /** @var int $count */
        echo 1;
        echo $count;
    }

    public function tryCatch()
    {
        try {
            /** @var \Exception $error */
            foo();
        } catch (\Exception $e) {
            echo $error;
        }
    }

    public function invalidType()
    {
        /** @var array<int $x */
        echo 1;
    }

    public function caseInsensitive($foo)
    {
        /** @var int $FOO */
        echo $foo;
    }

    public function twoTags($used)
    {
        /**
         * @var string $used
         */
        echo $used;
    }

    public function nested($a)
    {
        if ($a) {
            while ($a) {
                $a--;
            }
        }

        switch ($a) {
            case 1:
                echo 1;
                break;
        }

        foreach ([1] as $item) {
            echo $item;
            /**
             * Text stays.
             */

        }
    }

    public function closureParam()
    {
        /** @var int $x */
        $f = function ($x) {
            return $x;
        };
    }

    public function withDescription()
    {
        echo 1;
    }
}

final class Box
{
}
