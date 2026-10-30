<?php

namespace App;

final class AttributesAndStatements
{
    /** @var int */
    #[Attr]
    private int $a;

    #[Attr]
    /** @var int */
    private int $b;

    /** @phpstan-var int */
    private int $c;

    /** @psalm-var int */
    private int $d;

    /**
     * @var int
     * @deprecated
     */
    private int $e;

    /** @var self */
    private self $f;

    /** @var static */
    private self $g;

    /** @var \stdClass */
    private object $h;

    /** @var object */
    private \stdClass $i;

    /** @var int */
    private int $j, $k;

    /** @var int */
    public const int TYPED = 1;

    public function run(): void
    {
        /** @var \stdClass $s */
        $s = new \stdClass();
        /** @var \stdClass */
        $t = new \stdClass();
        if (true) {
            /** @var \stdClass $n */
            $n = new \stdClass();
        }
        /** @var array $l */
        $l = [];
        /** @var \stdClass|null $z */
        $z = rand(0, 1) ? new \stdClass() : null;
        $f = function () {
            /** @var \stdClass $q */
            $q = new \stdClass();
        };
    }
}
-----
<?php

namespace App;

final class AttributesAndStatements
{
    #[Attr]
    private int $a;

    #[Attr]
    /** @var int */
    private int $b;

    /** @phpstan-var int */
    private int $c;

    /** @psalm-var int */
    private int $d;

    /**
     * @deprecated
     */
    private int $e;

    private self $f;

    private self $g;

    /** @var \stdClass */
    private object $h;

    /** @var object */
    private \stdClass $i;

    private int $j, $k;

    public const int TYPED = 1;

    public function run(): void
    {
        $s = new \stdClass();
        $t = new \stdClass();
        if (true) {
            $n = new \stdClass();
        }
        /** @var array $l */
        $l = [];
        $z = rand(0, 1) ? new \stdClass() : null;
        $f = function () {
            $q = new \stdClass();
        };
    }
}
