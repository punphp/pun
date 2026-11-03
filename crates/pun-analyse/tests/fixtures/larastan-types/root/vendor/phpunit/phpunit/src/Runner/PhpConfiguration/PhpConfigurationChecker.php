<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner\PhpConfiguration;

use const E_ALL;
use function assert;
use function extension_loaded;
use function in_array;
use function ini_get;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class PhpConfigurationChecker
{
    /**
     * @return non-empty-list<PhpConfigurationCheckResult>
     */
    public function check(): array
    {}

    /**
     * @return non-empty-array<non-empty-string, array{expectedValues: non-empty-list<non-empty-string>, valueForConfiguration: non-empty-string, requiredExtensions: list<non-empty-string>}>
     */
    private function settings(): array
    {}
}
