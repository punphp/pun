<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\ErrorHandler\ErrorEnhancer;

use Composer\Autoload\ClassLoader;
use Symfony\Component\ErrorHandler\DebugClassLoader;
use Symfony\Component\ErrorHandler\Error\ClassNotFoundError;
use Symfony\Component\ErrorHandler\Error\FatalError;

/**
 * @author Fabien Potencier <fabien@symfony.com>
 */
class ClassNotFoundErrorEnhancer implements ErrorEnhancerInterface
{
    public function enhance(\Throwable $error): ?\Throwable
    {}

    /**
     * Tries to guess the full namespace for a given class name.
     *
     * By default, it looks for PSR-0 and PSR-4 classes registered via a Symfony or a Composer
     * autoloader (that should cover all common cases).
     *
     * @param string $class A class name (without its namespace)
     *
     * Returns an array of possible fully qualified class names
     */
    private function getClassCandidates(string $class): array
    {}

    private function findClassInPath(string $path, string $class, string $prefix): array
    {}

    private function convertFileToClass(string $path, string $file, string $prefix): ?string
    {}

    private function classExists(string $class): bool
    {}
}
