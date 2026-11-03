<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Routing;

/**
 * RouteCompiler compiles Route instances to CompiledRoute instances.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 * @author Tobias Schultze <http://tobion.de>
 */
class RouteCompiler implements RouteCompilerInterface
{
    /**
     * This string defines the characters that are automatically considered separators in front of
     * optional placeholders (with default and no static text following). Such a single separator
     * can be left out together with the optional placeholder from matching and generating URLs.
     */
    public const SEPARATORS = '/,;.:-_~+*=@|';

    /**
     * The maximum supported length of a PCRE subpattern name
     * http://pcre.org/current/doc/html/pcre2pattern.html#SEC16.
     *
     * @internal
     */
    public const VARIABLE_MAXIMUM_LENGTH = 32;

    /**
     * @throws \InvalidArgumentException if a path variable is named _fragment
     * @throws \LogicException           if a variable is referenced more than once
     * @throws \DomainException          if a variable name starts with a digit or if it is too long to be successfully used as
     *                                   a PCRE subpattern
     */
    public static function compile(Route $route): CompiledRoute
    {}

    private static function compilePattern(Route $route, string $pattern, bool $isHost): array
    {}

    /**
     * Determines the longest static prefix possible for a route.
     */
    private static function determineStaticPrefix(Route $route, array $tokens): string
    {}

    /**
     * Returns the next static character in the Route pattern that will serve as a separator (or the empty string when none available).
     */
    private static function findNextSeparator(string $pattern, bool $useUtf8): string
    {}

    /**
     * Computes the regexp used to match a specific token. It can be static text or a subpattern.
     *
     * @param array $tokens        The route tokens
     * @param int   $index         The index of the current token
     * @param int   $firstOptional The index of the first optional token
     */
    private static function computeRegexp(array $tokens, int $index, int $firstOptional): string
    {}

    private static function transformCapturingGroupsToNonCapturings(string $regexp): string
    {}
}
