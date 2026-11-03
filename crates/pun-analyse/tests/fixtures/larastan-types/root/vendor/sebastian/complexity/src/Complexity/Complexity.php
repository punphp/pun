<?php declare(strict_types=1);
/*
 * This file is part of sebastian/complexity.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\Complexity;

use function str_contains;

/**
 * @immutable
 */
final readonly class Complexity
{
    /**
     * @var non-empty-string
     */
    private string $name;

    /**
     * @var positive-int
     */
    private int $cyclomaticComplexity;

    /**
     * @param non-empty-string $name
     * @param positive-int     $cyclomaticComplexity
     */
    public function __construct(string $name, int $cyclomaticComplexity)
    {}

    /**
     * @return non-empty-string
     */
    public function name(): string
    {}

    /**
     * @return positive-int
     */
    public function cyclomaticComplexity(): int
    {}

    public function isFunction(): bool
    {}

    public function isMethod(): bool
    {}
}
