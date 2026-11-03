<?php declare(strict_types=1);
/*
 * This file is part of sebastian/cli-parser.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CliParser;

use function array_map;
use function array_merge;
use function array_shift;
use function array_slice;
use function assert;
use function count;
use function current;
use function explode;
use function is_array;
use function is_int;
use function key;
use function levenshtein;
use function next;
use function preg_replace;
use function reset;
use function rtrim;
use function sort;
use function str_ends_with;
use function str_starts_with;
use function strlen;
use function strstr;
use function substr;
use function usort;

final readonly class Parser
{
    /**
     * @param list<string> $argv
     * @param list<string> $longOptions
     *
     * @throws AmbiguousOptionException
     * @throws OptionDoesNotAllowArgumentException
     * @throws RequiredOptionArgumentMissingException
     * @throws UnknownOptionException
     *
     * @return array{0: list<array{0: string, 1: ?string}>, 1: list<string>}
     */
    public function parse(array $argv, string $shortOptions, ?array $longOptions = null): array
    {}

    /**
     * @param list<array{0: string, 1: ?string}> $options
     * @param list<string>                       $argv
     *
     * @throws RequiredOptionArgumentMissingException
     */
    private function parseShortOption(string $argument, string $shortOptions, array &$options, array &$argv): void
    {}

    /**
     * @param list<string>                       $longOptions
     * @param list<array{0: string, 1: ?string}> $options
     * @param list<string>                       $argv
     *
     * @throws AmbiguousOptionException
     * @throws OptionDoesNotAllowArgumentException
     * @throws RequiredOptionArgumentMissingException
     * @throws UnknownOptionException
     */
    private function parseLongOption(string $argument, array $longOptions, array &$options, array &$argv): void
    {}

    /**
     * An option that is spelled out in full is never ambiguous, no matter how many
     * other options begin with it: "--html" unambiguously means "html" even when
     * "html-views" is also declared.
     *
     * @param list<string> $longOptions
     */
    private function exactMatch(string $option, array $longOptions): ?string
    {}

    /**
     * @param list<array{int, string}> $similarOptions
     *
     * @return array<string>
     */
    private function formatSimilarOptions(array $similarOptions): array
    {}
}
