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
use Iterator;
use ReturnTypeWillChange;
use function count;
use function get_class;
use function sprintf;

/** @template-implements Iterator<int,DOMElement> */
abstract class ElementCollection implements Iterator {
    /** @var DOMElement[] */
    private $nodes = [];

    /** @var int */
    private $position;

    public function __construct(DOMNodeList $nodeList) {}

    #[ReturnTypeWillChange]
    abstract public function current();

    public function next(): void {}

    public function key(): int {}

    public function valid(): bool {}

    public function rewind(): void {}

    protected function getCurrentElement(): DOMElement {}

    private function importNodes(DOMNodeList $nodeList): void {}
}
