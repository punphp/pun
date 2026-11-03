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
 * Glob matches globbing patterns against text.
 *
 *     if match_glob("foo.*", "foo.bar") echo "matched\n";
 *
 *     // prints foo.bar and foo.baz
 *     $regex = glob_to_regex("foo.*");
 *     for (['foo.bar', 'foo.baz', 'foo', 'bar'] as $t)
 *     {
 *         if (/$regex/) echo "matched: $car\n";
 *     }
 *
 * Glob implements glob(3) style matching that can be used to match
 * against text, rather than fetching names from a filesystem.
 *
 * Based on the Perl Text::Glob module.
 *
 * @author Fabien Potencier <fabien@symfony.com> PHP port
 * @author     Richard Clamp <richardc@unixbeard.net> Perl version
 * @copyright  2004-2005 Fabien Potencier <fabien@symfony.com>
 * @copyright  2002 Richard Clamp <richardc@unixbeard.net>
 */
class Glob
{
    /**
     * Returns a regexp which is the equivalent of the glob pattern.
     */
    public static function toRegex(string $glob, bool $strictLeadingDot = true, bool $strictWildcardSlash = true, string $delimiter = '#'): string
    {}

    /**
     * Returns null when a "/" comes before the closing "]", and "\[" when no "]" closes the "[" at position $i.
     *
     * $scanEnd is where the last scan stopped without finding a "]": every "[" before that position gets the same result.
     */
    private static function bracketExpressionToRegex(string $glob, int &$i, string $delimiter, int &$scanEnd): ?string
    {}
}
