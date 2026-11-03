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

class Bound
{
    /**
     * @var string
     */
    private $version;

    /**
     * @var bool
     */
    private $isInclusive;

    /**
     * @param string $version
     * @param bool   $isInclusive
     */
    public function __construct($version, $isInclusive)
    {}

    /**
     * @return string
     */
    public function getVersion()
    {}

    /**
     * @return bool
     */
    public function isInclusive()
    {}

    /**
     * @return bool
     */
    public function isZero()
    {}

    /**
     * @return bool
     */
    public function isPositiveInfinity()
    {}

    /**
     * Compares a bound to another with a given operator.
     *
     * @param Bound  $other
     * @param string $operator
     *
     * @return bool
     */
    public function compareTo(Bound $other, $operator)
    {}

    public function __toString()
    {}

    /**
     * @return self
     */
    public static function zero()
    {}

    /**
     * @return self
     */
    public static function positiveInfinity()
    {}
}
