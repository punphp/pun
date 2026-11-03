<?php

/*
 * This file is part of composer/semver.
 *
 * (c) Composer <https://github.com/composer>
 *
 * For the full copyright and license information, please view
 * the LICENSE file that was distributed with this source code.
 */

namespace Composer\Semver\Constraint;

/**
 * Defines a constraint.
 */
class Constraint implements ConstraintInterface
{
    /* operator integer values */
    const OP_EQ = 0;
    const OP_LT = 1;
    const OP_LE = 2;
    const OP_GT = 3;
    const OP_GE = 4;
    const OP_NE = 5;

    /* operator string values */
    const STR_OP_EQ = '==';
    const STR_OP_EQ_ALT = '=';
    const STR_OP_LT = '<';
    const STR_OP_LE = '<=';
    const STR_OP_GT = '>';
    const STR_OP_GE = '>=';
    const STR_OP_NE = '!=';
    const STR_OP_NE_ALT = '<>';

    /**
     * Operator to integer translation table.
     *
     * @var array
     * @phpstan-var array<self::STR_OP_*, self::OP_*>
     */
    private static $transOpStr = array(
        '=' => self::OP_EQ,
        '==' => self::OP_EQ,
        '<' => self::OP_LT,
        '<=' => self::OP_LE,
        '>' => self::OP_GT,
        '>=' => self::OP_GE,
        '<>' => self::OP_NE,
        '!=' => self::OP_NE,
    );

    /**
     * Integer to operator translation table.
     *
     * @var array
     * @phpstan-var array<self::OP_*, self::STR_OP_*>
     */
    private static $transOpInt = array(
        self::OP_EQ => '==',
        self::OP_LT => '<',
        self::OP_LE => '<=',
        self::OP_GT => '>',
        self::OP_GE => '>=',
        self::OP_NE => '!=',
    );

    /**
     * @var int
     * @phpstan-var self::OP_*
     */
    protected $operator;

    /** @var string */
    protected $version;

    /** @var string|null */
    protected $prettyString;

    /** @var Bound */
    protected $lowerBound;

    /** @var Bound */
    protected $upperBound;

    /**
     * Sets operator and version to compare with.
     *
     * @param string $operator
     * @param string $version
     *
     * @throws \InvalidArgumentException if invalid operator is given.
     *
     * @phpstan-param self::STR_OP_* $operator
     */
    public function __construct($operator, $version)
    {}

    /**
     * @return string
     */
    public function getVersion()
    {}

    /**
     * @return string
     *
     * @phpstan-return self::STR_OP_*
     */
    public function getOperator()
    {}

    /**
     * @param ConstraintInterface $provider
     *
     * @return bool
     */
    public function matches(ConstraintInterface $provider)
    {}

    /**
     * {@inheritDoc}
     */
    public function setPrettyString($prettyString)
    {}

    /**
     * {@inheritDoc}
     */
    public function getPrettyString()
    {}

    /**
     * Get all supported comparison operators.
     *
     * @return array
     *
     * @phpstan-return list<self::STR_OP_*>
     */
    public static function getSupportedOperators()
    {}

    /**
     * @param  string $operator
     * @return int
     *
     * @phpstan-param  self::STR_OP_* $operator
     * @phpstan-return self::OP_*
     */
    public static function getOperatorConstant($operator)
    {}

    /**
     * @param string $a
     * @param string $b
     * @param string $operator
     * @param bool   $compareBranches
     *
     * @throws \InvalidArgumentException if invalid operator is given.
     *
     * @return bool
     *
     * @phpstan-param self::STR_OP_* $operator
     */
    public function versionCompare($a, $b, $operator, $compareBranches = false)
    {}

    /**
     * {@inheritDoc}
     */
    public function compile($otherOperator)
    {}

    /**
     * @param Constraint $provider
     * @param bool       $compareBranches
     *
     * @return bool
     */
    public function matchSpecific(Constraint $provider, $compareBranches = false)
    {}

    /**
     * @return string
     */
    public function __toString()
    {}

    /**
     * {@inheritDoc}
     */
    public function getLowerBound()
    {}

    /**
     * {@inheritDoc}
     */
    public function getUpperBound()
    {}

    /**
     * @return void
     */
    private function extractBounds()
    {}
}
