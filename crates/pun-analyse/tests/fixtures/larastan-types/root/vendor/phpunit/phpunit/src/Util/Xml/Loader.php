<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Util\Xml;

use const LIBXML_NONET;
use function assert;
use function error_reporting;
use function file_get_contents;
use function libxml_get_errors;
use function libxml_use_internal_errors;
use function sprintf;
use function trim;
use DOMDocument;
use DOMNode;
use DOMXPath;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class Loader
{
    /**
     * @throws XmlException
     */
    public function loadFile(string $filename, bool $ignoreComments = false): DOMDocument
    {}

    /**
     * @throws XmlException
     */
    public function load(string $actual, bool $ignoreComments = false): DOMDocument
    {}

    private function removeComments(DOMDocument $document): void
    {}
}
