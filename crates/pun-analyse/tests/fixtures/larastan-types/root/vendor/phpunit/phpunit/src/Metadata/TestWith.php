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
final readonly class TestWith extends Metadata
{
    private mixed $data;

    /**
     * @var ?non-empty-string
     */
    private ?string $name;

    /**
     * @param ?non-empty-string $name
     */
    protected function __construct(Level $level, mixed $data, ?string $name = null)
    {}

    public function isTestWith(): true
    {}

    public function data(): mixed
    {}

    /**
     * @phpstan-assert-if-true !null $this->name
     */
    public function hasName(): bool
    {}

    /**
     * @return ?non-empty-string
     */
    public function name(): ?string
    {}
}
