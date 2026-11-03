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

use function file_get_contents;
use SebastianBergmann\CodeCoverage\Serialization\FileCouldNotBeReadException;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final class FileAnalyser
{
    private readonly SourceAnalyser $sourceAnalyser;
    private readonly bool $useAnnotationsForIgnoringCode;
    private readonly bool $ignoreDeprecatedCode;

    /**
     * @var array<non-empty-string, AnalysisResult>
     */
    private array $cache = [];

    /**
     * @var array<non-empty-string, non-empty-string>
     */
    private array $parseErrors = [];

    public function __construct(SourceAnalyser $sourceAnalyser, bool $useAnnotationsForIgnoringCode, bool $ignoreDeprecatedCode)
    {}

    /**
     * @param non-empty-string $sourceCodeFile
     *
     * @throws FileCouldNotBeReadException
     */
    public function analyse(string $sourceCodeFile): AnalysisResult
    {}

    /**
     * Returns the files that could not be parsed for static analysis,
     * mapped to the parser's error message.
     *
     * @return array<non-empty-string, non-empty-string>
     */
    public function parseErrors(): array
    {}
}
