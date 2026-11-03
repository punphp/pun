<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Routing\Matcher\Dumper;

use Symfony\Component\ExpressionLanguage\ExpressionFunctionProviderInterface;
use Symfony\Component\ExpressionLanguage\ExpressionLanguage;
use Symfony\Component\Routing\Route;
use Symfony\Component\Routing\RouteCollection;

/**
 * CompiledUrlMatcherDumper creates PHP arrays to be used with CompiledUrlMatcher.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 * @author Tobias Schultze <http://tobion.de>
 * @author Arnaud Le Blanc <arnaud.lb@gmail.com>
 * @author Nicolas Grekas <p@tchwork.com>
 */
class CompiledUrlMatcherDumper extends MatcherDumper
{
    private ExpressionLanguage $expressionLanguage;
    private ?\Exception $signalingException = null;

    /**
     * @var ExpressionFunctionProviderInterface[]
     */
    private array $expressionLanguageProviders = [];

    public function dump(array $options = []): string
    {}

    public function addExpressionLanguageProvider(ExpressionFunctionProviderInterface $provider): void
    {}

    /**
     * Generates the arrays for CompiledUrlMatcher's constructor.
     */
    public function getCompiledRoutes(bool $forDump = false): array
    {}

    private function generateCompiledRoutes(): string
    {}

    /**
     * Splits static routes from dynamic routes, so that they can be matched first, using a simple switch.
     */
    private function groupStaticRoutes(RouteCollection $collection): array
    {}

    /**
     * Compiles static routes in a switch statement.
     *
     * Condition-less paths are put in a static array in the switch's default, with generic matching logic.
     * Paths that can match two or more routes, or have user-specified conditions are put in separate switch's cases.
     *
     * @throws \LogicException
     */
    private function compileStaticRoutes(array $staticRoutes, array &$conditions): array
    {}

    /**
     * Compiles a regular expression followed by a switch statement to match dynamic routes.
     *
     * The regular expression matches both the host and the pathinfo at the same time. For stellar performance,
     * it is built as a tree of patterns, with re-ordering logic to group same-prefix routes together when possible.
     *
     * Patterns are named so that we know which one matched (https://pcre.org/current/doc/html/pcre2syntax.html#SEC23).
     * This name is used to "switch" to the additional logic required to match the final route.
     *
     * Condition-less paths are put in a static array in the switch's default, with generic matching logic.
     * Paths that can match two or more routes, or have user-specified conditions are put in separate switch's cases.
     *
     * Last but not least:
     *  - Because it is not possible to mix unicode/non-unicode patterns in a single regexp, several of them can be generated.
     *  - The same regexp can be used several times when the logic in the switch rejects the match. When this happens, the
     *    matching-but-failing subpattern is excluded by replacing its name by "(*F)", which forces a failure-to-match.
     *    To ease this backlisting operation, the name of subpatterns is also the string offset where the replacement should occur.
     */
    private function compileDynamicRoutes(RouteCollection $collection, bool $matchHost, int $chunkLimit, array &$conditions): array
    {}

    /**
     * Compiles a regexp tree of subpatterns that matches nested same-prefix routes.
     *
     * @param \stdClass $state A simple state object that keeps track of the progress of the compilation,
     *                         and gathers the generated switch's "case" and "default" statements
     */
    private function compileStaticPrefixCollection(StaticPrefixCollection $tree, \stdClass $state, int $prefixLen, array &$conditions): string
    {}

    /**
     * Compiles a single Route to PHP code used to match it against the path info.
     */
    private function compileRoute(Route $route, string $name, string|array|null $vars, bool $hasTrailingSlash, bool $hasTrailingVar, array &$conditions): array
    {}

    private function getExpressionLanguage(): ExpressionLanguage
    {}

    private function indent(string $code, int $level = 1): string
    {}

    /**
     * @internal
     */
    public static function export(mixed $value): string
    {}
}
