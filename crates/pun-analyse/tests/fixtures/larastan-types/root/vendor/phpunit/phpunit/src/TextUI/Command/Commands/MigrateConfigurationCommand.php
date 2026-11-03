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
use function copy;
use function file_put_contents;
use function sprintf;
use PHPUnit\TextUI\XmlConfiguration\Migrator;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class MigrateConfigurationCommand implements Command
{
    private string $filename;

    public function __construct(string $filename)
    {}

    public function execute(): Result
    {}
}
