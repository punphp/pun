<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner\TestIndex;

use PHPUnit\Framework\TestCase;
use PHPUnit\Metadata\Api\Groups;
use PHPUnit\Metadata\Parser\Registry as MetadataRegistry;
use PHPUnit\Util\Reflection;
use PHPUnit\Util\Test as TestUtil;
use ReflectionClass;

/**
 * What is known about the tests in a single test file without loading it.
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class TestIndexEntry
{
    /**
     * @var class-string<TestCase>
     */
    private string $className;

    /**
     * @var array<non-empty-string, list<non-empty-string>>
     */
    private array $groups;

    /**
     * @var array<non-empty-string, bool>
     */
    private array $dataSets;
    private bool $madePhpUnitWarn;

    /**
     * @var non-empty-array<non-empty-string, non-empty-string>
     */
    private array $dependencies;

    /**
     * Returns null when the source files the entry would be derived from cannot
     * be read, in which case the file must not be skipped on a later run.
     *
     * @param ReflectionClass<TestCase> $class
     */
    public static function for(ReflectionClass $class, FileHasher $hasher, bool $madePhpUnitWarn): ?self
    {}

    /**
     * @param class-string<TestCase>                              $className
     * @param array<non-empty-string, list<non-empty-string>>     $groups
     * @param array<non-empty-string, bool>                       $dataSets
     * @param non-empty-array<non-empty-string, non-empty-string> $dependencies
     */
    public static function from(string $className, array $groups, array $dataSets, bool $madePhpUnitWarn, array $dependencies): self
    {}

    /**
     * @param class-string<TestCase>                              $className
     * @param array<non-empty-string, list<non-empty-string>>     $groups
     * @param array<non-empty-string, bool>                       $dataSets
     * @param non-empty-array<non-empty-string, non-empty-string> $dependencies
     */
    private function __construct(string $className, array $groups, array $dataSets, bool $madePhpUnitWarn, array $dependencies)
    {}

    /**
     * @return class-string<TestCase>
     */
    public function className(): string
    {}

    /**
     * The groups of every test method in the file, keyed by method name. These
     * are the groups that TestSuite::addTestMethod() merges into the test
     * suite, including the virtual groups that back --covers, --uses and
     * --requires-php-extension.
     *
     * @return array<non-empty-string, list<non-empty-string>>
     */
    public function groups(): array
    {}

    /**
     * Whether a test method has data sets, keyed by method name.
     *
     * The name of a test that has no data set is the name of its method, which
     * makes it possible to decide whether a filter for the name of a test can
     * select it. The name of a data set is only known once the data provider
     * has been invoked, so a method that has data sets cannot be decided.
     *
     * @return array<non-empty-string, bool>
     */
    public function dataSets(): array
    {}

    /**
     * Whether PHPUnit had something to say about this file when it was loaded.
     *
     * Such a file is never skipped: whether PHPUnit says it would otherwise
     * depend on the state of the index, and the same command has to produce the
     * same output twice.
     */
    public function madePhpUnitWarn(): bool
    {}

    /**
     * The source files this entry was derived from, and their hashes.
     *
     * @return non-empty-array<non-empty-string, non-empty-string>
     */
    public function dependencies(): array
    {}

    /**
     * An entry is only valid while every source file it was derived from still
     * has the contents it had when the entry was recorded.
     */
    public function isValid(FileHasher $hasher): bool
    {}
}
