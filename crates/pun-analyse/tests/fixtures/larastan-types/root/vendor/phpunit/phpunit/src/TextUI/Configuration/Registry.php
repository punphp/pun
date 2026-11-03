<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\Configuration;

use function assert;
use function file_get_contents;
use function file_put_contents;
use function serialize;
use function unserialize;
use PHPUnit\Event\Emitter;
use PHPUnit\TextUI\CliArguments\Configuration as CliConfiguration;
use PHPUnit\TextUI\CliArguments\Exception;
use PHPUnit\TextUI\XmlConfiguration\Configuration as XmlConfiguration;
use PHPUnit\Util\VersionComparisonOperator;

/**
 * CLI options and XML configuration are static within a single PHPUnit process.
 * It is therefore okay to use a Singleton registry here.
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class Registry
{
    private static ?Configuration $instance = null;

    public static function saveTo(string $path): bool
    {}

    /**
     * This method is used by the "run test(s) in separate process" templates.
     *
     * @noinspection PhpUnused
     *
     * @codeCoverageIgnore
     */
    public static function loadFrom(string $path): void
    {}

    public static function get(): Configuration
    {}

    /**
     * @throws \PHPUnit\TextUI\XmlConfiguration\Exception
     * @throws Exception
     * @throws NoCustomCssFileException
     */
    public static function init(CliConfiguration $cliConfiguration, XmlConfiguration $xmlConfiguration, Emitter $emitter): Configuration
    {}
}
