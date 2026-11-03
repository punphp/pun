<?php

namespace Orchestra\Sidekick\Filesystem;

use ReflectionClass;

if (! \function_exists('Orchestra\Sidekick\Filesystem\filename_from_classname')) {
    /**
     * Resolve filename from classname.
     *
     * @api
     *
     * @param  class-string  $className
     */
    function filename_from_classname(string $className): string|false
    {}
}

if (! \function_exists('Orchestra\Sidekick\Filesystem\is_symlink')) {
    /**
     * Determine if the path is a symlink for both Unix and Windows environments.
     *
     * @api
     */
    function is_symlink(string $path): bool
    {}
}

if (! \function_exists('Orchestra\Sidekick\Filesystem\join_paths')) {
    /**
     * Join the given paths together.
     *
     * @api
     */
    function join_paths(?string $basePath, string ...$paths): string
    {}
}
