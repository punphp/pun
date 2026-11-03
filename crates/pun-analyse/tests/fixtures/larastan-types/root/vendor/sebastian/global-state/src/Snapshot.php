<?php declare(strict_types=1);
/*
 * This file is part of sebastian/global-state.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\GlobalState;

use function array_keys;
use function array_merge;
use function array_reverse;
use function assert;
use function get_declared_classes;
use function get_declared_interfaces;
use function get_declared_traits;
use function get_defined_constants;
use function get_defined_functions;
use function get_included_files;
use function in_array;
use function ini_get_all;
use function is_array;
use function is_object;
use function is_resource;
use function is_scalar;
use function serialize;
use function unserialize;
use ReflectionClass;
use SebastianBergmann\ObjectReflector\ObjectReflector;
use SebastianBergmann\RecursionContext\Context;
use Throwable;

/**
 * A snapshot of global state.
 */
final class Snapshot
{
    private ExcludeList $excludeList;

    /**
     * @var array<string, mixed>
     */
    private array $globalVariables = [];

    /**
     * @var list<string>
     */
    private array $superGlobalArrays = [];

    /**
     * @var array<string, array<string, mixed>>
     */
    private array $superGlobalVariables = [];

    /**
     * @var array<string, array<string, mixed>>
     */
    private array $staticProperties = [];

    /**
     * @var array<non-empty-string, array{global_value: string, local_value: string, access: int}>
     */
    private array $iniSettings = [];

    /**
     * @var list<string>
     */
    private array $includedFiles = [];

    /**
     * @var array<string, mixed>
     */
    private array $constants = [];

    /**
     * @var list<callable-string>
     */
    private array $functions = [];

    /**
     * @var list<class-string>
     */
    private array $interfaces = [];

    /**
     * @var list<class-string>
     */
    private array $classes = [];

    /**
     * @var list<class-string>
     */
    private array $traits = [];

    public function __construct(?ExcludeList $excludeList = null, bool $includeGlobalVariables = true, bool $includeStaticProperties = true, bool $includeConstants = true, bool $includeFunctions = true, bool $includeClasses = true, bool $includeInterfaces = true, bool $includeTraits = true, bool $includeIniSettings = true, bool $includeIncludedFiles = true)
    {}

    public function excludeList(): ExcludeList
    {}

    /**
     * @return array<string, mixed>
     */
    public function globalVariables(): array
    {}

    /**
     * @return array<string, array<string, mixed>>
     */
    public function superGlobalVariables(): array
    {}

    /**
     * @return list<string>
     */
    public function superGlobalArrays(): array
    {}

    /**
     * @return array<string, array<string, mixed>>
     */
    public function staticProperties(): array
    {}

    /**
     * @return array<non-empty-string, array{global_value: string, local_value: string, access: int}>
     */
    public function iniSettings(): array
    {}

    /**
     * @return list<string>
     */
    public function includedFiles(): array
    {}

    /**
     * @return array<string, mixed>
     */
    public function constants(): array
    {}

    /**
     * @return list<callable-string>
     */
    public function functions(): array
    {}

    /**
     * @return list<class-string>
     */
    public function interfaces(): array
    {}

    /**
     * @return list<class-string>
     */
    public function classes(): array
    {}

    /**
     * @return list<class-string>
     */
    public function traits(): array
    {}

    private function snapshotConstants(): void
    {}

    private function snapshotFunctions(): void
    {}

    private function snapshotClasses(): void
    {}

    private function snapshotInterfaces(): void
    {}

    private function snapshotGlobals(): void
    {}

    private function snapshotSuperGlobalArray(string $superGlobalArray): void
    {}

    private function snapshotStaticProperties(): void
    {}

    private function setupSuperGlobalArrays(): void
    {}

    private function copyWithSerialize(mixed $variable): mixed
    {}

    private function canBeSerialized(mixed $variable): bool
    {}

    /**
     * @return array<mixed>
     */
    private function enumerateObjectsAndResources(mixed $variable, Context $processed = new Context): array
    {}
}
