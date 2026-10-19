<?php

final class ReadonlyShapes
{
    /** @readonly */
    public readonly int $a;

    /** some text @readonly */
    public readonly int $b;

    /** @var int @readonly */
    public readonly int $c;

    /**
     * @readonly */
    public readonly int $d;

    /** @var int
     * @readonly */
    public readonly int $e;

    /**
     */
    public readonly int $f;

    /**  @readonly   @var int */
    public readonly int $g;
}
-----
<?php

final class ReadonlyShapes
{
    public readonly int $a;

    /** some text @readonly */
    public readonly int $b;

    /** @var int @readonly */
    public readonly int $c;

    public readonly int $d;

    /** @var int */
    public readonly int $e;

    /**
     */
    public readonly int $f;

    /**  @readonly   @var int */
    public readonly int $g;
}
