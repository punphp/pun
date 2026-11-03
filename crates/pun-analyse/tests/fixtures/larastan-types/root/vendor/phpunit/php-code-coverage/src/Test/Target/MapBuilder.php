<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Test\Target;

use function array_keys;
use function array_merge;
use function array_slice;
use function array_unique;
use function array_values;
use function assert;
use function count;
use function dirname;
use function explode;
use function implode;
use function range;
use SebastianBergmann\CodeCoverage\Filter;
use SebastianBergmann\CodeCoverage\StaticAnalysis\Class_;
use SebastianBergmann\CodeCoverage\StaticAnalysis\FileAnalyser;
use SebastianBergmann\CodeCoverage\StaticAnalysis\Trait_;

/**
 * @phpstan-import-type TargetMap from Mapper
 * @phpstan-import-type TargetMapPart from Mapper
 * @phpstan-import-type ReverseLookup from Mapper
 *
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final readonly class MapBuilder
{
    /**
     * @return TargetMap
     */
    public function build(Filter $filter, FileAnalyser $analyser): array
    {}

    /**
     * @param non-empty-string                            $targetClass
     * @param array<non-empty-string, list<positive-int>> $sourceData
     * @param TargetMapPart                               $data
     *
     * @param-out TargetMapPart $data
     */
    private function mergeLines(string $targetClass, array $sourceData, array &$data): void
    {}

    /**
     * @param non-empty-string $file
     * @param TargetMapPart    $methods
     * @param ReverseLookup    $reverseLookup
     *
     * @param-out TargetMapPart $methods
     * @param-out ReverseLookup $reverseLookup
     */
    private function processMethods(Class_|Trait_ $classOrTrait, string $file, array &$methods, array &$reverseLookup): void
    {}

    /**
     * @param TargetMapPart    $data
     * @param non-empty-string $namespace
     * @param non-empty-string $file
     * @param positive-int     $startLine
     * @param positive-int     $endLine
     *
     * @param-out TargetMapPart $data
     */
    private function processNamespace(array &$data, string $namespace, string $file, int $startLine, int $endLine): void
    {}

    /**
     * @param TargetMapPart    $data
     * @param non-empty-string $unit
     * @param non-empty-string $file
     * @param positive-int     $startLine
     * @param positive-int     $endLine
     *
     * @param-out TargetMapPart $data
     */
    private function process(array &$data, string $unit, string $file, int $startLine, int $endLine): void
    {}

    /**
     * @param array<non-empty-string, Class_> $classDetails
     *
     * @return array<Class_>
     */
    private function parentClasses(array $classDetails, Class_ $class): array
    {}
}
