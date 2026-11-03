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

use const PHP_OS;
use const PHP_OS_FAMILY;
use const PHP_VERSION;
use function addcslashes;
use function array_column;
use function array_key_exists;
use function assert;
use function class_exists;
use function extension_loaded;
use function function_exists;
use function in_array;
use function ini_get;
use function method_exists;
use function phpversion;
use function preg_match;
use function sprintf;
use function substr_count;
use PHPUnit\Event\Emitter;
use PHPUnit\Metadata\Parser\Registry;
use PHPUnit\Metadata\RequiresClass;
use PHPUnit\Metadata\RequiresEnvironmentVariable;
use PHPUnit\Metadata\RequiresFunction;
use PHPUnit\Metadata\RequiresMethod;
use PHPUnit\Metadata\RequiresOperatingSystem;
use PHPUnit\Metadata\RequiresOperatingSystemFamily;
use PHPUnit\Metadata\RequiresPhp;
use PHPUnit\Metadata\RequiresPhpExtension;
use PHPUnit\Metadata\RequiresPhpunit;
use PHPUnit\Metadata\RequiresPhpunitExtension;
use PHPUnit\Metadata\RequiresSetting;
use PHPUnit\Metadata\Version\ComparisonRequirement;
use PHPUnit\Metadata\Version\InvalidVersionRequirement;
use PHPUnit\Metadata\Version\Requirement;
use PHPUnit\Runner\Version;
use PHPUnit\TextUI\Configuration\Registry as ConfigurationRegistry;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class Requirements
{
    private Emitter $emitter;

    public function __construct(Emitter $emitter)
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     *
     * @return list<string>
     */
    public function requirementsNotSatisfiedFor(string $className, string $methodName): array
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     *
     * @return list<non-empty-string>
     */
    public function invalidVersionRequirementsFor(string $className, string $methodName): array
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     */
    public function requiresXdebug(string $className, string $methodName): bool
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     */
    private function warnAboutIncompleteVersion(Requirement $versionRequirement, string $className, string $methodName): void
    {}
}
