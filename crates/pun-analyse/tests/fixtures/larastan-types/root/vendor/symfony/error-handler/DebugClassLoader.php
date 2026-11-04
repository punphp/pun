<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\ErrorHandler;

use Composer\InstalledVersions;
use Doctrine\Common\Persistence\Proxy as LegacyProxy;
use Doctrine\Persistence\Proxy;
use Mockery\MockInterface;
use Phake\IMock;
use PHPUnit\Framework\MockObject\Matcher\StatelessInvocation;
use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\Stub;
use Prophecy\Prophecy\ProphecySubjectInterface;
use ProxyManager\Proxy\ProxyInterface;
use Psr\Log\LogLevel;
use Symfony\Component\DependencyInjection\Argument\LazyClosure;
use Symfony\Component\ErrorHandler\Internal\TentativeTypes;
use Symfony\Component\VarExporter\LazyObjectInterface;

/**
 * Autoloader checking if the class is really defined in the file found.
 *
 * The ClassLoader will wrap all registered autoloaders
 * and will throw an exception if a file is found but does
 * not declare the class.
 *
 * It can also patch classes to turn docblocks into actual return types.
 * This behavior is controlled by the SYMFONY_PATCH_TYPE_DECLARATIONS env var,
 * which is a url-encoded array with the follow parameters:
 *  - "force": any value enables deprecation notices - can be any of:
 *      - "phpdoc" to patch only docblock annotations
 *      - "2" to add all possible return types
 *      - "1" to add return types but only to tests/final/internal/private methods
 *  - "php": the target version of PHP - e.g. "7.1" doesn't generate "object" types
 *  - "deprecations": "1" to trigger a deprecation notice when a child class misses a
 *                    return type while the parent declares an "@return" annotation
 *
 * Note that patching doesn't care about any coding style so you'd better to run
 * php-cs-fixer after, with rules "phpdoc_trim_consecutive_blank_line_separation"
 * and "no_superfluous_phpdoc_tags" enabled typically.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 * @author Christophe Coevoet <stof@notk.org>
 * @author Nicolas Grekas <p@tchwork.com>
 * @author Guilhem Niot <guilhem.niot@gmail.com>
 */
class DebugClassLoader
{
    private const SPECIAL_RETURN_TYPES = [
        'void' => 'void',
        'null' => 'null',
        'resource' => 'resource',
        'boolean' => 'bool',
        'true' => 'true',
        'false' => 'false',
        'integer' => 'int',
        'array' => 'array',
        'bool' => 'bool',
        'callable' => 'callable',
        'float' => 'float',
        'int' => 'int',
        'iterable' => 'iterable',
        'object' => 'object',
        'string' => 'string',
        'non-empty-string' => 'string',
        'self' => 'self',
        'parent' => 'parent',
        'mixed' => 'mixed',
        'static' => 'static',
        '$this' => 'static',
        'list' => 'array',
        'non-empty-list' => 'array',
        'class-string' => 'string',
        'never' => 'never',
    ];

    private const BUILTIN_RETURN_TYPES = [
        'void' => true,
        'array' => true,
        'false' => true,
        'bool' => true,
        'callable' => true,
        'float' => true,
        'int' => true,
        'iterable' => true,
        'object' => true,
        'string' => true,
        'self' => true,
        'parent' => true,
        'mixed' => true,
        'static' => true,
        'null' => true,
        'true' => true,
        'never' => true,
    ];

    private const MAGIC_METHODS = [
        '__isset' => 'bool',
        '__sleep' => 'array',
        '__toString' => 'string',
        '__debugInfo' => 'array',
        '__serialize' => 'array',
        '__set' => 'void',
        '__unset' => 'void',
        '__unserialize' => 'void',
        '__wakeup' => 'void',
    ];

    /**
     * @var callable
     */
    private $classLoader;
    private bool $isFinder;
    private array $loaded = [];
    private array $patchTypes = [];

