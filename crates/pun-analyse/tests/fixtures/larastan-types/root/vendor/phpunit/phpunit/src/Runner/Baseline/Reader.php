<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner\Baseline;

use const DIRECTORY_SEPARATOR;
use function assert;
use function dirname;
use function is_file;
use function realpath;
use function sprintf;
use function str_replace;
use function trim;
use DOMElement;
use DOMXPath;
use PHPUnit\Util\Xml\Loader as XmlLoader;
use PHPUnit\Util\Xml\XmlException;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class Reader
{
    /**
     * @param non-empty-string $baselineFile
     *
     * @throws CannotLoadBaselineException
     */
    public function read(string $baselineFile): Baseline
    {}
}
