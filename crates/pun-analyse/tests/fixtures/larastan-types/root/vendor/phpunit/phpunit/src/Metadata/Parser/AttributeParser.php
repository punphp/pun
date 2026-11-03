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

use const JSON_THROW_ON_ERROR;
use const PHP_EOL;
use function assert;
use function class_exists;
use function is_numeric;
use function json_decode;
use function method_exists;
use function sprintf;
use function str_starts_with;
use function strtolower;
use function trim;
use Error;
use PHPUnit\Event\Emitter;
use PHPUnit\Framework\Attributes\After;
use PHPUnit\Framework\Attributes\AfterClass;
use PHPUnit\Framework\Attributes\AllowMockObjectsWithoutExpectations;
use PHPUnit\Framework\Attributes\BackupGlobals;
use PHPUnit\Framework\Attributes\BackupStaticProperties;
use PHPUnit\Framework\Attributes\Before;
use PHPUnit\Framework\Attributes\BeforeClass;
use PHPUnit\Framework\Attributes\CoversClass;
use PHPUnit\Framework\Attributes\CoversClassesThatExtendClass;
use PHPUnit\Framework\Attributes\CoversClassesThatImplementInterface;
use PHPUnit\Framework\Attributes\CoversDirectory;
use PHPUnit\Framework\Attributes\CoversDirectoryRecursively;
use PHPUnit\Framework\Attributes\CoversFile;
use PHPUnit\Framework\Attributes\CoversFunction;
use PHPUnit\Framework\Attributes\CoversMethod;
use PHPUnit\Framework\Attributes\CoversNamespace;
use PHPUnit\Framework\Attributes\CoversNothing;
use PHPUnit\Framework\Attributes\CoversTrait;
use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\Attributes\DataProviderClosure;
use PHPUnit\Framework\Attributes\DataProviderExternal;
use PHPUnit\Framework\Attributes\Depends;
use PHPUnit\Framework\Attributes\DependsExternal;
use PHPUnit\Framework\Attributes\DependsExternalUsingDeepClone;
use PHPUnit\Framework\Attributes\DependsExternalUsingShallowClone;
use PHPUnit\Framework\Attributes\DependsOnClass;
use PHPUnit\Framework\Attributes\DependsOnClassUsingDeepClone;
use PHPUnit\Framework\Attributes\DependsOnClassUsingShallowClone;
use PHPUnit\Framework\Attributes\DependsUsingDeepClone;
use PHPUnit\Framework\Attributes\DependsUsingShallowClone;
use PHPUnit\Framework\Attributes\DisableReturnValueGenerationForTestDoubles;
use PHPUnit\Framework\Attributes\DoesNotPerformAssertions;
use PHPUnit\Framework\Attributes\ExcludeGlobalVariableFromBackup;
use PHPUnit\Framework\Attributes\ExcludeStaticPropertyFromBackup;
use PHPUnit\Framework\Attributes\Group;
use PHPUnit\Framework\Attributes\IgnoreDeprecations;
use PHPUnit\Framework\Attributes\IgnorePhpunitDeprecations;
use PHPUnit\Framework\Attributes\IgnorePhpunitWarnings;
use PHPUnit\Framework\Attributes\Large;
use PHPUnit\Framework\Attributes\Medium;
use PHPUnit\Framework\Attributes\PostCondition;
use PHPUnit\Framework\Attributes\PreCondition;
use PHPUnit\Framework\Attributes\PreserveGlobalState;
use PHPUnit\Framework\Attributes\Repeat;
use PHPUnit\Framework\Attributes\RequiresClass;
use PHPUnit\Framework\Attributes\RequiresEnvironmentVariable;
use PHPUnit\Framework\Attributes\RequiresFunction;
use PHPUnit\Framework\Attributes\RequiresMethod;
use PHPUnit\Framework\Attributes\RequiresOperatingSystem;
use PHPUnit\Framework\Attributes\RequiresOperatingSystemFamily;
use PHPUnit\Framework\Attributes\RequiresPhp;
use PHPUnit\Framework\Attributes\RequiresPhpExtension;
use PHPUnit\Framework\Attributes\RequiresPhpunit;
use PHPUnit\Framework\Attributes\RequiresPhpunitExtension;
use PHPUnit\Framework\Attributes\RequiresSetting;
use PHPUnit\Framework\Attributes\Retry;
use PHPUnit\Framework\Attributes\RunInSeparateProcess;
use PHPUnit\Framework\Attributes\RunTestsInSeparateProcesses;
use PHPUnit\Framework\Attributes\Small;
use PHPUnit\Framework\Attributes\Test;
use PHPUnit\Framework\Attributes\TestDox;
use PHPUnit\Framework\Attributes\TestDoxFormatter;
use PHPUnit\Framework\Attributes\TestDoxFormatterExternal;
use PHPUnit\Framework\Attributes\TestWith;
use PHPUnit\Framework\Attributes\TestWithJson;
use PHPUnit\Framework\Attributes\Ticket;
use PHPUnit\Framework\Attributes\UsesClass;
use PHPUnit\Framework\Attributes\UsesClassesThatExtendClass;
use PHPUnit\Framework\Attributes\UsesClassesThatImplementInterface;
use PHPUnit\Framework\Attributes\UsesDirectory;
use PHPUnit\Framework\Attributes\UsesDirectoryRecursively;
use PHPUnit\Framework\Attributes\UsesFile;
use PHPUnit\Framework\Attributes\UsesFunction;
use PHPUnit\Framework\Attributes\UsesMethod;
use PHPUnit\Framework\Attributes\UsesNamespace;
use PHPUnit\Framework\Attributes\UsesTrait;
use PHPUnit\Framework\Attributes\WithEnvironmentVariable;
use PHPUnit\Framework\Attributes\WithoutErrorHandler;
use PHPUnit\Framework\TestCase;
use PHPUnit\Metadata\Group as GroupMetadata;
use PHPUnit\Metadata\InvalidVersionRequirementException;
use PHPUnit\Metadata\Metadata;
use PHPUnit\Metadata\MetadataCollection;
use PHPUnit\Metadata\Version\InvalidVersionRequirement;
use PHPUnit\Metadata\Version\Requirement;
use PHPUnit\Runner\Filter\CompiledGroupFilter;
use ReflectionAttribute;
use ReflectionClass;
use ReflectionMethod;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class AttributeParser implements Parser
{
    /**
     * The attributes that add a test to a group and that are ignored when they
     * are declared on a parent class of a test class. This will change in
     * PHPUnit 14, see https://github.com/sebastianbergmann/phpunit/issues/6961.
     *
     * @var array<class-string, non-empty-string>
     */
    private const array GROUP_ATTRIBUTES = [
        Group::class  => '#[Group]',
        Large::class  => '#[Large]',
        Medium::class => '#[Medium]',
        Small::class  => '#[Small]',
        Ticket::class => '#[Ticket]',
    ];
    private readonly Emitter $emitter;

    /**
     * @var array<non-empty-string, true>
     */
    private array $deprecatedGroupAttributes = [];

    public function __construct(Emitter $emitter)
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

    /**
     * @param non-empty-string  $groupName
     * @param class-string      $testClassName
     * @param ?non-empty-string $testMethodName
     */
    private function isSizeGroup(string $groupName, string $testClassName, ?string $testMethodName = null): bool
    {}

    /**
     * In PHPUnit 14, the tests of a test class inherit the groups declared on
     * its parent classes, and their size from the nearest class that declares
     * one. An attribute on a parent class only triggers a deprecation when
     * this would change the groups or the size of the tests of the test class,
     * which is not the case when the test class itself declares the same
     * groups and a size.
     *
     * The deprecation keeps the test file of such a test class from being
     * skipped by the test index, which never skips a file that PHPUnit had
     * something to say about while it was loaded.
     *
     * @param ReflectionClass<object> $class
     */
    private function deprecateGroupAttributesOnParentClassesOf(ReflectionClass $class, MetadataCollection $metadata): void
    {}

    /**
     * An attribute that cannot be instantiated does not add a test to a group,
     * so it is treated as if its group was declared.
     *
     * @param ReflectionAttribute<object> $attribute
     * @param array<string, true>         $groups
     */
    private function isDeclaredGroup(ReflectionAttribute $attribute, array $groups): bool
    {}

    /**
     * Each attribute of a parent class only triggers the deprecation once, no
     * matter how many test classes extend it.
     *
     * @param non-empty-string $attribute
     * @param class-string     $className
     */
    private function deprecateGroupAttribute(string $attribute, string $className): void
    {}

    /**
     * The name of a group that the --group and --exclude-group CLI options
     * parse as a conjunction of the names of other groups cannot be used to
     * select the tests in it, see CompiledGroupFilter. The group is still
     * declared: dropping it would take the tests out of a group the test suite
     * is expected to have.
     *
     * The warning also keeps a test file that declares such a group from being
     * skipped by the test index, which never skips a file that PHPUnit had
     * something to say about while it was loaded.
     *
     * @param non-empty-string  $groupName
     * @param class-string      $testClassName
     * @param ?non-empty-string $testMethodName
     */
    private function warnAboutGroupNameThatCannotBeSelected(string $groupName, string $testClassName, ?string $testMethodName = null): void
    {}

    /**
     * @param non-empty-string  $attributeName
     * @param non-empty-string  $versionRequirement
     * @param class-string      $testClassName
     * @param ?non-empty-string $testMethodName
     */
    private function requirement(string $attributeName, string $versionRequirement, string $testClassName, ?string $testMethodName = null): ?Requirement
    {}

    /**
     * @param non-empty-string $attributeName
     * @param non-empty-string $target
     * @param non-empty-string $file
     * @param positive-int     $line
     * @param non-empty-string $message
     *
     * @return non-empty-string
     */
    private function invalidAttributeMessage(string $attributeName, string $target, string $file, int $line, string $message): string
    {}

    /**
     * @param class-string      $testClassName
     * @param ?non-empty-string $testMethodName
     *
     * @return non-empty-string
     */
    private function testAsString(string $testClassName, ?string $testMethodName = null): string
    {}
}
