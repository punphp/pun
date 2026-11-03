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

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final readonly class Method
{
    /**
     * @var non-empty-string
     */
    private string $name;

    /**
     * @var positive-int
     */
    private int $startLine;

    /**
     * @var positive-int
     */
    private int $endLine;
    private Visibility $visibility;

    /**
     * @var non-empty-string
     */
    private string $signature;

    /**
     * @var non-negative-int
     */
    private int $cyclomaticComplexity;

    /**
     * @param non-empty-string $name
     * @param positive-int     $startLine
     * @param positive-int     $endLine
     * @param non-empty-string $signature
     * @param non-negative-int $cyclomaticComplexity
     */
    public function __construct(string $name, int $startLine, int $endLine, string $signature, Visibility $visibility, int $cyclomaticComplexity)
    {}

    /**
     * @return non-empty-string
     */
    public function name(): string
    {}

    /**
     * @return positive-int
     */
    public function startLine(): int
    {}

    /**
     * @return positive-int
     */
    public function endLine(): int
    {}

    /**
     * @return non-empty-string
     */
    public function signature(): string
    {}

    public function visibility(): Visibility
    {}

    /**
     * @return non-negative-int
     */
    public function cyclomaticComplexity(): int
    {}
}
