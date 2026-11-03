<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI;

use PHPUnit\TestRunner\TestResult\TestResult;
use PHPUnit\TextUI\Configuration\Configuration;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class ShellExitCodeCalculator
{
    private const int SUCCESS_EXIT   = 0;
    private const int FAILURE_EXIT   = 1;
    private const int EXCEPTION_EXIT = 2;
    private const int TIMEOUT_EXIT   = 124;

    public function calculate(Configuration $configuration, TestResult $result): int
    {}
}
