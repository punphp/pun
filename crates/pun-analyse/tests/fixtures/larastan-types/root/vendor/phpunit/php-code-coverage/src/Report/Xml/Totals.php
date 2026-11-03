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

use SebastianBergmann\CodeCoverage\Util\Percentage;
use XMLWriter;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final readonly class Totals
{
    private XMLWriter $xmlWriter;

    public function __construct(XMLWriter $xmlWriter)
    {}

    public function setNumLines(int $loc, int $cloc, int $ncloc, int $executable, int $executed): void
    {}

    public function setNumClasses(int $count, int $tested): void
    {}

    public function setNumTraits(int $count, int $tested): void
    {}

    public function setNumMethods(int $count, int $tested): void
    {}

    public function setNumFunctions(int $count, int $tested): void
    {}

    public function getWriter(): XMLWriter
    {}
}
