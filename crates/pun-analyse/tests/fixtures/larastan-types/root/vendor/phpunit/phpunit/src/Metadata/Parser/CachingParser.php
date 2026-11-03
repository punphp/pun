<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Metadata\Parser;

use function assert;
use function class_exists;
use function method_exists;
use PHPUnit\Metadata\MetadataCollection;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class CachingParser implements Parser
{
    private readonly Parser $reader;

    /**
     * @var array<class-string, MetadataCollection>
     */
    private array $classCache = [];

    /**
     * @var array<non-empty-string, MetadataCollection>
     */
    private array $methodCache = [];

    /**
     * @var array<non-empty-string, MetadataCollection>
     */
    private array $classAndMethodCache = [];

    public function __construct(Parser $reader)
    {}

    /**
     * @param class-string $className
     */
    public function forClass(string $className): MetadataCollection
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     */
    public function forMethod(string $className, string $methodName): MetadataCollection
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     */
    public function forClassAndMethod(string $className, string $methodName): MetadataCollection
    {}
}
