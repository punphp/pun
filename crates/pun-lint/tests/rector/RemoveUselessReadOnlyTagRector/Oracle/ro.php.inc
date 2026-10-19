<?php

/** @readonly */
#[Attr]
final readonly class A
{
    /**
     * The name.
     *
     * @readonly
     */
    public readonly string $a;

    /**
     * @readonly
     * @readonly
     * @var string
     */
    public readonly string $b;

    /**
     * @psalm-readonly
     */
    public readonly string $c;

    /**
     * @readonly
     */
    public string $d;

    /**
     * @readonly
     *
     * Later text.
     */
    public readonly string $e;

    /** @readonly */
    public function __construct(
        /** @readonly */ public readonly int $x,
        /** @readonly */ int $y,
        /** @readonlyx */ public readonly int $z,
    ) {}
}

/**
 * @readonly
 */
class B
{
    public readonly int $x, $y;
}
-----
<?php

#[Attr]
final readonly class A
{
    /**
     * The name.
     */
    public readonly string $a;

    /**
     * @var string
     */
    public readonly string $b;

    /**
     * @psalm-readonly
     */
    public readonly string $c;

    /**
     * @readonly
     */
    public string $d;

    /**
     * @readonly
     *
     * Later text.
     */
    public readonly string $e;

    /** @readonly */
    public function __construct(
        public readonly int $x,
        /** @readonly */ int $y,
        /** @readonlyx */ public readonly int $z,
    ) {}
}

/**
 * @readonly
 */
class B
{
    public readonly int $x, $y;
}
