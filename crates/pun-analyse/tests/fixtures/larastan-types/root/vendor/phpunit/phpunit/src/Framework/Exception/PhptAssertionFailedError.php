<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class PhptAssertionFailedError extends AssertionFailedError
{
    private readonly string $syntheticFile;
    private readonly int $syntheticLine;

    /**
     * @var list<array{file?: string, line?: int, function?: string, type?: string, ...}>
     */
    private readonly array $syntheticTrace;
    private readonly string $diff;

    /**
     * @param list<array{file?: string, line?: int, function?: string, type?: string, ...}> $trace
     */
    public function __construct(string $message, int $code, string $file, int $line, array $trace, string $diff)
    {}

    public function syntheticFile(): string
    {}

    public function syntheticLine(): int
    {}

    /**
     * @return list<array{file?: string, line?: int, function?: string, type?: string, ...}>
     */
    public function syntheticTrace(): array
    {}

    public function diff(): string
    {}
}
