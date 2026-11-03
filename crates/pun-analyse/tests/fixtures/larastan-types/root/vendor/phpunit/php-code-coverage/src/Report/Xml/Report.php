<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Report\Xml;

use function basename;
use function dirname;
use DOMDocument;
use SebastianBergmann\CodeCoverage\Util\EnsuresUtf8;
use XMLWriter;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final class Report extends File
{
    use EnsuresUtf8;
    private readonly string $name;
    private readonly string $sha1;

    public function __construct(XMLWriter $xmlWriter, string $name, string $sha1)
    {}

    public function finalize(): void
    {}

    public function functionObject(
        string $name,
        string $signature,
        string $start,
        ?string $end,
        string $executable,
        string $executed,
        string $coverage,
        string $crap
    ): void {}

    public function classObject(
        string $name,
        string $namespace,
        int $start,
        int $executable,
        int $executed,
        float $crap
    ): Unit {}

    public function traitObject(
        string $name,
        string $namespace,
        int $start,
        int $executable,
        int $executed,
        float $crap
    ): Unit {}

    public function source(): Source
    {}
}
