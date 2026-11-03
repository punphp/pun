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
 * Defines the absence of a constraint.
 *
 * This constraint matches everything.
 */
class MatchAllConstraint implements ConstraintInterface
{
    /** @var string|null */
    protected $prettyString;

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
    public function compile($otherOperator)
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
    public function getUpperBound()
    {}

    /**
     * {@inheritDoc}
     */
    public function getLowerBound()
    {}
}
