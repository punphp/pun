<?php

namespace Orchestra\Testbench\Concerns;

use Pest\Contracts\HasPrintableTestCaseName;

trait InteractsWithPest
{
    use InteractsWithPHPUnit;
    use InteractsWithTestCase;

    /**
     * Determine if the trait is used within testing using Pest.
     *
     * @api
     *
     * @return bool
     */
    public function isRunningTestCaseUsingPest(): bool
    {}

    /**
     * Determine if the object implements Pest test runner.
     *
     * @return bool
     */
    protected static function isRunningViaPestPrinter(object|string $object): bool
    {}
}
