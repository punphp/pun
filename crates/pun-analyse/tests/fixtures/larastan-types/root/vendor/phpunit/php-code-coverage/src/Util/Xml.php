<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Util;

use const PHP_EOL;
use function libxml_clear_errors;
use function libxml_get_errors;
use function libxml_use_internal_errors;
use DOMDocument;
use SebastianBergmann\CodeCoverage\XmlException;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final readonly class Xml
{
    /**
     * @throws XmlException
     *
     * @see https://bugs.php.net/bug.php?id=79191
     */
    /**
     * @return non-empty-string
     */
    public static function asString(DOMDocument $document): string
    {}
}
