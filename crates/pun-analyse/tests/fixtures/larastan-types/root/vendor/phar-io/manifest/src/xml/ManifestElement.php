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

use DOMElement;
use DOMNodeList;
use function sprintf;

class ManifestElement {
    public const XMLNS = 'https://phar.io/xml/manifest/1.0';

    /** @var DOMElement */
    private $element;

    public function __construct(DOMElement $element) {}

    protected function getAttributeValue(string $name): string {}

    protected function hasAttribute(string $name): bool {}

    protected function getChildByName(string $elementName): DOMElement {}

    protected function getChildrenByName(string $elementName): DOMNodeList {}

    protected function hasChild(string $elementName): bool {}
}
