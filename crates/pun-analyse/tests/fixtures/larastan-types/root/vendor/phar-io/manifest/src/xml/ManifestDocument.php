<?php declare(strict_types = 1);
/*
 * This file is part of PharIo\Manifest.
 *
 * Copyright (c) Arne Blankerts <arne@blankerts.de>, Sebastian Heuer <sebastian@phpeople.de>, Sebastian Bergmann <sebastian@phpunit.de> and contributors
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 *
 */
namespace PharIo\Manifest;

use DOMDocument;
use DOMElement;
use Throwable;
use function count;
use function file_get_contents;
use function is_file;
use function libxml_clear_errors;
use function libxml_get_errors;
use function libxml_use_internal_errors;
use function sprintf;

class ManifestDocument {
    public const XMLNS = 'https://phar.io/xml/manifest/1.0';

    /** @var DOMDocument */
    private $dom;

    public static function fromFile(string $filename): ManifestDocument {}

    public static function fromString(string $xmlString): ManifestDocument {}

    private function __construct(DOMDocument $dom) {}

    public function getContainsElement(): ContainsElement {}

    public function getCopyrightElement(): CopyrightElement {}

    public function getRequiresElement(): RequiresElement {}

    public function hasBundlesElement(): bool {}

    public function getBundlesElement(): BundlesElement {}

    private function ensureCorrectDocumentType(DOMDocument $dom): void {}

    private function fetchElementByName(string $elementName): DOMElement {}
}
