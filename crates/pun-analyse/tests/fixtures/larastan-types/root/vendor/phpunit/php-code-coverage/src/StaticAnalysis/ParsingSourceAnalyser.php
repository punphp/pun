<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\StaticAnalysis;

use const T_COMMENT;
use const T_DOC_COMMENT;
use function array_intersect_key;
use function array_replace;
use function assert;
use function is_array;
use function ksort;
use function max;
use function substr_count;
use function token_get_all;
use function trim;
use PhpParser\Error;
use PhpParser\NodeTraverser;
use PhpParser\NodeVisitor\NameResolver;
use PhpParser\Parser;
use PhpParser\ParserFactory;
use SebastianBergmann\LinesOfCode\LineCountingVisitor;

/**
 * @internal This interface is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final readonly class ParsingSourceAnalyser implements SourceAnalyser
{
    private Parser $parser;

    public function __construct()
    {}

    /**
     * @param non-empty-string $sourceCodeFile
     */
    public function analyse(string $sourceCodeFile, string $sourceCode, bool $useAnnotationsForIgnoringCode, bool $ignoreDeprecatedCode): AnalysisResult
    {}

    /**
     * A file that cannot be parsed gets a degraded analysis result: no code
     * units, no executable lines, and no dead lines. Lines ignored using
     * line-based annotations are still found, because that scan is based on
     * token_get_all() and does not require the parser.
     *
     * @param positive-int $linesOfCode
     */
    private function resultForFileThatCannotBeParsed(string $sourceCode, bool $useAnnotationsForIgnoringCode, int $linesOfCode, Error $error): AnalysisResult
    {}

    /**
     * @return array<int, true>
     */
    private function findLinesIgnoredByLineBasedAnnotations(string $source, bool $useAnnotationsForIgnoringCode): array
    {}
}
