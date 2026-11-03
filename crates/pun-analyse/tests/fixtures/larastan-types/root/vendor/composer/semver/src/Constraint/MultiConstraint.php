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
 * Defines a conjunctive or disjunctive set of constraints.
 */
class MultiConstraint implements ConstraintInterface
{
    /**
     * @var ConstraintInterface[]
     * @phpstan-var non-empty-array<ConstraintInterface>
     */
    protected $constraints;

    /** @var string|null */
    protected $prettyString;

    /** @var string|null */
    protected $string;

    /** @var bool */
    protected $conjunctive;

    /** @var Bound|null */
    protected $lowerBound;

    /** @var Bound|null */
    protected $upperBound;

    /**
     * @param ConstraintInterface[] $constraints A set of constraints
     * @param bool                  $conjunctive Whether the constraints should be treated as conjunctive or disjunctive
     *
     * @throws \InvalidArgumentException If less than 2 constraints are passed
     */
    public function __construct(array $constraints, $conjunctive = true)
    {}

    /**
     * @return ConstraintInterface[]
     */
    public function getConstraints()
    {}

    /**
     * @return bool
     */
    public function isConjunctive()
    {}

    /**
     * @return bool
     */
    public function isDisjunctive()
    {}

    /**
     * {@inheritDoc}
     */
    public function compile($otherOperator)
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
     * {@inheritDoc}
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
     * Tries to optimize the constraints as much as possible, meaning
     * reducing/collapsing congruent constraints etc.
     * Does not necessarily return a MultiConstraint instance if
     * things can be reduced to a simple constraint
     *
     * @param ConstraintInterface[] $constraints A set of constraints
     * @param bool                  $conjunctive Whether the constraints should be treated as conjunctive or disjunctive
     *
     * @return ConstraintInterface
     */
    public static function create(array $constraints, $conjunctive = true)
    {}

    /**
     * @param  ConstraintInterface[] $constraints
     * @param  bool                  $conjunctive
     * @return ?array
     *
     * @phpstan-return array{0: list<ConstraintInterface>, 1: bool}|null
     */
    private static function optimizeConstraints(array $constraints, $conjunctive)
    {}

    /**
     * @return void
     */
    private function extractBounds()
    {}
}
