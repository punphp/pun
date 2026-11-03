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
use const STDIN;
use function assert;
use function defined;
use function fgets;
use function file_put_contents;
use function getcwd;
use function is_file;
use function sprintf;
use function trim;
use PHPUnit\Runner\Version;
use PHPUnit\TextUI\XmlConfiguration\Generator;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class GenerateConfigurationCommand implements Command
{
    public function execute(): Result
    {}

    private function read(): string
    {}
}
