<?php

namespace Illuminate\Testing;

use ArrayAccess;
use Illuminate\Testing\Constraints\ArraySubset;
use Illuminate\Testing\Exceptions\InvalidArgumentException;
use PHPUnit\Framework\Assert as PHPUnit;

/**
 * @internal This class is not meant to be used or overwritten outside the framework itself.
 */
abstract class Assert extends PHPUnit
{
    /**
     * Asserts that an array has a specified subset.
     *
     * @param  \ArrayAccess|array  $subset
     * @param  \ArrayAccess|array  $array
     * @param  bool  $checkForIdentity
     * @param  string  $msg
     * @return void
     */
    public static function assertArraySubset($subset, $array, bool $checkForIdentity = false, string $msg = ''): void
    {}
}
