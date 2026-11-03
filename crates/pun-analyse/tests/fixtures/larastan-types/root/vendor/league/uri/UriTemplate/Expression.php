<?php

/**
 * League.Uri (https://uri.thephpleague.com)
 *
 * (c) Ignace Nyamagana Butera <nyamsprod@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

declare(strict_types=1);

namespace League\Uri\UriTemplate;

use Deprecated;
use Iterator;
use IteratorAggregate;
use League\Uri\Exceptions\SyntaxError;
use Stringable;
use ValueError;

use function array_filter;
use function array_map;
use function array_reverse;
use function array_unique;
use function array_values;
use function count;
use function explode;
use function implode;
use function is_string;
use function ksort;
use function strlen;
use function strpos;
use function substr;

/**
 * @internal The class exposes the internal representation of an Expression and its usage
 * @link https://www.rfc-editor.org/rfc/rfc6570#section-2.2
 * @implements IteratorAggregate<VarSpecifier>
 */
final class Expression implements IteratorAggregate
{
    /** @var array<VarSpecifier> */
    private readonly array $varSpecifiers;
    /** @var array<string> */
    public readonly array $variableNames;
    public readonly string $value;

    private function __construct(public readonly Operator $operator, VarSpecifier ...$varSpecifiers)
    {}

    /**
     * @throws SyntaxError if the expression is invalid
     */
    public static function new(Stringable|string $expression): self
    {}

    /**
     * @return Iterator<VarSpecifier>
     */
    public function getIterator(): Iterator
    { yield; }

    public function expand(VariableBag $variables): string
    {}

    /**
     * Extracts variables from a value according to this expression.
     *
     * Named expressions match variables by name, query expressions match query
     * parameters by name regardless of their order, while positional expressions
     * assign values according to the order of their variable specifiers.
     *
     * @throws VariableCanNotBeExtracted If the value cannot be extracted according
     *                                   to the expression.
     */
    public function extract(string $value): ExtractionResult
    {}

    /**
     * Extracts variables from a named expression value.
     *
     * Named variables are matched by name rather than by position. Non-exploded
     * variables may consume at most one component, while an exploded variable
     * may consume multiple components.
     *
     * @throws VariableCanNotBeExtracted If the value cannot be matched to the
     *                                   variable specifiers.
     */
    private function extractNamedValues(string $value): ExtractionResult
    {}

    /**
     * Matches named components against the variable specifiers.
     *
     * Components are matched by variable name rather than by position.
     * Non-exploded variables may occur at most once, while exploded variables
     * may consume multiple components.
     *
     * @param list<string> $components
     *
     * @return list<string|null>|null
     */
    private function matchNamedValues(array $components): ?array
    {}

    /**
     * Extracts variables from a positional expression value.
     *
     * Values are assigned to variable specifiers in their declared order.
     * Exploded variables consume the remaining values required to leave enough
     * values for the following variable specifiers.
     *
     * @throws VariableCanNotBeExtracted If a value cannot be extracted or a value
     *                                   exceeds its variable's prefix length.
     */
    private function extractPositionalValues(string $value): ExtractionResult
    {}

    private function extractPositionalCandidates(
        string $value,
        int $varSpecifierOffset,
        int $valueOffset,
        ExtractionResult $previousResult,
    ): ExtractionResult {}

    /**
     * Finds all positions of a delimiter at or after the given offset.
     *
     * @param int $offset The position from which to search.
     *
     *
     * @throws ValueError If the delimiter is empty.
     * @return list<int> The positions at which the delimiter occurs.
     */
    private function delimiterPositions(string $value, int $offset, string $delimiter): array
    {}

    /**
     * Ensures that an extracted value does not exceed its variable's prefix length.
     *
     * @throws VariableCanNotBeExtracted If the extracted value exceeds the
     *                                   variable's prefix length.
     */
    private function assertPrefixLength(VarSpecifier $varSpecifier, ExtractionResult $variables): void
    {}

    /**
     * DEPRECATION WARNING! This method will be removed in the next major point release.
     *
     * @throws SyntaxError if the expression is invalid
     * @see Expression::new()
     *
     * @deprecated Since version 7.0.0
     * @codeCoverageIgnore
     */
    #[Deprecated(message:'use League\Uri\UriTemplate\Exppression::new() instead', since:'league/uri:7.0.0')]
    public static function createFromString(Stringable|string $expression): self
    {}
}
