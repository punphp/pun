<?php

/*
 * This file is part of composer/semver.
 *
 * (c) Composer <https://github.com/composer>
 *
 * For the full copyright and license information, please view
 * the LICENSE file that was distributed with this source code.
 */

namespace Composer\Semver;

use Composer\Semver\Constraint\Constraint;

class Semver
{
    const SORT_ASC = 1;
    const SORT_DESC = -1;

    /** @var VersionParser */
    private static $versionParser;

    /**
     * Determine if given version satisfies given constraints.
     *
     * @param string $version
     * @param string $constraints
     *
     * @return bool
     */
    public static function satisfies($version, $constraints)
    {}

    /**
     * Return all versions that satisfy given constraints.
     *
     * @param string[] $versions
     * @param string   $constraints
     *
     * @return list<string>
     */
    public static function satisfiedBy(array $versions, $constraints)
    {}

    /**
     * Sort given array of versions.
     *
     * @param string[] $versions
     *
     * @return list<string>
     */
    public static function sort(array $versions)
    {}

    /**
     * Sort given array of versions in reverse.
     *
     * @param string[] $versions
     *
     * @return list<string>
     */
    public static function rsort(array $versions)
    {}

    /**
     * @param string[] $versions
     * @param int      $direction
     *
     * @return list<string>
     */
    private static function usort(array $versions, $direction)
    {}
}
