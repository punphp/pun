<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Finder;

/**
 * Gitignore matches against text.
 *
 * @author Michael Voříšek <vorismi3@fel.cvut.cz>
 * @author Ahmed Abdou <mail@ahmd.io>
 */
class Gitignore
{
    /**
     * Returns a regexp which is the equivalent of the gitignore pattern.
     *
     * Format specification: https://git-scm.com/docs/gitignore#_pattern_format
     */
    public static function toRegex(string $gitignoreFileContent): string
    {}

    public static function toRegexMatchingNegatedPatterns(string $gitignoreFileContent): string
    {}

    private static function buildRegex(string $gitignoreFileContent, bool $inverted): string
    {}

    /**
     * Consecutive patterns share a single group, so deep nesting stays proportional to the number of negative patterns.
     *
     * @param list<string> $alternatives
     */
    private static function alternate(string $res, array &$alternatives): string
    {}

    private static function lineToRegex(string $gitignoreLine): string
    {}

    /**
     * Unescapes the "-" of each range. A range that ends before its start only matches its start, as in git.
     */
    private static function rangesToRegex(string $class): string
    {}
}