    private static int $caseCheck;
    private static array $checkedClasses = [];
    private static array $final = [];
    private static array $finalMethods = [];
    private static array $finalProperties = [];
    private static array $finalConstants = [];
    private static array $deprecated = [];
    private static array $internal = [];
    private static array $internalMethods = [];
    private static array $annotatedParameters = [];
    private static array $darwinCache = ['/' => ['/', []]];
    /**
     * @var array<string, list<array{0: string, 1: bool, 2: string, 3: string, 4: string|null}>>
     *
     * Maps an interface FQCN (or an abstract class accumulating entries from its interfaces) to the list of
     * "@method" annotations declared on it. For interfaces, the entry is populated directly by parsing the
     * annotations from the interface's docblock. For abstract classes, the information from all implemented
     * interfaces is merged together, so that the check can later be applied to the first concrete subclass.
     *
     * Each entry is a tuple of:
     *   [0] string      $interface   - FQCN of the interface that carries the "@method" annotation
     *   [1] bool        $static      - whether the method is declared static
     *   [2] string      $returnType  - return type from the annotation, or '' if absent
     *   [3] string      $name        - method name plus its parameter signature, e.g. "foo($arg, int $n)"
     *   [4] string|null $description - description text (period-normalised), or null if absent
     */
    private static array $method = [];
    private static array $returnTypes = [];
    private static array $methodTraits = [];
    private static array $fileOffsets = [];

    /**
     * @var array<string, string>|null Re-mapping configuration for vendor comparison. Maps namespace prefixes to the value to be used for comparison.
     */
    private static ?array $namespaceRemappings = null;

    /**
     * @var array<string, true|string> Caches vendor comparison strings. Maps FQCNs to vendor prefixes; true = root namespace (matches every vendor).
     */
    private static array $vendorPrefixCache = [];

    public function __construct(callable $classLoader)
    {}

    public function getClassLoader(): callable
    {}

    /**
     * Wraps all autoloaders.
     *
     * @param array<string, string>|null $deprecationsNamespacesMapping Overrides the vendor-boundary detection used to decide whether deprecation notices
     *                                                                  are emitted. Each key is a fully-qualified class name or namespace prefix of the
     *                                                                  class being loaded; the corresponding value is the vendor string it will be
     *                                                                  compared against instead of its natural first namespace segment. Classes resolving
     *                                                                  to the same value (whether through this mapping or through a natural first-segment
     *                                                                  collision with such a value) are treated as belonging to the same vendor and will
     *                                                                  not emit deprecation or @internal notices against each other; pick a token unlikely
     *                                                                  to clash with any real vendor name to avoid silent merges.
     *                                                                  Pass null (default) to use the first namespace segment as vendor name.
     */
    public static function enable(/* ?array $deprecationsNamespacesMapping = null */): void
    { func_get_args(); }

    /**
     * Disables the wrapping.
     */
    public static function disable(): void
    {}

    public static function checkClasses(): bool
    {}

    public function findFile(string $class): ?string
    {}

    /**
     * Loads the given class or interface.
     *
     * @throws \RuntimeException
     */
    public function loadClass(string $class): void
    {}

    private function checkClass(string $class, ?string $file = null): void
    {}

    public function checkAnnotations(\ReflectionClass $refl, string $class): array
    {}

    public function checkCase(\ReflectionClass $refl, string $file, string $class): ?array
    {}

    /**
     * `realpath` on MacOSX doesn't normalize the case of characters.
     */
    private function darwinRealpath(string $real): string
    {}

    /**
     * `class_implements` includes interfaces from the parents so we have to manually exclude them.
     *
     * @return string[]
     */
    private function getOwnInterfaces(string $class, ?string $parent): array
    {}

    /**
     * @param string $class The class being loaded and inspected for deprecation violations
     * @param string $use   The parent class, interface, or trait it extends, implements, or uses
     */
    private function areFromTheSameVendor(string $class, string $use): bool
    {}

    /**
     * Returns the vendor string for a class, computing and caching it if necessary. Takes
     * remapping into account, see {@see enable()}.
     *
     * @return true|string the vendor prefix to consider; true when the class is in the root namespace
     *                     (matches every vendor)
     */
    private function getVendorEntry(string $class): bool|string
    {}

    private function setReturnType(string $types, string $class, string $method, string $filename, ?string $parent, ?\ReflectionType $returnType = null, array $templates = []): void
    {}

    private function normalizeType(string $type, string $class, ?string $parent, ?\ReflectionType $returnType): string
    {}

    /**
     * Utility method to add #[ReturnTypeWillChange] where php triggers deprecations.
     */
    private function patchReturnTypeWillChange(\ReflectionMethod $method): void
    {}

    /**
     * Utility method to add @return annotations to the Symfony code-base where it triggers self-deprecations.
     */
    private function patchMethod(\ReflectionMethod $method, string $returnType, string $declaringFile, string $normalizedType): void
    {}

    private static function getUseStatements(string $file): array
    {}

    private function fixReturnStatements(\ReflectionMethod $method, string $returnType): void
    {}

    /**
     * @param \ReflectionClass|\ReflectionMethod|\ReflectionProperty $reflector
     */
    private function parsePhpDoc(\Reflector $reflector): array
    {}
}
