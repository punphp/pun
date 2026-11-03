<?php

declare(strict_types=1);

/**
 * Mockery (https://docs.mockery.io/en/stable/)
 *
 * @copyright https://github.com/mockery/mockery/blob/HEAD/COPYRIGHT.md
 * @license   https://github.com/mockery/mockery/blob/HEAD/LICENSE BSD 3-Clause License
 * @see       https://github.com/mockery/mockery for the canonical source repository
 */

use Mockery\Matcher\AndAnyOtherArgs;
use Mockery\Matcher\AnyArgs;
use Mockery\MockInterface;

if (! \function_exists('andAnyOtherArgs')) {
    function andAnyOtherArgs(): AndAnyOtherArgs
    {}
}

if (! \function_exists('andAnyOthers')) {
    function andAnyOthers(): AndAnyOtherArgs
    {}
}

if (! \function_exists('anyArgs')) {
    function anyArgs(): AnyArgs
    {}
}

if (! \function_exists('get_debug_type')) {
    /**
     * Copied from symfony/polyfill (https://github.com/symfony/polyfill/blob/1.x/src/Php80/Php80.php)
     *
     * @copyright Fabien Potencier
     * @license https://github.com/symfony/polyfill/blob/1.x/src/Php80/LICENSE
     *
     * @param mixed $value
     */
    function get_debug_type($value): string
    {}
}

if (! \function_exists('mock')) {
    /**
     * @param  mixed         ...$args
     * @return MockInterface
     *
     * @throws Throwable
     */
    function mock(...$args)
    {}
}

if (! \function_exists('namedMock')) {
    /**
     * @param  mixed         ...$args
     * @return MockInterface
     *
     * @throws Throwable
     */
    function namedMock(...$args)
    {}
}

if (! \function_exists('spy')) {
    /**
     * @param  mixed         ...$args
     * @return MockInterface
     *
     * @throws Throwable
     */
    function spy(...$args)
    {}
}

if (! \function_exists('str_contains')) {
    /**
     * Copied from symfony/polyfill (https://github.com/symfony/polyfill/blob/1.x/src/Php80/Php80.php)
     *
     * @copyright Fabien Potencier
     * @license https://github.com/symfony/polyfill/blob/1.x/src/Php80/LICENSE
     */
    function str_contains(string $haystack, string $needle): bool
    {}
}

if (! \function_exists('str_ends_with')) {
    /**
     * Modified copy from symfony/polyfill (https://github.com/symfony/polyfill/blob/1.x/src/Php80/Php80.php)
     *
     * @copyright Fabien Potencier
     * @license https://github.com/symfony/polyfill/blob/1.x/src/Php80/LICENSE
     */
    function str_ends_with(string $haystack, string $needle): bool
    {}
}

if (! \function_exists('str_starts_with')) {
    /**
     * Copied from symfony/polyfill (https://github.com/symfony/polyfill/blob/1.x/src/Php80/Php80.php)
     *
     * @copyright Fabien Potencier
     * @license https://github.com/symfony/polyfill/blob/1.x/src/Php80/LICENSE
     */
    function str_starts_with(string $haystack, string $needle): bool
    {}
}

if (! \function_exists('array_is_list')) {
    /**
     * Modified copy from https://www.php.net/manual/en/function.array-is-list.php#127044
     *
     * @license https://www.php.net/manual/en/cc.license.php
     *
     * @param array<mixed> $array
     */
    function array_is_list(array $array): bool
    {}
}
