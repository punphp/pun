<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Metadata\Api;

use function array_key_exists;
use function assert;
use function count;
use function get_debug_type;
use function is_array;
use function is_int;
use function is_iterable;
use function is_string;
use function sprintf;
use PHPUnit\Event;
use PHPUnit\Event\Code\TestMethod;
use PHPUnit\Framework\InvalidDataProviderException;
use PHPUnit\Framework\TestCase;
use PHPUnit\Metadata\DataProvider as DataProviderMetadata;
use PHPUnit\Metadata\DataProviderClosure as DataProviderClosureMetadata;
use PHPUnit\Metadata\MetadataCollection;
use PHPUnit\Metadata\Parser\Registry as MetadataRegistry;
use PHPUnit\Metadata\TestWith;
use PHPUnit\Util\Test;
use ReflectionMethod;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class DataProvider
{
    private Event\Emitter $emitter;

    public function __construct(Event\Emitter $emitter)
    {}

    /**
     * @param class-string<TestCase> $className
     * @param non-empty-string       $methodName
     *
     * @throws InvalidDataProviderException
     *
     * @return ?array<ProvidedData>
     */
    public function providedData(string $className, string $methodName): ?array
    {}

    /**
     * @param class-string<TestCase> $testClassName
     *
     * @throws InvalidDataProviderException
     *
     * @return array<ProvidedData>
     */
    private function dataProvidedByMethods(string $testClassName, ReflectionMethod $testMethod, MetadataCollection $dataProvider, MetadataCollection $dataProviderClosure): array
    {}

    /**
     * @return array<ProvidedData>
     */
    private function dataProvidedByMetadata(ReflectionMethod $testMethod, MetadataCollection $testWith): array
    {}

    /**
     * @param int|non-empty-string $key
     *
     * @return non-empty-string
     */
    private function formatKey(int|string $key): string
    {}

    private function triggerWarningForMixingOfDataProviderAndTestWith(ReflectionMethod $method): void
    {}

    private function triggerWarningForArgumentCount(ReflectionMethod $method, string $key, string $label, int $numberOfValues, int $testMethodNumberOfParameters): void
    {}

    private function testValueObject(ReflectionMethod $method): TestMethod
    {}
}
