<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework\TestCase;

use function is_int;
use function sprintf;
use PHPUnit\Util\Exporter;
use PHPUnit\Util\Sanitizer;

/**
 * The data set that a data provider provided for a test method.
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @immutable
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class DataSet
{
    private int|string $name;

    /**
     * @var array<mixed>
     */
    private array $data;

    public static function empty(): self
    {}

    /**
     * @param array<mixed> $data
     */
    public function __construct(int|string $name, array $data)
    {}

    public function isEmpty(): bool
    {}

    public function name(): int|string
    {}

    /**
     * @return array<mixed>
     */
    public function data(): array
    {}

    public function asString(): string
    {}

    public function asStringWithData(): string
    {}
}
