<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\Command;

use const PHP_EOL;
use function assert;
use function max;
use function sprintf;
use function strlen;
use PHPUnit\Runner\PhpConfiguration\PhpConfigurationChecker;
use PHPUnit\Runner\Version;
use PHPUnit\Util\Color;
use SebastianBergmann\Environment\Console;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class CheckPhpConfigurationCommand implements Command
{
    private bool $colorize;

    public function __construct()
    {}

    public function execute(): Result
    {}

    /**
     * @return non-empty-string
     */
    private function ok(): string
    {}

    /**
     * @return non-empty-string
     */
    private function notOk(string $actualValue): string
    {}
}
