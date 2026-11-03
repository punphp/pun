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

use function assert;
use PHPUnit\Framework\TestCase;
use PHPUnit\Metadata\CoversClass;
use PHPUnit\Metadata\CoversClassesThatExtendClass;
use PHPUnit\Metadata\CoversClassesThatImplementInterface;
use PHPUnit\Metadata\CoversDirectory;
use PHPUnit\Metadata\CoversDirectoryRecursively;
use PHPUnit\Metadata\CoversFile;
use PHPUnit\Metadata\CoversFunction;
use PHPUnit\Metadata\CoversMethod;
use PHPUnit\Metadata\CoversNamespace;
use PHPUnit\Metadata\CoversTrait;
use PHPUnit\Metadata\MetadataCollection;
use PHPUnit\Metadata\Parser\Registry;
use PHPUnit\Metadata\UsesClass;
use PHPUnit\Metadata\UsesClassesThatExtendClass;
use PHPUnit\Metadata\UsesClassesThatImplementInterface;
use PHPUnit\Metadata\UsesDirectory;
use PHPUnit\Metadata\UsesDirectoryRecursively;
use PHPUnit\Metadata\UsesFile;
use PHPUnit\Metadata\UsesFunction;
use PHPUnit\Metadata\UsesMethod;
use PHPUnit\Metadata\UsesNamespace;
use PHPUnit\Metadata\UsesTrait;
use SebastianBergmann\CodeCoverage\Test\Target\Target;
use SebastianBergmann\CodeCoverage\Test\Target\TargetCollection;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class CodeCoverage
{
    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     */
    public function coversTargets(string $className, string $methodName): TargetCollection
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     */
    public function usesTargets(string $className, string $methodName): TargetCollection
    {}

    public function shouldCodeCoverageBeCollectedFor(TestCase $test): bool
    {}

    /**
     * @param class-string $className
     */
    public function coversNothingContradictsCoversOrUses(string $className): bool
    {}

    private function coversTargetsFor(MetadataCollection $metadataCollection): TargetCollection
    {}

    private function usesTargetsFor(MetadataCollection $metadataCollection): TargetCollection
    {}
}
