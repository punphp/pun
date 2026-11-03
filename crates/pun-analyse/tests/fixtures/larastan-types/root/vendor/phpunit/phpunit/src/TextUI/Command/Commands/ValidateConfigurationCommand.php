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
use function realpath;
use function sprintf;
use PHPUnit\Runner\Version;
use PHPUnit\TextUI\XmlConfiguration\CannotFindSchemaException;
use PHPUnit\TextUI\XmlConfiguration\SchemaFinder;
use PHPUnit\TextUI\XmlConfiguration\Validator;
use PHPUnit\Util\Xml\Loader as XmlLoader;
use PHPUnit\Util\Xml\XmlException;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class ValidateConfigurationCommand implements Command
{
    private string $filename;

    public function __construct(string $filename)
    {}

    public function execute(): Result
    {}
}
