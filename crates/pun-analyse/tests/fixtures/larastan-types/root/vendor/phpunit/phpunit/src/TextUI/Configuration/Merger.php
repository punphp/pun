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

use const DIRECTORY_SEPARATOR;
use const PATH_SEPARATOR;
use function array_diff;
use function array_key_exists;
use function array_values;
use function assert;
use function dirname;
use function explode;
use function getenv;
use function is_int;
use function is_string;
use function realpath;
use function sprintf;
use function time;
use LogicException;
use PHPUnit\Event\Emitter;
use PHPUnit\Runner\TestSuiteSorter;
use PHPUnit\TextUI\CliArguments\Configuration as CliConfiguration;
use PHPUnit\TextUI\CliArguments\Exception;
use PHPUnit\TextUI\XmlConfiguration\Configuration as XmlConfiguration;
use PHPUnit\TextUI\XmlConfiguration\LoadedFromFileConfiguration;
use PHPUnit\TextUI\XmlConfiguration\SchemaDetector;
use PHPUnit\Util\Filesystem;
use SebastianBergmann\CodeCoverage\Report\Html\Colors;
use SebastianBergmann\CodeCoverage\Report\Thresholds;
use SebastianBergmann\Environment\Console;
use SebastianBergmann\Invoker\Invoker;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class Merger
{
    private Emitter $emitter;

    public function __construct(Emitter $emitter)
    {}

    /**
     * @throws \PHPUnit\TextUI\XmlConfiguration\Exception
     * @throws Exception
     * @throws NoCustomCssFileException
     */
    public function merge(CliConfiguration $cliConfiguration, XmlConfiguration $xmlConfiguration): Configuration
    {}

    /**
     * @return null|non-empty-string
     */
    private function nullableNonEmptyString(?string $value): ?string
    {}

    /**
     * @throws LogicException
     *
     * @return non-empty-string
     */
    private function requireNonEmptyString(string $value, string $context): string
    {}

    /**
     * @return non-negative-int
     */
    private function clampNonNegativeInt(int $value): int
    {}

    /**
     * @return positive-int
     */
    private function clampPositiveInt(int $value): int
    {}

    private function hasExplicitTestSelection(CliConfiguration $cliConfiguration): bool
    {}

    /**
     * @param non-empty-string $attribute
     * @param non-empty-string $enablingSetting
     * @param non-empty-string $cliOption
     */
    private function warnAboutFailOnSettingThatHasNoEffect(string $attribute, string $enablingSetting, string $cliOption): void
    {}
}
