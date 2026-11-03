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

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
abstract class Target
{
    /**
     * @param non-empty-string $namespace
     */
    public static function forNamespace(string $namespace): Namespace_
    {}

    /**
     * @param class-string $className
     */
    public static function forClass(string $className): Class_
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     */
    public static function forMethod(string $className, string $methodName): Method
    {}

    /**
     * @param class-string $interfaceName
     */
    public static function forClassesThatImplementInterface(string $interfaceName): ClassesThatImplementInterface
    {}

    /**
     * @param class-string $className
     */
    public static function forClassesThatExtendClass(string $className): ClassesThatExtendClass
    {}

    /**
     * @param non-empty-string $functionName
     */
    public static function forFunction(string $functionName): Function_
    {}

    /**
     * @param trait-string $traitName
     */
    public static function forTrait(string $traitName): Trait_
    {}

    /**
     * @param non-empty-string $path
     */
    public static function forFile(string $path): File
    {}

    /**
     * @param non-empty-string $directory
     */
    public static function forDirectory(string $directory): Directory
    {}

    /**
     * @param non-empty-string $directory
     */
    public static function forDirectoryRecursively(string $directory): DirectoryRecursively
    {}

    public function isNamespace(): bool
    {}

    public function isClass(): bool
    {}

    public function isMethod(): bool
    {}

    public function isClassesThatImplementInterface(): bool
    {}

    public function isClassesThatExtendClass(): bool
    {}

    public function isFunction(): bool
    {}

    public function isTrait(): bool
    {}

    public function isFile(): bool
    {}

    public function isDirectory(): bool
    {}

    public function isDirectoryRecursively(): bool
    {}

    /**
     * @return 'classes'|'classesThatExtendClass'|'classesThatImplementInterface'|'directories'|'directoriesRecursively'|'files'|'functions'|'methods'|'namespaces'|'traits'
     */
    abstract public function key(): string;

    /**
     * @return non-empty-string
     */
    abstract public function target(): string;

    /**
     * @return non-empty-string
     */
    abstract public function description(): string;
}
