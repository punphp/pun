<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework\MockObject;

use function array_map;
use function implode;
use function sprintf;
use function str_starts_with;
use function strtolower;
use function substr;
use PHPUnit\Framework\SelfDescribing;
use PHPUnit\Util\Exporter;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class Invocation implements SelfDescribing
{
    /**
     * @var class-string
     */
    private string $className;

    /**
     * @var non-empty-string
     */
    private string $methodName;

    /**
     * @var array<mixed>
     */
    private array $parameters;
    private string $returnType;
    private bool $isReturnTypeNullable;
    private MockObjectInternal|StubInternal $object;

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     * @param array<mixed>     $parameters
     */
    public function __construct(string $className, string $methodName, array $parameters, string $returnType, MockObjectInternal|StubInternal $object)
    {}

    /**
     * @return class-string
     */
    public function className(): string
    {}

    /**
     * @return non-empty-string
     */
    public function methodName(): string
    {}

    /**
     * @return array<mixed>
     */
    public function parameters(): array
    {}

    /**
     * @throws Exception
     */
    public function generateReturnValue(): mixed
    {}

    public function toString(): string
    {}

    public function object(): MockObjectInternal|StubInternal
    {}
}
