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

use Composer\Semver\Constraint\ConstraintInterface;
use Composer\Semver\Constraint\MatchAllConstraint;
use Composer\Semver\Constraint\MultiConstraint;
use Composer\Semver\Constraint\Constraint;

/**
 * Version parser.
 *
 * @author Jordi Boggiano <j.boggiano@seld.be>
 */
class VersionParser
{
    /**
     * Regex to match pre-release data (sort of).
     *
     * Due to backwards compatibility:
     *   - Instead of enforcing hyphen, an underscore, dot or nothing at all are also accepted.
     *   - Only stabilities as recognized by Composer are allowed to precede a numerical identifier.
     *   - Numerical-only pre-release identifiers are not supported, see tests.
     *
     *                        |--------------|
     * [major].[minor].[patch] -[pre-release] +[build-metadata]
     *
     * @var string
     */
    private static $modifierRegex = '[._-]?(?:(stable|beta|b|RC|alpha|a|patch|pl|p)((?:[.-]?\d+)*+)?)?([.-]?dev)?';

    /** @var string */
    private static $stabilitiesRegex = 'stable|RC|beta|alpha|dev';

    /**
     * Returns the stability of a version.
     *
     * @param string $version
     *
     * @return string
     * @phpstan-return 'stable'|'RC'|'beta'|'alpha'|'dev'
     */
    public static function parseStability($version)
    {}

    /**
     * @param string $stability
     *
     * @return string
     * @phpstan-return 'stable'|'RC'|'beta'|'alpha'|'dev'
     */
    public static function normalizeStability($stability)
    {}

    /**
     * @param string $version
     *
     * @return bool
     */
    public function isValid($version)
    {}

    /**
     * Normalizes a version string to be able to perform comparisons on it.
     *
     * @param string $version
     * @param ?string $fullVersion optional complete version string to give more context
     *
     * @throws \UnexpectedValueException
     *
     * @return string
     */
    public function normalize($version, $fullVersion = null)
    {}

    /**
     * Extract numeric prefix from alias, if it is in numeric format, suitable for version comparison.
     *
     * @param string $branch Branch name (e.g. 2.1.x-dev)
     *
     * @return string|false Numeric prefix if present (e.g. 2.1.) or false
     */
    public function parseNumericAliasPrefix($branch)
    {}

    /**
     * Normalizes a branch name to be able to perform comparisons on it.
     *
     * @param string $name
     *
     * @return string
     */
    public function normalizeBranch($name)
    {}

    /**
     * Normalizes a default branch name (i.e. master on git) to 9999999-dev.
     *
     * @param string $name
     *
     * @return string
     *
     * @deprecated No need to use this anymore in theory, Composer 2 does not normalize any branch names to 9999999-dev anymore
     */
    public function normalizeDefaultBranch($name)
    {}

    /**
     * Parses a constraint string into MultiConstraint and/or Constraint objects.
     *
     * @param string $constraints
     *
     * @return ConstraintInterface
     */
    public function parseConstraints($constraints)
    {}

    /**
     * @param string $constraint
     *
     * @throws \UnexpectedValueException
     *
     * @return array
     *
     * @phpstan-return non-empty-array<ConstraintInterface>
     */
    private function parseConstraint($constraint)
    {}

    /**
     * Increment, decrement, or simply pad a version number.
     *
     * Support function for {@link parseConstraint()}
     *
     * @param array  $matches   Array with version parts in array indexes 1,2,3,4
     * @param int    $position  1,2,3,4 - which segment of the version to increment/decrement
     * @param int    $increment
     * @param string $pad       The string to pad version parts after $position
     *
     * @return string|null The new version
     *
     * @phpstan-param string[] $matches
     */
    private function manipulateVersionString(array $matches, $position, $increment = 0, $pad = '0')
    {}

    /**
     * Expand shorthand stability string to long version.
     *
     * @param string $stability
     *
     * @return string
     */
    private function expandStability($stability)
    {}
}
