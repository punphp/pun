<?php

final readonly class A
{
    readonly public int $a;
    protected   readonly int $b;
    final readonly int $c;
    public static int $d;
    private readonly int $e, $f;

    public function __CONSTRUCT(
        readonly int $g,
        public readonly int $h,
        protected readonly int $i = 1,
        $j = 2,
        private(set) readonly int $k = 3,
    ) {
    }

    public function other(readonly int $l)
    {
    }
}

$o = new readonly class(1) {
    public function __construct(private readonly int $a)
    {
    }
};

class B
{
    public readonly int $a;
}
-----
<?php

final readonly class A
{
    public int $a;
    protected int $b;
    final public int $c;
    public static int $d;
    private int $e, $f;

    public function __CONSTRUCT(
        public int $g,
        public int $h,
        protected int $i = 1,
        $j = 2,
        private(set) int $k = 3,
    ) {
    }

    public function other(readonly int $l)
    {
    }
}

$o = new readonly class(1) {
    public function __construct(private int $a)
    {
    }
};

class B
{
    public readonly int $a;
}
