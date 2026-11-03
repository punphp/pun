<?php

namespace Orchestra\Testbench\PHPUnit;

abstract class Assert extends \Illuminate\Testing\Assert
{
    /**
     * Mark the test as skipped when condition is not equivalent to true.
     *
     * @param  bool  $condition
     * @param  string  $message
     * @return void
     *
     * @codeCoverageIgnore
     */
    public static function markTestSkippedUnless(bool $condition, string $message = ''): void
    {}

    /**
     * Mark the test as skipped when condition is equivalent to true.
     *
     * @param  bool  $condition
     * @param  string  $message
     * @return void
     *
     * @codeCoverageIgnore
     */
    public static function markTestSkippedWhen(bool $condition, string $message = ''): void
    {}
}
