<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Util;

use function array_unshift;
use PHPUnit\TextUI\Configuration\Registry as ConfigurationRegistry;
use SebastianBergmann\Comparator\Factory as ComparatorFactory;
use SebastianBergmann\Exporter\Exporter as OriginalExporter;
use SebastianBergmann\Exporter\ObjectExporter;
use SebastianBergmann\Exporter\ObjectExporterChain;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final class Exporter
{
    /**
     * @var list<ObjectExporter>
     */
    private static array $objectExporters                         = [];
    private static ?OriginalExporter $exporter                    = null;
    private static ?OriginalExporter $exporterOfComparatorFactory = null;

    public static function registerObjectExporter(ObjectExporter $objectExporter): void
    {}

    public static function unregisterObjectExporter(ObjectExporter $objectExporter): void
    {}

    public static function export(mixed $value): string
    {}

    /**
     * @param array<mixed> $data
     */
    public static function shortenedRecursiveExport(array $data): string
    {}

    public static function shortenedExport(mixed $value): string
    {}

    private static function updateComparatorFactory(): void
    {}

    private static function exporter(): OriginalExporter
    {}
}
