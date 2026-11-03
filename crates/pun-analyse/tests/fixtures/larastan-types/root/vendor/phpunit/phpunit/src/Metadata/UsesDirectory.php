<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Metadata;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final readonly class UsesDirectory extends Metadata
{
    /**
     * @var non-empty-string
     */
    private string $directory;

    /**
     * @param non-empty-string $directory
     */
    protected function __construct(Level $level, string $directory)
    {}

    public function isUsesDirectory(): true
    {}

    /**
     * @return non-empty-string
     */
    public function directory(): string
    {}
}
