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
final readonly class Trait_
{
    /**
     * @var non-empty-string
     */
    private string $name;

    /**
     * @var non-empty-string
     */
    private string $namespacedName;
    private string $namespace;

    /**
     * @var non-empty-string
     */
    private string $file;

    /**
     * @var positive-int
     */
    private int $startLine;

    /**
     * @var positive-int
     */
    private int $endLine;

    /**
     * @var list<non-empty-string>
     */
    private array $traits;

    /**
     * @var array<non-empty-string, Method>
     */
    private array $methods;

    /**
     * @param non-empty-string                $name
     * @param non-empty-string                $namespacedName
     * @param non-empty-string                $file
     * @param positive-int                    $startLine
     * @param positive-int                    $endLine
     * @param list<non-empty-string>          $traits
     * @param array<non-empty-string, Method> $methods
     */
    public function __construct(string $name, string $namespacedName, string $namespace, string $file, int $startLine, int $endLine, array $traits, array $methods)
    {}

    /**
     * @return non-empty-string
     */
    public function name(): string
    {}

    /**
     * @return non-empty-string
     */
    public function namespacedName(): string
    {}

    public function isNamespaced(): bool
    {}

    public function namespace(): string
    {}

    /**
     * @return non-empty-string
     */
    public function file(): string
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
     * @return list<non-empty-string>
     */
    public function traits(): array
    {}

    /**
     * @return array<non-empty-string, Method>
     */
    public function methods(): array
    {}
}
